use std::sync::mpsc::Receiver;
use midly::{EventKind, MidiMessage};
use std::collections::HashMap;
use midly::number::u7;

use crate::theory::{Note, Pitch31};
use crate::data::{ActiveNote};
use crate::tonal_space::{TSPitch, TonalSpace};


pub(crate) fn process(rx: Receiver<Option<Vec<u8>>>) {

    // Keys contain collection of notes in tonal space,
    // and it maps to a list of notes across the different octaves it spans
    // TODO: Determine if the semitone interval for tonal space should
    // be octave-dependant or not. (E.g. whether C4 B4 would clear C4 from the tonal space
    // just as how C4 B3 does.
    let mut tonal_space = TonalSpace::new();

    let mut active_notes: HashMap<u7, ActiveNote> = HashMap::new();

    let mut parser_running_status = None;

    // Once None is sent, app will be terminated
    while let Some(mut raw) = rx.recv().unwrap() {
        let mut raw = raw.as_slice();
        let parsed_msg = EventKind::parse(&mut raw, &mut parser_running_status);
        let ev;
        match parsed_msg {
            Ok(event) => {
                ev = event;
            }
            Err(e) => {
                println!("error parsing midi msg: {}", e);
                continue;
            }
        }
        if let EventKind::Midi {channel, message} = ev {
            match message {
                MidiMessage::NoteOn {key, vel} => {
                    convert_to_31(key, vel, &mut tonal_space);
                }
                MidiMessage::NoteOff {key, vel} => {

                }
                MidiMessage::Controller {controller, value} => {

                }
                _ => ()
            }
        }
    }
}

pub fn convert_to_31(key: u7, vel: u7, tonal_space: &mut TonalSpace) {

}

struct ControlInterface {

}