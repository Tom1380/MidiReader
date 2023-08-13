pub mod custom_sounds;

use custom_sounds::CustomSound;

use rodio::{OutputStream, OutputStreamHandle, Sample, Sink};
use std::collections::HashMap;
use std::marker::PhantomData;

// 36 -> C2
// 37 -> C#2
// ...
// 96 -> C7
// We don't need it now, but it's very handy to keep.
#[allow(dead_code)]
pub fn note_name(note_index: u8) -> String {
    let octave = (note_index / 12) - 1;
    let remainder = note_index % 12;
    // The 12 notes, octave independent.
    let absolute_notes = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
    let note = absolute_notes[remainder as usize];
    format!("{note}{octave}")
}

#[derive(Debug)]
pub enum NoteMessage {
    // Note index, velocity
    On(u8, u8),
    // Note index
    Off(u8),
    // Stop all notes
    Reset,
}

pub struct NotePlayer<S> {
    // We don't use it, but stream_handle needs it to work.
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sinks: HashMap<u8, Sink>,
    volume: f32,
    _sound_type: PhantomData<S>,
}

impl<S> NotePlayer<S>
where
    S: 'static + CustomSound + Send,
    S::Item: Sample + Send,
{
    pub fn new() -> NotePlayer<S> {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        NotePlayer {
            _stream,
            stream_handle,
            sinks: HashMap::new(),
            volume: 0.1,
            _sound_type: PhantomData,
        }
    }

    pub fn note_on(&mut self, note_index: u8, velocity: u8) {
        match self.sinks.get(&note_index) {
            Some(_) => {}
            None => {
                self.sinks
                    .insert(note_index, self.build_sink(note_index, velocity));
            }
        }
    }

    fn build_sink(&self, note_index: u8, velocity: u8) -> Sink {
        let sink = Sink::try_new(&self.stream_handle).unwrap();
        sink.set_volume(self.volume * velocity as f32 / 100.0);
        let sound_wave = Self::get_sound_wave(note_index);
        sink.append(sound_wave);
        sink
    }

    pub fn note_off(&mut self, note_index: u8) {
        match self.sinks.remove(&note_index) {
            Some(sink) => sink.stop(),
            None => {}
        }
    }

    pub fn reset(&mut self) {
        for sink in self.sinks.values() {
            sink.stop();
        }

        self.sinks.clear();
    }

    fn get_sound_wave(note_index: u8) -> S {
        S::new(Self::get_frequency(note_index))
    }

    /// MIDI supports notes from A0 onwards.
    /// A0's MIDI code is 21 and it's frequency is 27.50hz.
    fn get_frequency(note_index: u8) -> f32 {
        let distance_from_a0_in_semitones = note_index - 21;
        // 27.50 is the frequency for our chosen starting point, which is A0.
        27.50
            * (2.0_f32)
                // ¹²√2 because it needs to be 2 when raised to the power of 12.
                // In other words, it needs to double every 12 notes, meaning the whole octave.
                .powf(distance_from_a0_in_semitones as f32 / 12.0)
    }
}

impl<S> Default for NotePlayer<S>
where
    S: 'static + CustomSound + Send,
    S::Item: Sample + Send,
{
    fn default() -> Self {
        Self::new()
    }
}
