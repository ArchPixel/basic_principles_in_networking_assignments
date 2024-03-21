use mac_address2::mac_address_by_name;
use networkmanager::devices::{Device,Wireless};
use networkmanager::{Error,NetworkManager};
use std::thread;
use std::time::Duration;
use wifi_rs::{prelude::*, WiFi};
fn scan() -> Result<(), Error>  {
    //scan available wifi and print out wifi channel and SSID
    let nm = NetworkManager::new()?;
    for dev in nm.get_devices()? {
        match dev {
            Device::WiFi(x) => {
                x.request_scan(std::collections::HashMap::new())?;
                for ap in x.get_all_access_points()? {
                    println!("----------------------------");
                    println!("BSSID: {:?}",ap.hw_address()?);
                    println!("SSID: {:?}", ap.ssid()?);
                    let freq = ap.frequency()?;
                    let channel = (freq - 2412)/5 + 1;
                    println!("Frequency: {:?}", freq);
                    println!("Channel: {}", channel);
                }
            }
            _ => {}
        }
    }
    println!("----------------------------");
    Ok(())
}
fn connect_wifi() -> Result<(),WifiConnectionError> {
    let config = Some(Config {
        interface: Some("wlp7s0"),
    });
    let mut wifi = WiFi::new(config);
    match wifi.connect("iPhone_ㄒㄉ", "j9qv0ob46fot3") {
        Ok(result) => println!(
            "{}",
            if result == true {
                "Connection Successful."
            } else {
                "Invalid password."
            }
        ),
        Err(err) => println!("The following error occurred: {:?}", err),
    }
    Ok(())
}
fn main() {
    //print out MAC address
    match mac_address_by_name("wlp7s0") {
        Ok(Some(ma)) => {
            println!("WiFi MAC address of this computer = {}", ma);
            println!("                        ");
        }
        Ok(None) => println!("No MAC address found."),
        Err(e) => println!("{:?}", e),
    }
    //scan wifi every 10 seconds for 3 times
    for i in 1..=3 {
        println!("Scanning the wifi {} time",i);
        let scan_results =scan();
        if scan_results.is_ok() {
            println!("WiFi Scan succeeded!");
        } else {
            println!("Error: {:?}", scan_results);
        }
        println!("                        ");
        thread::sleep(Duration::from_secs(10));
    }
    //Connect to a wifi provided with SSID & password
    let connect_result = connect_wifi();
    if connect_result.is_err() {
        println!("Error: {:?}",connect_result);
    }
}