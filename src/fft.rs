use num::complex::Complex32;
use std::f32::consts::TAU;

pub fn _fft(samples: &[f32]) -> Vec<Complex32> {
    let number_of_samples = samples.len();
    let mut output = Vec::with_capacity(number_of_samples);

    for current_sample_index in 0..number_of_samples {
        let mut sum = Complex32::new(0.0, 0.0);

        for (test_index, &test_sample) in samples.iter().enumerate() {
            // angle = -2π · k · t / N
            let angle = -1.0 * TAU * (current_sample_index as f32) * (test_index as f32)
                / (number_of_samples as f32);
            sum += Complex32::cis(angle) * test_sample;
        }

        output.push(sum);
    }
    output
}
