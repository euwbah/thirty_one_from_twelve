use std::sync::mpsc::Receiver;
use midly::{EventKind, MidiMessage};
use std::collections::HashMap;
use midly::number::u7;

pub(crate) fn process(rx: Receiver<Option<EventKind>>) {

    // Keys contain collection of notes in tonal space,
    // and it maps to a list of notes across the different octaves it spans
    // TODO: Determine if the semitone interval for tonal space should
    // be octave-dependant or not. (E.g. whether C4 B4 would clear C4 from the tonal space
    // just as how C4 B3 does.
    let mut tonal_space: HashMap<Note, Vec<TSPitch>> = HashMap::new();

    let mut active_notes: HashMap<u7, Pitch31> = HashMap::new();


    // Once None is sent, app will be terminated
    while let Some(ev) = rx.recv() {
        if let EventKind::Midi {channel, message} = ev {
            match message {
                MidiMessage::NoteOn {key, vel} => {

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

struct TSPitch {
    // Abstraction for values of tonal space hashmap, for future use of assigning
    // additional properties to each note in the tonal space
    pitch: Pitch31
}