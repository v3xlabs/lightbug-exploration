use config::DriverConfig;
use display_interface_spi::SPIInterface;
use embedded_graphics::pixelcolor::Rgb565;
use esp_idf_hal::delay::{Delay, Ets};
use esp_idf_hal::gpio::{
    AnyInputPin, AnyOutputPin, Gpio1, Gpio16, Gpio4, OutputPin, PinDriver, Pull,
};
use esp_idf_hal::spi::SpiDriver;
use esp_idf_hal::spi::{self, *};
use mipidsi::{models, Builder};
use smart_leds::hsv::{hsv2rgb, Hsv};

use esp_idf_hal::{delay::FreeRtos, peripherals::Peripherals};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_sys::EspError;
use esp_idf_sys::*;
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

/**
 * Pin Mapping
 * gpio9 LEDs
 * spi2 - bus 2
 * gpio4 - data control (display)
 * gpio5 - display reset (display)
 *
 * the device has 4 individually addressable leds in a row on pin 9
 */

fn main() -> Result<(), EspError> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Booting... this is your 5 seconds to unbrick the device Kappa");

    // Important, do not remove, prevents device bricking
    FreeRtos::delay_ms(5000);

    // Grab peripherals
    let peripherals = Peripherals::take().unwrap();

    // Temp button setup
    // let buttons = setup_buttons(peripherals);

    // Initialize NVS
    let nvs_partition = EspDefaultNvsPartition::take().unwrap();

    // Initialize system event loop
    let sysloop = EspSystemEventLoop::take().unwrap();

    // Either doesnt matter
    let mut delay = Delay::new_default();
    // let mut delay = Ets;

    let rst = PinDriver::input_output_od(peripherals.pins.gpio5).unwrap();
    let dc = PinDriver::input_output_od(peripherals.pins.gpio4).unwrap();
    let sdo = peripherals.pins.gpio7;

    let spi = SpiDriver::new(
        peripherals.spi2,
        peripherals.pins.gpio6, // sclk
        sdo,                    // mosi
        None::<AnyInputPin>,    // miso
        &spi::SpiDriverConfig::default(),
    )?;

    let spi = SpiDeviceDriver::new(spi, None::<AnyOutputPin>, &spi::SpiConfig::default())?;

    let di = SPIInterface::new(spi, dc);
    let mut display = Builder::new(models::ST7789, di)
        .reset_pin(rst)
        .init(&mut delay)
        .unwrap();

    display
        .set_pixel(5, 5, Rgb565::new(0xFF, 0x00, 0x00))
        .unwrap();

    let mut button1 = PinDriver::input(peripherals.pins.gpio10).unwrap();
    button1.set_pull(Pull::Up).unwrap();

    let led_pin = peripherals.pins.gpio9;
    let channel = peripherals.rmt.channel0;

    let mut ws2812 = Ws2812Esp32Rmt::new(channel, led_pin).unwrap();

    let mut hue = unsafe { esp_random() } as u8;
    loop {
        let pixels = std::iter::repeat(hsv2rgb(Hsv {
            hue,
            sat: 255,
            val: 8,
        }))
        .take(25);
        ws2812.write_nocopy(pixels).unwrap();

        // check button 1
        if button1.is_low() {
            log::info!("Button 1 pressed!");
        }

        FreeRtos::delay_ms(100);

        hue = hue.wrapping_add(10);
    }

    // // Set up Wi-Fi access point
    // let mut netif = EspNetif::new_with_conf(&NetifConfiguration {
    //     custom_mac: None,
    //     description: heapless::String::from_str("hello").unwrap(),
    //     route_priority: 1,
    //     key: heapless::String::new(),
    //     ip_configuration: esp_idf_svc::ipv4::Configuration::Router(RouterConfiguration {
    //         subnet: Subnet {
    //             gateway: Ipv4Addr::new(10, 0, 0, 1),
    //             mask: Ipv4Addr::new(255, 255, 255, 0).try_into().unwrap(),
    //         },
    //         ..Default::default()
    //     }),
    //     stack: NetifStack::Ap,
    // })
    // .unwrap();

    // let mut wifi = EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs_partition))?;

    // wifi.swap_netif_ap(netif).unwrap();

    // let ap_config = AccessPointConfiguration {
    //     ssid: heapless::String::from_str("V3X-Lightbug").unwrap(),
    //     password: heapless::String::from_str("thewifipassword").unwrap(),
    //     channel: 1,
    //     auth_method: esp_idf_svc::wifi::AuthMethod::WPA2Personal,
    //     ..Default::default()
    // };

    // wifi.set_configuration(&Configuration::AccessPoint(ap_config))?;
    // wifi.start()?;

    // let ip = wifi.ap_netif().get_ip_info().unwrap();
    // log::info!("WiFi AP IP address: {}", ip.ip);

    // // Set up HTTP server
    // let configuration = http::server::Configuration {
    //     ..Default::default()
    // };
    // let mut http = EspHttpServer::new(&configuration).unwrap();

    // http.fn_handler("/", http::Method::Get, |request| {
    //     log::info!("Request incomming from {}", request.uri());
    //     request
    //         .into_ok_response()?
    //         .write_all(b"<html><body>Hello world!</body></html>")
    //         .map(|_| ())
    // })
    // .unwrap();

    // log::info!("WiFi AP and HTTP server are running!");

    // Keep the main function alive
    // let mut i = 0;
    // loop {
    //     FreeRtos::delay_ms(1000);
    //     i += 1;

    //     if i > 10 {
    //         log::info!("Reached 10 seconds, http server");
    //         i = 0;
    //     }
    // }
}

// /**
//  * Pin Mapping
//  * gpio10 - button1
//  * gpio8 - button2
//  * gpio3 - button3
//  * gpio2 - button4
//  *
//  * button1 is the most right button
//  * button4 is the most left button
//  */
// pub fn setup_buttons(peripherals: Peripherals) {
//     let mut button1 = PinDriver::input(peripherals.pins.gpio10).unwrap();
//     button1.set_pull(esp_idf_hal::gpio::Pull::Up).unwrap();

//     let mut button2 = PinDriver::input(peripherals.pins.gpio8).unwrap();
//     button2.set_pull(esp_idf_hal::gpio::Pull::Up).unwrap();

//     let mut button3 = PinDriver::input(peripherals.pins.gpio3).unwrap();
//     button3.set_pull(esp_idf_hal::gpio::Pull::Up).unwrap();

//     let mut button4 = PinDriver::input(peripherals.pins.gpio2).unwrap();
//     button4.set_pull(esp_idf_hal::gpio::Pull::Up).unwrap();
// }
