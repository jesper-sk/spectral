use cpal;

#[derive(Debug)]
pub struct AudioInfo {
    pub volume_rms: f32,
}

impl Default for AudioInfo {
    fn default() -> Self {
        Self { volume_rms: 0. }
    }
}

impl AudioInfo {
    pub fn new_from<T>(samples: &Vec<T>) -> Self
    where
        T: cpal::SizedSample,
        f32: cpal::FromSample<T>,
    {
        let sum_sq: f32 = samples
            .iter()
            .map(|&s| {
                let s_float = s.to_sample::<f32>();
                s_float * s_float
            })
            .sum();

        let rms = (sum_sq / samples.len() as f32).sqrt();

        Self { volume_rms: rms }
    }
}