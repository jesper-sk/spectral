use cpal::{self, traits::DeviceTrait};
use ringbuf as rb;
use ringbuf::producer as rbp;
use ringbuf::traits::{Consumer, Split};

use crate::analysis;

macro_rules! match_format {
    ($format:expr, $T:ident => $body:block) => {
        match $format {
            cpal::SampleFormat::I8 => {
                type $T = i8;
                $body
            }
            cpal::SampleFormat::I16 => {
                type $T = i16;
                $body
            }
            cpal::SampleFormat::I24 => {
                type $T = i32;
                $body
            }
            cpal::SampleFormat::I32 => {
                type $T = i32;
                $body
            }
            cpal::SampleFormat::I64 => {
                type $T = i64;
                $body
            }
            cpal::SampleFormat::U8 => {
                type $T = u8;
                $body
            }
            cpal::SampleFormat::U16 => {
                type $T = u16;
                $body
            }
            cpal::SampleFormat::U24 => {
                type $T = u32;
                $body
            }
            cpal::SampleFormat::U32 => {
                type $T = u32;
                $body
            }
            cpal::SampleFormat::U64 => {
                type $T = u64;
                $body
            }
            cpal::SampleFormat::F32 => {
                type $T = f32;
                $body
            }
            cpal::SampleFormat::F64 => {
                type $T = f64;
                $body
            }
            _ => panic!("Unexpected cpal::SampleFormat variant"),
        }
    };
}

pub fn init(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
) -> (
    cpal::Stream,
    Box<dyn FnMut() -> Option<analysis::AudioInfo>>,
) {
    return match_format!(config.sample_format(), T => {
        let rb = rb::HeapRb::<T>::new(8192);
        let (prod, mut cons) = rb.split();
        let stream = make_stream(&device, &config, prod);

        let mut analysis_buffer = Vec::with_capacity(2048);

        let updater: Box<dyn FnMut() -> Option<analysis::AudioInfo>> = Box::new(move || {
            while let Some(sample) = cons.try_pop() {
                analysis_buffer.push(sample);
            }

            if analysis_buffer.len() >= 1024 {
                let analysis = analysis::AudioInfo::new_from(&analysis_buffer);
                analysis_buffer.clear();

                return Some(analysis);
            }

            return None;
        });

        (stream, updater)
    });
}

pub fn make_stream<TSample, TProd>(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
    mut prod: TProd,
) -> cpal::Stream
where
    TSample: cpal::SizedSample + Send,
    TProd: rbp::Producer<Item = TSample> + Send + 'static,
{
    let stream = device
        .build_input_stream(
            &config.config(),
            move |data: &[TSample], _: &cpal::InputCallbackInfo| {
                let _ = prod.push_slice(data);
            },
            |err| eprintln!("Stream error: {:?}", err),
            None,
        )
        .expect("Couldn't build stream");

    return stream;
}
