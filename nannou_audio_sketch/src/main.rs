// First audio program with nannou, working off an example code 
// mostly trying to understand how to use nannou_audio
// not concerned with best practices and code quality, obviously it's
// not a good idea to sleep the thread in the key_pressed function
// but it's just for testing purposes

use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use std::f64::consts::PI;

const MIDI_NOTES: [u8; 128] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
    62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91,
    92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117,
    118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
];

fn main() {
    nannou::app(model).run();
}

struct Model {
    stream: audio::Stream<Audio>,
}

struct Audio {
    phase: f64,
    hz: f64,
}

fn model(app: &App) -> Model {
    app.new_window()
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();

    let audio_host = audio::Host::new();

    let model = Audio {
        phase: 0.0,
        hz: 440.0,
    };

    let stream = audio_host
        .new_output_stream(model)
        .render(audio)
        .build()
        .unwrap();

    // stream.play().unwrap();

    Model { stream }
}

fn audio(audio: &mut Audio, buffer: &mut Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    let volume = 0.5;
    for frame in buffer.frames_mut() {
        let sine_amp = (2.0 * PI * audio.phase).sin() as f32;
        audio.phase += audio.hz / sample_rate;
        audio.phase %= sample_rate;
        for channel in frame {
            *channel = sine_amp * volume;
        }
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::A =>{
            model.stream.send(|audio|{
                audio.hz = midi_note_to_frequency(MIDI_NOTES[65]);
            }).unwrap();
            model.stream.play().unwrap();
            // pause after 1 second
            std::thread::sleep(std::time::Duration::from_secs(1));
            model.stream.pause().unwrap();
        }
        Key::S =>{
            model.stream.send(|audio|{
                audio.hz = midi_note_to_frequency(MIDI_NOTES[66]);
            }).unwrap();
            model.stream.play().unwrap();
            // pause after 1 second
            std::thread::sleep(std::time::Duration::from_secs(1));
            model.stream.pause().unwrap();
        }
        Key::D =>{
            model.stream.send(|audio|{
                audio.hz = midi_note_to_frequency(MIDI_NOTES[67]);
            }).unwrap();
            model.stream.play().unwrap();
            // pause after 1 second
            std::thread::sleep(std::time::Duration::from_secs(1));
            model.stream.pause().unwrap();
        }
        Key::F =>{
            model.stream.send(|audio|{
                audio.hz = midi_note_to_frequency(MIDI_NOTES[68]);
            }).unwrap();
            model.stream.play().unwrap();
            // pause after 1 second
            std::thread::sleep(std::time::Duration::from_secs(1));
            model.stream.pause().unwrap();
        }
        Key::G =>{
            model.stream.send(|audio|{
                audio.hz = midi_note_to_frequency(MIDI_NOTES[69]);
            }).unwrap();
            model.stream.play().unwrap();
            // pause after 1 second
            std::thread::sleep(std::time::Duration::from_secs(1));
            model.stream.pause().unwrap();
        }
        Key::H =>{
            model.stream.send(|audio|{
                audio.hz = midi_note_to_frequency(MIDI_NOTES[70]);
            }).unwrap();
            model.stream.play().unwrap();
            // pause after 1 second
            std::thread::sleep(std::time::Duration::from_secs(1));
            model.stream.pause().unwrap();
        }
        Key::J =>{
            model.stream.send(|audio|{
                audio.hz = midi_note_to_frequency(MIDI_NOTES[71]);
            }).unwrap();
            model.stream.play().unwrap();
            // pause after 1 second
            std::thread::sleep(std::time::Duration::from_secs(1));
            model.stream.pause().unwrap();
        }
        Key::K =>{
            model.stream.send(|audio|{
                audio.hz = midi_note_to_frequency(MIDI_NOTES[72]);
            }).unwrap();
            model.stream.play().unwrap();
            // pause after 1 second
            std::thread::sleep(std::time::Duration::from_secs(1));
            model.stream.pause().unwrap();
        }
        Key::L =>{
            model.stream.send(|audio|{
                audio.hz = midi_note_to_frequency(MIDI_NOTES[73]);
            }).unwrap();
            model.stream.play().unwrap();
            // pause after 1 second
            std::thread::sleep(std::time::Duration::from_secs(1));
            model.stream.pause().unwrap();
        }

        _ => {}
    }
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(BLACK);
}

fn midi_note_to_frequency(note: u8) -> f64 {
    let a4_note: f64 = 69.0;
    let a4_freq: f64 = 440.0;
    let semitone_ratio: f64 = 2.0_f64.powf(1.0 / 12.0);

    a4_freq * semitone_ratio.powf(note as f64 - a4_note)
}