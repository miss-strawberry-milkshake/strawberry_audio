use num::complex::Complex32;
use std::f32::consts::PI;
use std::io::{self, Write};

pub fn dft(samples: &[f32]) -> Vec<f32> {
    let number_of_samples = samples.len();
    let mut frequency_spectrum : Vec<f32> = vec![];

    for current_sample_index in 0..((number_of_samples/2)+1) { // devide by two so we only calculate up to the nyquist
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

pub fn print_spectrum(spectrum: Vec<f32>){

        print!("\x1B[2J\x1B[1;1H"); //clears the terminal
        io::stdout().flush().unwrap(); // resets the cursor

        // Δf = f_s / 2N
        // frequency_step = sample rate / 2 * number of samples
        // we multiply by two because of the nyquist thing
        let frequency_delta = 44100/(2 * spectrum.len());

        println!("~~~~~~~~~~~~~~~~~~~~~");

        let mut bin_samples: Vec<f32> = vec![];
        let mut mod_for_log: f32 = 1.;
        let mod_factor = 1.2;
        let scale_factor = 20.;
        let base = 10.;

        for (index, apmlitude) in spectrum.iter().enumerate() {

                if index < 2 { continue; } //ignore mega bass because a simple dft breaks down there. ! fix later !

                bin_samples.push(*apmlitude);

                if (index % (mod_for_log.floor()) as usize) == 0{

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