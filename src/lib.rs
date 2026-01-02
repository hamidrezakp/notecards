use crate::midi::setup_midi;
use midly::live::LiveEvent;
use wasm_bindgen::prelude::*;

mod midi;
mod utils;

#[wasm_bindgen(start)]
pub async fn start() {
    let data = 0usize;
    setup_midi(parse_midi_message, data).await.unwrap();
}

fn parse_midi_message(timestamp: u64, message: &[u8], data: &mut usize) {
    let message = match LiveEvent::parse(message) {
        Ok(LiveEvent::Midi { channel, message }) => (channel, message),
        Err(e) => {
            jprintln!("Parsing midi message failed: {e:?}");
            return;
        }
        _ => return,
    };

    match message.1 {
        //midly::MidiMessage::NoteOff { key, .. } |
        midly::MidiMessage::NoteOn { key, .. } => jprintln!("{key}"),
        _ => {}
    }
}
