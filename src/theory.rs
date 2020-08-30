use num_traits::FromPrimitive;
use regex::Regex;
use lazy_static;
use std::str::FromStr;

lazy_static! {
    // Splits a str into note and acc
    static ref NOTE_REGEX: Regex = Regex::new(r"(?P<note>[a-gA-G])(?P<acc>.*)")?;
    // Splits a strs into note and octave, to get acc of note, pass into NOTE_REGEX
    static ref PITCH_REGEX: Regex = Regex::new(r"(?P<note>[a-gA-G].*?)(?P<oct>-*\d+)")?;
}

struct Pitch31 {
    note: Note,
    octave: i8,
}

impl Pitch31 {
    pub fn new(s: &str) -> Result<Self, &str> {
        let capts = PITCH_REGEX.captures(s);
        match capts {
            Some(capts) => {
                let note_str = capts.name("note").unwrap();
                let oct = capts.name("oct").unwrap();

                match Note::new(note_str.as_str()) {
                    Ok(note) => {
                        if let Ok(octave) = i8::from_str(oct.as_str()) {
                            Ok(Pitch31{
                                note, octave
                            })
                        } else {
                            Err("invalid octave: " + oct.as_str())
                        }
                    }
                    Err(e) => Err("error constructing note: " + e)
                }
            }
            None => Err("invalid pitch name in pitch constructor: " + s)
        }
    }

    pub fn to_steps_from_a4(&self) -> i16 {
        self.note.to_steps_from_a() + 31 * (self.octave - 4)
    }
}

#[derive(FromPrimitive)]
enum Note {
    Abb, Ebb, Bbb,
    // Cb is renamed BuCb to prevent confusion of the octaves
    // Bu4 and Cb4 are technically in two different octaves
    // When dealing with this enum, the octave only
    // begins at exactly C
    Fb, BuCb, Gb, Db, Ab, Eb, Bb,
    F, C, G, D, A, E, B,
    Fs, Cs, Gs, Ds, As, Es, Bs,
    Fx, Cx, Gx, DxFbb, AxCbb, ExGbb, BxDbb,
}

impl Note {
    pub fn new(s: &str) -> Result<Self, &str> {
        let capts = NOTE_REGEX.captures(s);
        match capts {
            Some(capts) => {
                let note = capts.name("note").unwrap();
                let acc = capts.name("acc");

                match acc {
                    None => {
                        match note.as_str() {
                            "A" => Ok(Note::A),
                            "B" => Ok(Note::B),
                            "C" => Ok(Note::C),
                            "D" => Ok(Note::D),
                            "E" => Ok(Note::E),
                            "F" => Ok(Note::F),
                            "G" => Ok(Note::G),
                            _ => Err("impossible note name??")
                        }
                    }
                    Some(acc) => {
                        match acc.as_str() {
                            "bb" =>
                                match note.as_str() {
                                    "A" => Ok(Note::Abb),
                                    "B" => Ok(Note::Bbb),
                                    "C" => Ok(Note::AxCbb),
                                    "D" => Ok(Note::BxDbb),
                                    "E" => Ok(Note::Ebb),
                                    "F" => Ok(Note::DxFbb),
                                    "G" => Ok(Note::ExGbb),
                                    _ => Err("impossible note name??")
                                }
                            "bv" | "bb^" =>
                                match note.as_str() {
                                    "A" => Ok(Note::Gs),
                                    "B" => Ok(Note::As),
                                    "C" => Ok(Note::B),
                                    "D" => Ok(Note::Cs),
                                    "E" => Ok(Note::Ds),
                                    "F" => Ok(Note::Es),
                                    "G" => Ok(Note::Fs),
                                    _ => Err("impossible note name??")
                                }
                            "b" =>
                                match note.as_str() {
                                    "A" => Ok(Note::Ab),
                                    "B" => Ok(Note::Bb),
                                    "C" => Ok(Note::BuCb),
                                    "D" => Ok(Note::Db),
                                    "E" => Ok(Note::Eb),
                                    "F" => Ok(Note::Fb),
                                    "G" => Ok(Note::Gb),
                                    _ => Err("impossible note name??")
                                }
                            "v" =>
                                match note.as_str() {
                                    "A" => Ok(Note::Gx),
                                    "B" => Ok(Note::AxCbb),
                                    "C" => Ok(Note::Bs),
                                    "D" => Ok(Note::Cx),
                                    "E" => Ok(Note::DxFbb),
                                    "F" => Ok(Note::Es),
                                    "G" => Ok(Note::Fx),
                                    _ => Err("impossible note name??")
                                }
                            "^" | "#v" =>
                                match note.as_str() {
                                    "A" => Ok(Note::Bbb),
                                    "B" => Ok(Note::BuCb),
                                    "C" => Ok(Note::BxDbb),
                                    "D" => Ok(Note::Ebb),
                                    "E" => Ok(Note::Fb),
                                    "F" => Ok(Note::ExGbb),
                                    "G" => Ok(Note::Abb),
                                    _ => Err("impossible note name??")
                                }
                            "#" =>
                                match note.as_str() {
                                    "A" => Ok(Note::As),
                                    "B" => Ok(Note::Bs),
                                    "C" => Ok(Note::Cs),
                                    "D" => Ok(Note::Ds),
                                    "E" => Ok(Note::Es),
                                    "F" => Ok(Note::Fs),
                                    "G" => Ok(Note::Gs),
                                    _ => Err("impossible note name??")
                                }
                            "#^" | "xv" =>
                                match note.as_str() {
                                    "A" => Ok(Note::Bb),
                                    "B" => Ok(Note::C),
                                    "C" => Ok(Note::Db),
                                    "D" => Ok(Note::Eb),
                                    "E" => Ok(Note::F),
                                    "F" => Ok(Note::Gb),
                                    "G" => Ok(Note::Ab),
                                    _ => Err("impossible note name??")
                                }
                            "x" =>
                                match note.as_str() {
                                    "A" => Ok(Note::AxCbb),
                                    "B" => Ok(Note::BxDbb),
                                    "C" => Ok(Note::Cx),
                                    "D" => Ok(Note::DxFbb),
                                    "E" => Ok(Note::ExGbb),
                                    "F" => Ok(Note::Fx),
                                    "G" => Ok(Note::Gx),
                                    _ => Err("impossible note name??")
                                }
                            _ => Err(acc.as_str() + " is an invalid accidental")
                        }
                    }
                }
            }
            None => Err("invalid note name in Note contructor: " + s)
        }
    }

    /// Returns number of steps this particular note is from A in the same octave
    ///
    /// e.g. if the note is G, this note will return -5 as G is 5 dieses below A
    pub fn to_steps_from_a(&self) -> i16 {
        use Note::*;
        match self {
            A => 0, Bbb => 1, As => 2, Bb => 3, AxCbb => 4,
            B => 5, BuCb => 6, Bs => 7,
            C => -23, BxDbb => -22, Cs => -21, Db => -20, Cx => -19,
            D => -18, Ebb => -17, Ds => -16, Eb => -15, DxFbb => -14,
            E => -13, Fb => -12, Es => -11,
            F => -10, ExGbb => -9, Fs => -8, Gb => -7, Fx => -6,
            G => -5, Abb => -4, Gs => -3, Ab => -2, Gx => -1
        }
    }
}