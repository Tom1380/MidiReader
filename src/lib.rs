mod sound_playing;

pub use sound_playing::custom_sounds;
pub use sound_playing::note_name;

use sound_playing::custom_sounds::CustomSound;
use sound_playing::*;

use rodio::Sample;
use std::sync::mpsc;
use std::thread;

pub struct NotePlayerHandle {
    tx: mpsc::Sender<NoteMessage>,
}

pub fn note_player<S>() -> NotePlayerHandle
where
    S: 'static + CustomSound + Send,
    S::Item: Sample + Send,
{
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut player: NotePlayer<S> = NotePlayer::new();

        for msg in rx {
            match msg {
                NoteMessage::On(note_index, velocity) => {
                    player.note_on(note_index, velocity);
                }
                NoteMessage::Off(note_index) => {
                    player.note_off(note_index);
                }
                NoteMessage::Reset => {
                    player.reset();
                }
            }
        }
    });

    NotePlayerHandle { tx }
}

impl NotePlayerHandle {
    pub fn note_on(
        &self,
        note_index: u8,
        velocity: u8,
    ) -> Result<(), mpsc::SendError<NoteMessage>> {
        self.tx.send(NoteMessage::On(note_index, velocity))
    }

    pub fn note_off(&self, note_index: u8) -> Result<(), mpsc::SendError<NoteMessage>> {
        self.tx.send(NoteMessage::Off(note_index))
    }

    pub fn reset(&self) -> Result<(), mpsc::SendError<NoteMessage>> {
        self.tx.send(NoteMessage::Reset)
    }
}
