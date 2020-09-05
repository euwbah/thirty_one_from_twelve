use std::collections::HashMap;
use crate::theory::{Note, Pitch31};
use midly::number::u7;

/// An interval that is this number of steps or less apart will be regarded as a "semitone" clash
const SEMITONE_THRESHOLD: i16 = 3;

/// Once TSPitch.clash_counter exceeds this number it will destroy itself
/// Usage: If you believe a major 7th/min 9th has equal precedence in the step-wise shift of a note in
///        the tonal space as a min 2nd, set this to zero.
///
///        If you think octave reduction is an invalid concept, set this value to
///        (x - 1) where x is the coefficient multiplier of how much more important the
///        < min2nd interval is as compared to octave-displaced inversions.
const DIFF_OCT_CLASH_THRESHOLD: u8 = 0;

/// The second most recent note is this much as important as the first,
/// the third most recent note is this much as important as the second,
/// etc...
const ORDER_PRECEDENCE_COEFFICIENT: f64 = 0.99;

#[derive(Default)]
pub struct TonalSpace {
    notes: HashMap<Note, Vec<TSPitch>>,
    note_order: Vec<Note>
}

impl TonalSpace {
    pub fn new() -> Self {
        let mut notes = HashMap::new();
        notes.insert(Note::C,
                     vec![TSPitch::new(
                         Pitch31::new("C4").unwrap(),
                         u7::from(60))]);

        TonalSpace {
            notes,
            note_order: vec![Note::C]
        }
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

        // Loop through candidate notes that are considered adjacent and 'clashing' w.r.t. the
        // newly-added pitch. Remove them if they are present in the tonal space.
        for dieses in -SEMITONE_THRESHOLD..=SEMITONE_THRESHOLD {
            if dieses == 0 {
                continue
            }

            let n = Note::from(pitch.note.to_steps_from_a() + dieses);
            if let Some(adj_pitch_octaves) = self.notes.get_mut(&n) {
                adj_pitch_octaves.retain(|adj_pitch| {
                    if (adj_pitch.pitch.to_steps_from_a4() - pitch.to_steps_from_a4()).abs() > SEMITONE_THRESHOLD {
                        // Not same octave, use DIFF_OCT_CLASH_THRESHOLD to determine if the
                        // adjacent note should be kept as part of the tonal space
                        if adj_pitch.clash_counter != DIFF_OCT_CLASH_THRESHOLD {
                            return true
                        }
                    }

                    // Otherwise, remove true adjacent note from the tonal space
                    false
                });
            }
        }

        // Remove new note from note_order
        self.note_order.retain(|n| *n != pitch.note);

        // Set most recent note as top of note order
        self.note_order.insert(0, pitch.note);
    }

    /// Returns all possible note candidates sorted by best (lowest) assonance score first.
    pub fn convert_to_31(&self, midi_note: u7, am: AssonanceMetric, pt: ProjectionType) -> Vec<(Pitch31, f64)> {
        let mut scores = HashMap::new();

        let mut order_multiplier = 1.0;

        for n in &self.note_order {
            if let Some(pitches) = self.notes.get(&n) {
                for ts_pitch in pitches {
                    let candidates = ts_pitch.get_candidate_projections(midi_note, pt);
                    for can in candidates {
                        // divis by number of pitches in the same octave necessary to prevent double counting
                        // octaves (TODO: or is it?)
                        let assonance =
                            ts_pitch.get_assonance_coefficient(can, am)
                                / pitches.len() as f64
                                * order_multiplier;

                        if let Some( score) = scores.get_mut(&can) {
                            *score += assonance;
                        } else {
                            scores.insert(can, assonance);
                        }
                    }
                }
            }
            order_multiplier *= ORDER_PRECEDENCE_COEFFICIENT;
        }

        let mut sorted = scores
            .iter()
            .map(|(p, s)| (*p, *s))
            .collect::<Vec<_>>();
        sorted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        sorted
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

    /// The lower the number, the more assonant (mathematically preferable) the interval is when
    /// compared with the selected metric
    pub fn get_assonance_coefficient(&self, pitch: Pitch31, metric: AssonanceMetric) -> f64 {
        match metric {
            AssonanceMetric::Pythagorean => {
                pitch.note.fifths_to(self.pitch.note) as f64
            }
        }
    }

    /// Get possible 31 edo realisations of the 12 edo interval created between
    /// `self.midi_key` and `midi_note`
    pub fn get_candidate_projections(&self, midi_note: u7, projection_type: ProjectionType) -> Vec<Pitch31> {
        let dist12 = (midi_note.as_int() as i16) - (self.midi_key.as_int() as i16);
        let dist12_octs = dist12.div_euclid(12);
        let dist12_semis = dist12.rem_euclid(12);

        match projection_type {
            ProjectionType::Meantone17 => {
                let candidate = match dist12_semis {
                    0 => vec![0],
                    1 => vec![2, 3],
                    2 => vec![5],
                    3 => vec![7, 8],
                    4 => vec![10],
                    5 => vec![13],
                    6 => vec![15, 16],
                    7 => vec![18],
                    8 => vec![20, 21],
                    9 => vec![23],
                    10 => vec![25, 26],
                    11 => vec![28],
                    _ => panic!("impossible scenario")
                };

                candidate.into_iter().map(|x| self.pitch + (x + 31 * dist12_octs)).collect()
            }
            ProjectionType::Meantone31KeepUnison => {
                let candidate = match dist12_semis {
                    0 => vec![0],
                    1 => vec![2, 3],
                    2 => vec![5, 4, 6],
                    3 => vec![7, 8],
                    4 => vec![10, 9, 11],
                    5 => vec![13, 12, 14],
                    6 => vec![15, 16],
                    7 => vec![18, 17, 19],
                    8 => vec![20, 21],
                    9 => vec![23, 22, 24],
                    10 => vec![25, 26],
                    11 => vec![28, 27, 29],
                    _ => panic!("impossible scenario")
                };

                candidate.into_iter().map(|x| self.pitch + (x + 31 * dist12_octs)).collect()
            }
        }
    }
}

#[derive(Copy, Clone)]
pub enum AssonanceMetric {
    Pythagorean
}

#[derive(Copy, Clone)]
pub enum ProjectionType {
    /// U1, P4/5, Maj2/3/6/7 -> Only 1 option
    /// m2/3/6/7, dim5 -> 2 options: #/b variants
    Meantone17,

    /// U1 -> Only 1 option
    /// P4/5, Maj2/3/6/7 -> 3 options: v / natural / ^
    /// m2/3/6/7, dim5 -> 2 options: #/b variants
    Meantone31KeepUnison
}