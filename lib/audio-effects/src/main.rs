use std::fs;
use rmp3::{Decoder, Frame};
use hound;

fn main() {
    let mp3 = fs::read("sample.mp3").expect("file not found");
    let mut decoder = Decoder::new(&mp3);

    let mut effected_samples: Vec<i16> = Vec::new();
    let mut sample_rate = 0;

    while let Some(frame) = decoder.next() {
        if let Frame::Audio(audio) = frame {
            let samples = audio.samples();
            sample_rate = audio.sample_rate();

            // let mut effected = effect_reverb(&samples, sample_rate);
            let mut effected = effect_gain(&samples, 5.5);
            effected_samples.append(&mut effected);
        }
    }

    save_wav("output.wav", &effected_samples, sample_rate);
}

fn save_wav(filename: &str, samples: &[i16], sample_rate: u32) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(filename, spec).expect("Failed to create WAV file");

    for &sample in samples {
        writer.write_sample(sample).expect("Failed to write sample");
    }

    writer.finalize().expect("Failed to finalize WAV file");
}

fn effect_gain(samples: &[i16], gain: f32) -> Vec<i16> {
    let mut effected_samples_boost: Vec<i16> = Vec::new();

    for &sample in samples {
        let new_sample = ((sample as f32) * gain) as i16;
        effected_samples_boost.push(new_sample);
    }

    return effected_samples_boost;
}

fn _effect_reverb(samples: &[i16], sample_rate: u32) -> Vec<i16> { // doesn't work
    let delay_samples = sample_rate / 30;
    let mut effected_samples_reverb: Vec<i16> = Vec::new();

    for i in 0..samples.len() {
        if i >= delay_samples as usize {
            let new_sample = samples[i] + samples[i - (delay_samples as usize)] / 2;
            effected_samples_reverb.push(new_sample);
        } else {
            effected_samples_reverb.push(samples[i]);
        }
    }

    return effected_samples_reverb;
}
