extern crate coreaudio;

use coreaudio::audio_unit::audio_format::LinearPcmFlags;
use coreaudio::audio_unit::macos_helpers::{audio_unit_from_device_id, get_default_device_id};
use coreaudio::audio_unit::render_callback::{self, data};
use coreaudio::audio_unit::{Element, SampleFormat, Scope, StreamFormat};
use objc2_audio_toolbox::kAudioUnitProperty_StreamFormat;
use rtrb::RingBuffer;

mod dft;
mod fft;
// mod sin;
// mod time_utils;
mod wav_file;

fn main() -> Result<(), coreaudio::Error> {
    // ~~~ CORE AUDIO SETUP ~~~

    let stream_format = StreamFormat {
        sample_rate: 44100.,
        sample_format: SampleFormat::F32,
        flags: LinearPcmFlags::IS_FLOAT | LinearPcmFlags::IS_PACKED,
        channels: 2,
    };
    let audio_unit_id = get_default_device_id(false).unwrap();
    let mut audio_unit = audio_unit_from_device_id(audio_unit_id, false)?;
    let id = kAudioUnitProperty_StreamFormat;
    let stream_format_asbd = stream_format.to_asbd();
    audio_unit.set_property(id, Scope::Input, Element::Output, Some(&stream_format_asbd))?;
    let stream_format = audio_unit.output_stream_format()?;
    assert_eq!(
        stream_format.sample_format,
        SampleFormat::F32,
        "plz use float"
    );
    assert_eq!(stream_format.sample_rate, 44100., "plz use 441k");
    type Args = render_callback::Args<data::Interleaved<f32>>;

    // ~~~ LOAD WAVE FILE ~~~

    let _white_noise = "./audio/white_noise.wav";
    let ptv = "./audio/Pierce The Veil - Collide With The Sky - 11 Stained Glass Eyes And Colorful Tears.wav";

    let mut song = wav_file::Wav::new(ptv,);

    let (mut producer, mut consumer) = RingBuffer::<f32>::new(1024);

    // ~~~ MAIN LOOP ~~~

    audio_unit.set_render_callback(move |args| {
        let Args {
            num_frames, data, ..
        } = args;

        assert_eq!(num_frames, 512, "for 44.1, the buffer size should be 512");

        let mut stero_buffer: Vec<f32> = vec![];

        for _i in 0..num_frames {
            let song_samples = song.next();

            if song_samples == None { break; }

            let left = song_samples.unwrap()[0];
            let right = song_samples.unwrap()[1];

            stero_buffer.push(left);
            stero_buffer.push(right);
        }

        assert_eq!(stero_buffer.len(), 1024, "512 frames * 2 channels should equal 1024");

        data.buffer
            .iter_mut()
            .zip(stero_buffer.iter())
            .for_each(|(d, s)| *d = s.clone());

        let (_pushed, _remainder) = producer.push_partial_slice(&stero_buffer);

        Ok(())
    })?;


        const FRAME_SIZE: usize = 1024;
        const ANALYSIS_LENGTH: usize = FRAME_SIZE * 2;
        let mut analysis_buffer: Vec<f32> = vec![];
        let mut counter = 0;

    audio_unit.start()?;


    loop { // infinite loop, fix to have this break once the song ends or whatever

        let mut poppy_vec = vec![0.; FRAME_SIZE];
        let (_filled, _remainder) = consumer.pop_partial_slice(&mut poppy_vec);

        analysis_buffer.append(&mut poppy_vec.clone());

        counter += 1;
        if counter % (ANALYSIS_LENGTH/FRAME_SIZE) != 0 { continue; };

        let mut mono_buffer = [0.; ANALYSIS_LENGTH/2];

        for i in 0..mono_buffer.len(){
                mono_buffer[i] = (analysis_buffer[2*i] * analysis_buffer[(2*i)+1])/2.; //take the average of the stero signal for mono
        }

        let spectrum = dft::dft(&mono_buffer);
        dft::print_spectrum(spectrum);

        analysis_buffer.clear();

    }
}
