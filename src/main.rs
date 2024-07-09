use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{IOPin, PinDriver},
    peripherals::Peripherals,
};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let mut led_pin = PinDriver::output(peripherals.pins.gpio6).unwrap();

    led_pin.set_low().unwrap();

    loop {
        log::info!("Blinking LED");
        led_pin.set_high().unwrap();
        FreeRtos::delay_ms(1000);
        led_pin.set_low().unwrap();
        FreeRtos::delay_ms(1000);
    }
}
