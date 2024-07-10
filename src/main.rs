use esp_idf_hal::{
    delay::FreeRtos, gpio::{IOPin, PinDriver, Pull}, modem, peripherals::Peripherals, sys
};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition, wifi::{ClientConfiguration, Configuration}};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    // Important, do not remove, prevents device bricking
    FreeRtos::delay_ms(5000);

    // Grab peripherals
    let peripherals = Peripherals::take().unwrap();

    let mut btn_pin = PinDriver::input(peripherals.pins.gpio6.downgrade()).unwrap();
    btn_pin.set_pull(Pull::Down).unwrap();

    loop {
        // Loop through gpio pins and detect button presses
        println!("Button pressed: {}", btn_pin.is_high());
        FreeRtos::delay_ms(500);

        // // Scan for wifi networks, list them over info!, and then wait 10 seconds loop
        // let peripherals = Peripherals::take().unwrap();
        // let modem = peripherals.modem;
        // let sysloop = EspSystemEventLoop::take().unwrap();
        // // let nvs = EspDefaultNvsPartition::take().unwrap();
        // let mut wifi = esp_idf_svc::wifi::EspWifi::new(modem, sysloop.clone(), None).unwrap();

        // wifi.set_configuration(&Configuration::Client(ClientConfiguration::default())).unwrap();
        // wifi.start().unwrap();

        // let scan = wifi.scan().unwrap();

        // for network in scan.iter() {
        //     log::info!("{:?}", network);
        // }

        // log::info!("Completed scan waiting 5 seconds");

        // FreeRtos::delay_ms(5000);
    }
}
