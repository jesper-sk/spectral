use std::{io::Write, time::Duration};

use cpal::{
    Device, Host, SupportedStreamConfigRange,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use itertools::Itertools;

mod analysis;
mod stream;
mod util;

fn main() {
    let host = choose_host();
    let device = choose_input_device(&host);
    let config_range = choose_config(&device);
    let config = config_range.with_max_sample_rate();

    let (stream, mut update) = stream::init(&device, &config);

    stream.play().expect("Couldn't play the stream");
    println!("Recording, press CTRL + C to stop");
    loop {
        if let Some(analysis) = update() {
            let total = (analysis.volume_rms * 100.0) as usize;
            print!("\r|{}{}|", "|".repeat(total), " ".repeat(100 - total));
        }
        std::thread::sleep(Duration::from_millis(10));
        std::io::stdout().flush().unwrap();
    }
}

fn choose_host() -> Host {
    loop {
        let hosts = cpal::available_hosts();
        println!("Available hosts: \n {}", util::print_list(&hosts));
        let chosen_index: usize = util::input_validated(|i| i < hosts.len());

        if let Ok(host) = cpal::host_from_id(hosts[chosen_index]) {
            return host;
        }
        println!("Host is unavailable");
    }
}

fn choose_input_device(host: &Host) -> Device {
    let mut devices = host
        .input_devices()
        .expect("Can't fetch devices")
        .collect_vec();

    let names = devices
        .iter()
        .map(|d| d.description().unwrap())
        .collect_vec();

    println!("Available devices: \n {}", util::print_list(&names));
    let chosen_index: usize = util::input_validated(|i| i < devices.len());
    return devices.remove(chosen_index);
}

fn choose_config(device: &Device) -> SupportedStreamConfigRange {
    let mut configs = device
        .supported_input_configs()
        .expect("Can't find any supported input configs")
        .collect_vec();

    println!("Available configs: \n {}", util::print_list(&configs));
    let chosen_index: usize = util::input_validated(|i| i < configs.len());
    return configs.remove(chosen_index);
}
