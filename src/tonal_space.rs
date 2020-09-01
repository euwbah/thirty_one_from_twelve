use std::collections::HashMap;
use crate::theory::{Note, Pitch31};
use midly::number::u7;
use std::default;

/// An interval that is this number of steps or less apart will be regarded as a "semitone" clash
const SEMITONE_THRESHOLD: i8 = 3;

/// Once TSPitch.clash_counter exceeds this number it will destroy itself
const CLASH_THRESHOLD: u8 = 1;

#[derive(Default)]
pub struct TonalSpace {
    notes: HashMap<Note, Vec<TSPitch>>,
    note_order: Vec<TSPitch>
}

impl TonalSpace {
    pub fn new() -> Self {
       Default::default()
    }

    pub fn insert(&mut self, pitch: Pitch31, midi_key: u7) {
        let to_add = TSPitch::new(pitch, midi_key);

        match self.notes.get(&pitch.note) {
            Some(tspitches) => {
                let mut already_contained = false;
                for p in tspitches {
                    if p.pitch == pitch {
                        already_contained = true;
                        break;
                    }
                }
                if !already_contained {
                    self.notes.get_mut(&pitch.note).unwrap().push(to_add);
                }
            }
            None => {
                self.notes.insert(pitch.note, vec![to_add]);
            }
        }

        for semis in -SEMITONE_THRESHOLD..=SEMITONE_THRESHOLD {

        }
    }


}

pub struct TSPitch {
    // Abstraction for values of tonal space hashmap, for future use of assigning
    // additional properties to each note in the tonal space
    pitch: Pitch31,

    /// the counter increments when notes are played in other octaves that could clash with
    /// the current note by less than a semitone, if the octave displacement was discounted for.
    clash_counter: u8,

    /// The midi key that triggered this note into existence
    /// Used to calculate the expected resultant interval after conversion
    midi_key: u7
}

impl TSPitch {
    pub fn new(pitch: Pitch31, midi_key: u7) -> Self {
        TSPitch {
            pitch,
            clash_counter: 0,
            midi_key
        }
    }
}

