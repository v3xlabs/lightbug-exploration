use std::net::Ipv4Addr;
use std::str::FromStr;

use esp_idf_hal::{delay::FreeRtos, io::Write, peripherals::Peripherals};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::ipv4::Subnet;
use esp_idf_svc::netif::NetifStack;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::EspWifi;
use esp_idf_svc::{
    http::{self, server::EspHttpServer},
    ipv4::RouterConfiguration,
    netif::{EspNetif, NetifConfiguration},
    wifi::{AccessPointConfiguration, Configuration},
};
use esp_idf_sys::EspError;

/**
 * Pin Mapping
 * gpio9 LEDs
 * spi2 - bus 2
 * gpio4 - data control (display)
 * gpio5 - display reset (display)
 * pin10 - button1
 * 8 - button2
 * 3 - button3
 * 2 - button4
 */

fn main() -> Result<(), EspError> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Booting... this is your 5 seconds to unbrick the device Kappa");

    // Important, do not remove, prevents device bricking
    FreeRtos::delay_ms(5000);

    // Grab peripherals
    let peripherals = Peripherals::take().unwrap();

    // Initialize NVS
    let nvs_partition = EspDefaultNvsPartition::take().unwrap();

    // Initialize system event loop
    let sysloop = EspSystemEventLoop::take().unwrap();

    // Set up Wi-Fi access point
    let mut netif = EspNetif::new_with_conf(&NetifConfiguration {
        custom_mac: None,
        description: heapless::String::from_str("hello").unwrap(),
        route_priority: 1,
        key: heapless::String::new(),
        ip_configuration: esp_idf_svc::ipv4::Configuration::Router(RouterConfiguration {
            subnet: Subnet {
                gateway: Ipv4Addr::new(10, 0, 0, 1),
                mask: Ipv4Addr::new(255, 255, 255, 0).try_into().unwrap(),
            },
            ..Default::default()
        }),
        stack: NetifStack::Ap,
    })
    .unwrap();

    let mut wifi = EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs_partition))?;

    wifi.swap_netif_ap(netif).unwrap();

    let ap_config = AccessPointConfiguration {
        ssid: heapless::String::from_str("V3X-Lightbug").unwrap(),
        password: heapless::String::from_str("thewifipassword").unwrap(),
        channel: 1,
        auth_method: esp_idf_svc::wifi::AuthMethod::WPA2Personal,
        ..Default::default()
    };

    wifi.set_configuration(&Configuration::AccessPoint(ap_config))?;
    wifi.start()?;

    let ip = wifi.ap_netif().get_ip_info().unwrap();
    log::info!("WiFi AP IP address: {}", ip.ip);

    // Set up HTTP server
    let configuration = http::server::Configuration {
        ..Default::default()
    };
    let mut http = EspHttpServer::new(&configuration).unwrap();

    http.fn_handler("/", http::Method::Get, |request| {
        log::info!("Request incomming from {}", request.uri());
        request
            .into_ok_response()?
            .write_all(b"<html><body>Hello world!</body></html>")
            .map(|_| ())
    })
    .unwrap();

    log::info!("WiFi AP and HTTP server are running!");

    // Keep the main function alive
    let mut i = 0;
    loop {
        FreeRtos::delay_ms(1000);
        i += 1;

        if i > 10 {
            log::info!("Reached 10 seconds, http server");
            i = 0;
        }
    }
}
