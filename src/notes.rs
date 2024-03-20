use std::{collections::HashSet, fmt};

use crate::intervals::{Diatonic, Interval};

pub type Accidentals = i32;
pub const NATURAL: i32 = 0;
pub const SHARP: i32 = 1;
pub const DOUBLE_SHARP: i32 = 2;
pub const FLAT: i32 = -1;
pub const DOUBLE_FLAT: i32 = -2;

pub type Octave = u32;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NoteName {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Note {
    pub pitch: Pitch,
    pub octave: Octave,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pitch {
    pub name: NoteName,
    pub accidentals: Accidentals,
}

impl Pitch {
    pub fn next(&self, semitones: i32) -> Pitch {
        use NoteName::*;
        match self.name {
            C => Pitch {
                name: D,
                accidentals: self.accidentals + semitones - 2,
            },
            D => Pitch {
                name: E,
                accidentals: self.accidentals + semitones - 2,
            },
            E => Pitch {
                name: F,
                accidentals: self.accidentals + semitones - 1,
            },
            F => Pitch {
                name: G,
                accidentals: self.accidentals + semitones - 2,
            },
            G => Pitch {
                name: A,
                accidentals: self.accidentals + semitones - 2,
            },
            A => Pitch {
                name: B,
                accidentals: self.accidentals + semitones - 2,
            },
            B => Pitch {
                name: C,
                accidentals: self.accidentals + semitones - 1,
            },
        }
    }

    pub fn prev(&self, semitones: i32) -> Pitch {
        use NoteName::*;

        match self.name {
            C => Pitch {
                name: B,
                accidentals: self.accidentals - semitones + 1,
            },
            D => Pitch {
                name: C,
                accidentals: self.accidentals - semitones + 2,
            },
            E => Pitch {
                name: D,
                accidentals: self.accidentals - semitones + 2,
            },
            F => Pitch {
                name: E,
                accidentals: self.accidentals - semitones + 1,
            },
            G => Pitch {
                name: F,
                accidentals: self.accidentals - semitones + 2,
            },
            A => Pitch {
                name: G,
                accidentals: self.accidentals - semitones + 2,
            },
            B => Pitch {
                name: A,
                accidentals: self.accidentals - semitones + 2,
            },
        }
    }

    pub fn leap(&self, interval: Interval) -> Pitch {
        let mut pitch = (0..interval.diatonic_steps()).fold(*self, |p, _| p.next(0));
        pitch.accidentals += interval.size() as i32;
        pitch
    }

    pub fn fall(&self, interval: Interval) -> Pitch {
        let mut pitch = (0..interval.diatonic_steps()).fold(*self, |p, _| p.prev(0));
        pitch.accidentals -= interval.size() as i32;
        pitch
    }
}

impl Note {
    /// Returns the next note name. Accidentals are updated such that the new note is
    /// `semitones` away from `self`.
    pub fn next(&self, semitones: i32) -> Note {
        use NoteName::*;
        match self.pitch.name {
            C => Note {
                pitch: self.pitch.next(semitones),
                octave: self.octave,
            },
            D => Note {
                pitch: self.pitch.next(semitones),
                octave: self.octave,
            },
            E => Note {
                pitch: self.pitch.next(semitones),
                octave: self.octave,
            },
            F => Note {
                pitch: self.pitch.next(semitones),
                octave: self.octave,
            },
            G => Note {
                pitch: self.pitch.next(semitones),
                octave: self.octave,
            },
            A => Note {
                pitch: self.pitch.next(semitones),
                octave: self.octave,
            },
            B => Note {
                pitch: self.pitch.next(semitones),
                octave: self.octave + 1,
            },
        }
    }

    pub fn prev(&self, semitones: i32) -> Note {
        use NoteName::*;

        match self.pitch.name {
            C => Note {
                pitch: self.pitch.prev(semitones),
                octave: self.octave - 1,
            },
            D => Note {
                pitch: self.pitch.prev(semitones),
                octave: self.octave,
            },
            E => Note {
                pitch: self.pitch.prev(semitones),
                octave: self.octave,
            },
            F => Note {
                pitch: self.pitch.prev(semitones),
                octave: self.octave,
            },
            G => Note {
                pitch: self.pitch.prev(semitones),
                octave: self.octave,
            },
            A => Note {
                pitch: self.pitch.prev(semitones),
                octave: self.octave,
            },
            B => Note {
                pitch: self.pitch.prev(semitones),
                octave: self.octave,
            },
        }
    }

    pub fn leap(&self, interval: Interval) -> Note {
        let mut note = (0..interval.diatonic_steps()).fold(*self, |n, _| n.next(0));
        note.pitch.accidentals += interval.size() as i32;
        note
    }

    pub fn fall(&self, interval: Interval) -> Note {
        let mut note = (0..interval.diatonic_steps()).fold(*self, |n, _| n.prev(0));
        note.pitch.accidentals -= interval.size() as i32;
        note
    }

    fn accidental_to_string(accidental: Accidentals) -> String {
        let double_logic = |n: i32, double: &str, single: &str| {
            double.to_string().repeat((n / 2).unsigned_abs() as usize)
                + single
                    .to_string()
                    .repeat((n.unsigned_abs() % 2) as usize)
                    .as_str()
        };
        match accidental {
            n if n < 0 => double_logic(n, "𝄫", "♭"),
            n => double_logic(n, "𝄪", "♯"),
        }
    }

    fn accidental_to_dutch_notation(accidental: Accidentals) -> String {
        match accidental {
            n if n < 0 => "es".to_string().repeat(n.unsigned_abs() as usize),
            n => "is".to_string().repeat(n as usize),
        }
    }
}

pub fn octave(pitch: Pitch, octave: u32) -> Note {
    Note { pitch, octave }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.pitch, self.octave)
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let note_name = match self.name {
            NoteName::C => "C",
            NoteName::D => "D",
            NoteName::E => "E",
            NoteName::F => "F",
            NoteName::G => "G",
            NoteName::A => "A",
            NoteName::B => "B",
        };

        let accidental_str = Note::accidental_to_string(self.accidentals);
        write!(f, "{}{}", note_name, accidental_str)
    }
}

pub trait FormatAsCode {
    fn fmt_as_code(&self) -> String;
}

impl FormatAsCode for Note {
    fn fmt_as_code(&self) -> String {
        let note_name = match self.pitch.name {
            NoteName::C => "c",
            NoteName::D => "d",
            NoteName::E => "e",
            NoteName::F => "f",
            NoteName::G => "g",
            NoteName::A => "a",
            NoteName::B => "b",
        };

        let accidental_str = Note::accidental_to_dutch_notation(self.pitch.accidentals);
        format!("{}{}!({})", note_name, accidental_str, self.octave)
    }
}

impl FormatAsCode for Pitch {
    fn fmt_as_code(&self) -> String {
        let note_name = match self.name {
            NoteName::C => "c",
            NoteName::D => "d",
            NoteName::E => "e",
            NoteName::F => "f",
            NoteName::G => "g",
            NoteName::A => "a",
            NoteName::B => "b",
        };

        let accidental_str = Note::accidental_to_dutch_notation(self.accidentals);
        format!("{}{}!()", note_name, accidental_str)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Notes(pub Vec<Note>);

impl fmt::Display for Notes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, note) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", note)?;
        }
        Ok(())
    }
}

impl FormatAsCode for Notes {
    fn fmt_as_code(&self) -> String {
        let Notes(notes) = self;
        format!(
            "[{}]",
            notes
                .iter()
                .map(|note| note.fmt_as_code())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl IntoIterator for Notes {
    type Item = Note;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub fn pitches(notes: &Notes) -> HashSet<Pitch> {
    notes.clone().into_iter().map(|n| n.pitch).collect()
}

pub fn pretty_pitches(pitches: &HashSet<Pitch>) -> String {
    let mut vec: Vec<&Pitch> = pitches.iter().collect();
    vec.sort();
    let vec: Vec<String> = vec.iter().map(|p| format!("{}", p)).collect();
    vec.join(", ")
}

#[macro_export]
macro_rules! c {
    () => {
        $crate::notes::Pitch {
            name: $crate::notes::NoteName::C,
            accidentals: 0,
        }
    };
}

#[macro_export]
macro_rules! cis {
    () => {
        $crate::notes::Pitch {
            name: $crate::notes::NoteName::C,
            accidentals: 1,
        }
    };
}

#[macro_export]
macro_rules! ces {
    () => {
        $crate::notes::Pitch {
            name: $crate::notes::NoteName::C,
            accidentals: -1,
        }
    };
}

#[macro_export]
macro_rules! d {
    () => {
        $crate::notes::Pitch {
            name: $crate::notes::NoteName::D,
            accidentals: 0,
        }
    };
}

#[macro_export]
macro_rules! dis {
    () => {
        $crate::notes::Pitch {
            name: $crate::notes::NoteName::D,
            accidentals: 1,
        }
    };
}

#[macro_export]
macro_rules! des {
    () => {
        $crate::notes::Pitch {
            name: $crate::notes::NoteName::D,
            accidentals: -1,
        }
    };
}

#[macro_export]
macro_rules! e {
    () => {
        $crate::notes::Pitch {
            name: $crate::notes::NoteName::E,
            accidentals: 0,
        }
    };
}

#[macro_export]
macro_rules! eis {
    () => {
        $crate::notes::Pitch {
            name: $crate::notes::NoteName::E,
            accidentals: 1,
        }
    };
}

#[macro_export]
macro_rules! ees {
    () => {
        $crate::notes::Pitch {
            name: $crate::notes::NoteName::E,
            accidentals: -1,
        }
    };
}

#[macro_export]
macro_rules! f {
    () => {
        $crate::notes::Pitch {
            name: $crate::notes::NoteName::F,
            accidentals: 0,
        }
    };
}

#[macro_export]
macro_rules! fis {
    () => {
        $crate::notes::Pitch {
            name: $crate::notes::NoteName::F,
            accidentals: 1,
        }
    };
}

#[macro_export]
macro_rules! fes {
    () => {
        $crate::notes::Pitch {
            name: $crate::notes::NoteName::F,
            accidentals: -1,
        }
    };
}

#[macro_export]
macro_rules! g {
    () => {
        $crate::notes::Pitch {
            name: $crate::notes::NoteName::G,
            accidentals: 0,
        }
    };
}

#[macro_export]
macro_rules! gis {
    () => {
        $crate::notes::Pitch {
            name: $crate::notes::NoteName::G,
            accidentals: 1,
        }
    };
}

#[macro_export]
macro_rules! ges {
    () => {
        $crate::notes::Pitch {
            name: $crate::notes::NoteName::G,
            accidentals: -1,
        }
    };
}

#[macro_export]
macro_rules! a {
    () => {
        $crate::notes::Pitch {
            name: $crate::notes::NoteName::A,
            accidentals: 0,
        }
    };
}

#[macro_export]
macro_rules! ais {
    () => {
        $crate::notes::Pitch {
            name: $crate::notes::NoteName::A,
            accidentals: 1,
        }
    };
}

#[macro_export]
macro_rules! aes {
    () => {
        $crate::notes::Pitch {
            name: $crate::notes::NoteName::A,
            accidentals: -1,
        }
    };
}

#[macro_export]
macro_rules! b {
    () => {
        $crate::notes::Pitch {
            name: $crate::notes::NoteName::B,
            accidentals: 0,
        }
    };
}

#[macro_export]
macro_rules! bis {
    () => {
        $crate::notes::Pitch {
            name: $crate::notes::NoteName::B,
            accidentals: 1,
        }
    };
}

#[macro_export]
macro_rules! bes {
    () => {
        $crate::notes::Pitch {
            name: $crate::notes::NoteName::B,
            accidentals: -1,
        }
    };
}

#[macro_export]
macro_rules! note_c {
    ($octave:expr) => {
        $crate::notes::Note {
            pitch: $crate::notes::Pitch {
                name: $crate::notes::NoteName::C,
                accidentals: 0,
            },
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! note_cis {
    ($octave:expr) => {
        $crate::notes::Note {
            name: $crate::notes::NoteName::C,
            accidentals: 1,
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! note_ces {
    ($octave:expr) => {
        $crate::notes::Note {
            pitch: $crate::notes::Pitch {
                name: $crate::notes::NoteName::C,
                accidentals: -1,
            },
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! note_d {
    ($octave:expr) => {
        $crate::notes::Note {
            pitch: $crate::notes::Pitch {
                name: $crate::notes::NoteName::D,
                accidentals: 0,
            },
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! note_dis {
    ($octave:expr) => {
        $crate::notes::Note {
            pitch: $crate::notes::Pitch {
                name: $crate::notes::NoteName::D,
                accidentals: 1,
            },
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! note_des {
    ($octave:expr) => {
        $crate::notes::Note {
            pitch: $crate::notes::Pitch {
                name: $crate::notes::NoteName::D,
                accidentals: -1,
            },
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! note_e {
    ($octave:expr) => {
        $crate::notes::Note {
            pitch: $crate::notes::Pitch {
                name: $crate::notes::NoteName::E,
                accidentals: 0,
            },
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! note_eis {
    ($octave:expr) => {
        $crate::notes::Note {
            pitch: $crate::notes::Pitch {
                name: $crate::notes::NoteName::E,
                accidentals: 1,
            },
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! note_ees {
    ($octave:expr) => {
        $crate::notes::Note {
            pitch: $crate::notes::Pitch {
                name: $crate::notes::NoteName::E,
                accidentals: -1,
            },
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! note_f {
    ($octave:expr) => {
        $crate::notes::Note {
            pitch: $crate::notes::Pitch {
                name: $crate::notes::NoteName::F,
                accidentals: 0,
            },
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! note_fis {
    ($octave:expr) => {
        $crate::notes::Note {
            pitch: $crate::notes::Pitch {
                name: $crate::notes::NoteName::F,
                accidentals: 1,
            },
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! note_fes {
    ($octave:expr) => {
        $crate::notes::Note {
            pitch: $crate::notes::Pitch {
                name: $crate::notes::NoteName::F,
                accidentals: -1,
            },
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! note_g {
    ($octave:expr) => {
        $crate::notes::Note {
            pitch: $crate::notes::Pitch {
                name: $crate::notes::NoteName::G,
                accidentals: 0,
            },
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! note_gis {
    ($octave:expr) => {
        $crate::notes::Note {
            pitch: $crate::notes::Pitch {
                name: $crate::notes::NoteName::G,
                accidentals: 1,
            },
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! note_ges {
    ($octave:expr) => {
        $crate::notes::Note {
            pitch: $crate::notes::Pitch {
                name: $crate::notes::NoteName::G,
                accidentals: -1,
            },
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! note_a {
    ($octave:expr) => {
        $crate::notes::Note {
            pitch: $crate::notes::Pitch {
                name: $crate::notes::NoteName::A,
                accidentals: 0,
            },
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! note_ais {
    ($octave:expr) => {
        $crate::notes::Note {
            pitch: $crate::notes::Pitch {
                name: $crate::notes::NoteName::A,
                accidentals: 1,
            },
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! note_aes {
    ($octave:expr) => {
        $crate::notes::Note {
            pitch: $crate::notes::Pitch {
                name: $crate::notes::NoteName::A,
                accidentals: -1,
            },
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! note_b {
    ($octave:expr) => {
        $crate::notes::Note {
            pitch: $crate::notes::Pitch {
                name: $crate::notes::NoteName::B,
                accidentals: 0,
            },
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! note_bis {
    ($octave:expr) => {
        $crate::notes::Note {
            pitch: $crate::notes::Pitch {
                name: $crate::notes::NoteName::B,
                accidentals: 1,
            },
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! note_bes {
    ($octave:expr) => {
        $crate::notes::Note {
            pitch: $crate::notes::Pitch {
                name: $crate::notes::NoteName::B,
                accidentals: -1,
            },
            octave: $octave,
        }
    };
}
