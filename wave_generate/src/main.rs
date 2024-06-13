use std::f32::consts::PI;
use hound;

const SAMPLE_RATE : u32 = 44100;

fn main() {
    let freq = 440.0;
    let duration = 2.0;

    let samples = generate_sine(freq, duration, SAMPLE_RATE);

    write_to_file("sine_wave.wav", &samples, SAMPLE_RATE).expect("Failed to write to file :(");
}

fn generate_sine(frequency: f32, duration: f32, sample_rate: u32) -> Vec<i16> {
    let num_samples = (duration * sample_rate as f32) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples{
        let sample = (i as f32 * frequency * 2.0 * PI / sample_rate as f32).sin();

        samples.push((sample * i16::MAX as f32) as i16);
    }

    samples
}

fn write_to_file(filename: &str, samples: &[i16], sample_rate: u32) -> hound::Result<()> {
    let spec = hound::WavSpec{
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(filename, spec)?;

    for &sample in samples{
        writer.write_sample(sample)?;
    }

    writer.finalize();
    Ok(())
}   
