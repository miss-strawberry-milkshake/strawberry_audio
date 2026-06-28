use num::complex::Complex32;
use std::f32::consts::TAU;

pub fn dft(samples: &[f32]) -> Vec<Complex32> {
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

pub fn print_spectrum(spectrum: &[Complex32], sample_rate: f32, width: usize, height: usize) {
    let n = spectrum.len();
    let half = n / 2;
    if half < 2 || width == 0 || height == 0 {
        return; // need at least two non-DC bins to span a log axis
    }

    let bins = &spectrum[1..=half];
    let bin_freq = |k: usize| (k as f32) * sample_rate / (n as f32);

    let (f_min, f_max) = (bin_freq(1), bin_freq(half));
    let (log_min, log_max) = (f_min.log10(), f_max.log10());

    let mut cols = vec![0.0f32; width];
    for (i, c) in bins.iter().enumerate() {
        let f = bin_freq(i + 1); // +1: we sliced starting at index 1
        let frac = (f.log10() - log_min) / (log_max - log_min);
        let col = ((frac * (width - 1) as f32).round() as usize).min(width - 1);
        cols[col] = cols[col].max(c.norm());
    }

    let max_amp = cols.iter().copied().fold(0.0f32, f32::max);
    if max_amp <= 0.0 {
        return;
    }

    for row in 0..height {
        let threshold = max_amp * (height - row) as f32 / height as f32;
        let line: String = cols
            .iter()
            .map(|&amp| if amp >= threshold { '█' } else { ' ' })
            .collect();
        println!("{line}");
    }

    println!("{}", "─".repeat(width));
    let mid_freq = 10f32.powf((log_min + log_max) / 2.0);
    let (left, mid, right) = (
        format!("{f_min:.0}"),
        format!("{mid_freq:.0}"),
        format!("{f_max:.0} Hz"),
    );
    let mut label = vec![b' '; width.max(right.len())];
    let place = |label: &mut [u8], s: &str, start: usize| {
        for (i, b) in s.bytes().enumerate() {
            if let Some(slot) = label.get_mut(start + i) {
                *slot = b;
            }
        }
    };
    place(&mut label, &left, 0);
    place(&mut label, &mid, width / 2 - mid.len() / 2);
    place(&mut label, &right, width.saturating_sub(right.len()));
    println!("{}", String::from_utf8(label).unwrap());
}
