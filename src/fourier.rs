use num::complex::Complex32;
use std::f32::consts::PI;
use std::io::{self, Write};

pub fn dft(samples: &[f32]) -> Vec<f32> {
    let number_of_samples = samples.len();
    let mut frequency_spectrum: Vec<f32> = vec![];

    for current_sample_index in 0..((number_of_samples / 2) + 1) {
        // devide by two so we only calculate up to the nyquist
        let mut sum = Complex32::new(0.0, 0.0);

        for (test_index, &test_sample) in samples.iter().enumerate() {
            // angle = -2π · k · t / N
            let angle = (-2.0 * PI) * (current_sample_index as f32) * (test_index as f32)
                / (number_of_samples as f32);

            sum += Complex32::cis(angle) * test_sample;
        }

        frequency_spectrum.push(sum.re.hypot(sum.im));
    }

    frequency_spectrum
}

pub fn fft(samples: &[f32]) -> Vec<f32> {
    assert!(
        samples.len().is_power_of_two(),
        "fft buddy, gotta be a power of two, you know the rules"
    );

    let samples_complex: Vec<Complex32> =
        samples.iter().map(|&re| Complex32::new(re, 0.0)).collect();

    let spectrum: Vec<f32> = fft_recursive(&samples_complex)
        .iter()
        .map(|sample| sample.re.hypot(sample.im))
        .collect::<Vec<f32>>()[0..(samples_complex.len() / 2)]
        .to_vec();

    spectrum
}

fn fft_recursive(samples: &[Complex32]) -> Vec<Complex32> {
    let number_of_samples = samples.len();

    if number_of_samples <= 1 {
        return samples.to_vec();
    }

    // goddamn it i love iterators in rust
    let even_samples: Vec<Complex32> = samples.iter().step_by(2).copied().collect();
    let odd_samples: Vec<Complex32> = samples.iter().skip(1).step_by(2).copied().collect();

    let odd_fft = fft_recursive(&odd_samples);
    let even_fft = fft_recursive(&even_samples);

    let mut complex_spectrum = vec![Complex32::new(0.0, 0.0); number_of_samples];

    for test_index in 0..(number_of_samples / 2) {
        let angle = (-2.0 * PI) * (test_index as f32) / (number_of_samples as f32);

        // what's a twiddle? great question...
        let twiddle = Complex32::from_polar(1.0, angle) * odd_fft[test_index];

        complex_spectrum[test_index] = even_fft[test_index] + twiddle;
        complex_spectrum[test_index + (number_of_samples / 2)] = even_fft[test_index] - twiddle;
    }

    complex_spectrum
}

pub fn print_spectrum(spectrum: Vec<f32>) {
    // Δf = f_s / 2N
    // frequency_step = sample rate / 2 * number of samples
    // we multiply by two because of the nyquist thing
    let frequency_delta = 44100 / (2 * spectrum.len());

    let mut bin_samples: Vec<f32> = vec![];
    let mut mod_for_log: f32 = 1.;
    let mod_factor = 1.2;
    let scale_factor = 20.;
    let base = 10.;

    print!("\x1B[2J\x1B[1;1H"); //clears the terminal
    io::stdout().flush().unwrap(); // resets the cursor
    println!("~~~~~~~~~~~~~~~~~~~~~");

    for (index, apmlitude) in spectrum.iter().enumerate() {
        if index < 2 {
            continue;
        } //ignore mega bass because a simple dft breaks down there. plz do a window function future me

        bin_samples.push(*apmlitude);

        if (index % (mod_for_log.floor()) as usize) == 0 {
            print!("{}\t", frequency_delta * index);

            let amp = ((bin_samples.iter().sum::<f32>()).log(base) * scale_factor).ceil() as i32;
            for _i in 0..amp {
                print!("■");
            }
            print!("\n");

            bin_samples.clear();
            mod_for_log *= mod_factor;
        }
    }
    println!("~~~~~~~~~~~~~~~~~~~~~");
}

// i'd love to write a version of this that works from the f32 vector of real amplitudes
// sucks that when we take the normal it throws out the phase, i wonder if that's not the best way to do it
// solution for now should be a frequency_spectrum struct and a time_spectrum struct, and just save both.
// that's gonna be useful for a bunch of other stuff too
pub fn _inverse_fft(spectrum: &[Complex32]) -> Vec<f32> {
    let number_of_samples = spectrum.len();
    assert!(
        number_of_samples.is_power_of_two(),
        "still needs to be a power of two because its running fft on the congjugate"
    );

    // congjugate is just the vector flipped around the real axis. so would map 1 - 3i to 1 + 3i
    let conjugated: Vec<Complex32> = spectrum.iter().map(|sample| sample.conj()).collect();

    let time_domain = fft_recursive(&conjugated);

    let scale = 1. / (number_of_samples as f32);
    time_domain.iter().map(|c| c.conj().re * scale).collect()
}

// erm... figure out how to test if this is working... also write a graphic representation... might need real graphics ):