#![no_std]
#![no_main]

extern crate alloc;

use alloc::sync::Arc;
use esp_idf_hal::delay::FreeRtos;
// use embassy_time::{Duration, Timer};
use smilepal_esp32_ble::{utilities::mutex::Mutex, uuid128, BLEClient, BLEDevice};
use esp_idf_sys as _;
use futures::executor::block_on;
use log::*;

#[no_mangle]
fn main() {
  esp_idf_sys::link_patches();

  esp_idf_svc::log::EspLogger::initialize_default();
  esp_idf_svc::log::EspLogger.set_target_level("SmilePal-BLErs", log::LevelFilter::Warn);

  log::set_max_level(log::LevelFilter::Debug);

  // esp_idf_svc::timer::embassy_time::driver::link(); // tbh I have no idea what this does, but it was in the original example code

  block_on(async {
		let ble_device = BLEDevice::take();
		let ble_scan = ble_device.get_scan();
		let connect_device = Arc::new(Mutex::new(None));

		let device0 = connect_device.clone();
		ble_scan
			.active_scan(true)
			.interval(100)
			.window(99)
			.on_result(move |device| {
				if device.name().contains("ESP32") {
					BLEDevice::take().get_scan().stop().unwrap();
					(*device0.lock()) = Some(device.clone());
				}
			});
		ble_scan.start(10000).await.unwrap();

		let device = &*connect_device.lock();
		if let Some(device) = device {
			info!("Advertised Device: {:?}", device);

			let mut client = BLEClient::new();
			client.connect(device.addr()).await.unwrap();

			let service = client
				.get_service(uuid128!("fafafafa-fafa-fafa-fafa-fafafafafafa"))
				.await
				.unwrap();

			let uuid = uuid128!("d4e0e0d0-1a2b-11e9-ab14-d663bd873d93");
			let characteristic = service.get_characteristic(uuid).await.unwrap();
			let value = characteristic.read_value().await.unwrap();
			::log::info!(
				"{:?} value: {}",
				uuid,
				core::str::from_utf8(&value).unwrap()
			);

			let uuid = uuid128!("a3c87500-8ed3-4bdf-8a39-a01bebede295");
			let characteristic = service.get_characteristic(uuid).await.unwrap();
			::log::info!("subscribe {:?}", uuid);
			characteristic
				.on_notify(|data| {
					::log::info!("{}", core::str::from_utf8(&data).unwrap());
				})
				.subscribe_notify(false)
				.await
				.unwrap();

			FreeRtos::delay_ms(1000 * 10);

			client.disconnect().unwrap();
		}
	});
}