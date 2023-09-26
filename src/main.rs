use midir::MidiInput;
use note_player::custom_sounds::*;
use note_player::*;
use std::error::Error;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let midi_in = MidiInput::new("midir reading input")?;
    let in_ports = midi_in.ports();
    let in_port = &in_ports[1];
    let in_port_name = midi_in.port_name(in_port)?;

    let note_player = note_player::<SawWave>();

    // The connection closes when this is dropped, so it needs to be bound.
    let _conn_in = midi_in.connect(in_port, &in_port_name, callback, note_player)?;

    println!("Connection open, reading input from '{in_port_name}'",);

    loop {
        std::thread::sleep(std::time::Duration::from_secs(u64::MAX));
    }
}

fn callback(_timestamp: u64, message: &[u8], note_player: &mut NotePlayerHandle) {
    // https://www.midi.org/specifications-old/item/table-2-expanded-messages-list-status-bytes
    if let [144, note_index, velocity] = message {
        match velocity {
            0 => note_player.note_off(*note_index),
            _ => note_player.note_on(*note_index, *velocity),
        }
        .unwrap();
    }
}
