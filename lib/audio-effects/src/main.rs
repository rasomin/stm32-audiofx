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
            let delay_samples = audio.sample_rate() / 30; // 잔향
            let samples = audio.samples();
            sample_rate = audio.sample_rate();

            for i in 0..samples.len() {
                if i >= delay_samples as usize {
                    let new_sample = samples[i] + samples[i - (delay_samples as usize)] / 2;
                    effected_samples.push(new_sample);
                } else {
                    effected_samples.push(samples[i]);
                }
            }
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
