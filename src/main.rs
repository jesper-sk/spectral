use std::{fmt::Debug, fmt::Display, time::Duration};

use cpal::{
    Device, Host, InputCallbackInfo, SizedSample, Stream, StreamError, SupportedStreamConfigRange,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use itertools::Itertools;

mod util;

#[derive(Debug)]
struct RecordingState<T: SizedSample> {
    volume: T,
}

impl<T: SizedSample> Default for RecordingState<T> {
    fn default() -> Self {
        Self {
            volume: T::EQUILIBRIUM,
        }
    }
}

impl<T: SizedSample> RecordingState<T> {
    pub fn update(&mut self, data: &[T]) {
        let max = data
            .iter()
            .reduce(|x, y| if x <= y { y } else { x })
            .expect("whops");

        self.volume = *max;
    }
}

fn main() {
    let host = choose_host();
    let device = choose_input_device(&host);
    let config = choose_config(&device);
    let stream = make_stream(&device, &config);

    let state = RecordingState::<f32>::default();

    stream.play().unwrap();
    println!("Press enter to stop");
    util::read_line();
    stream.pause().unwrap();
}

fn make_stream<'a>(
    device: &'a Device,
    config: &'a SupportedStreamConfigRange,
) -> (&'a RecordingState<f32>, &'a Stream) {
    let mut state = RecordingState::<f32>::default();
    let stream = device
        .build_input_stream(
            &'a config.with_max_sample_rate().into(),
            build_handler::<f32>(&'a mut state),
            error_handler,
            Some(Duration::from_secs(5)),
        )
        .expect("No stream!");

    return (&state, &stream);
}

fn error_handler(err: StreamError) {
    eprintln!("An error occurred in recording: {}", err);
}

fn build_handler<T: SizedSample>(
    state: &mut RecordingState<T>,
) -> impl FnMut(&[T], &InputCallbackInfo) {
    |data: &[T], _info: &InputCallbackInfo| {
        state.update(data);
    }
}

fn input_handler<T: Display + PartialOrd + Debug>(data: &[T], info: &InputCallbackInfo) {
    let min = data.iter().reduce(|x, y| if x <= y { x } else { y });
    let max = data.iter().reduce(|x, y| if x >= y { x } else { y });
    println!("{:?} – {:?}", min, max);
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
