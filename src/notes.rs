use std::fmt;

use crate::intervals::Interval;

pub type Accidentals = i32;
pub const NATURAL: i32 = 0;
pub const SHARP: i32 = 1;
pub const DOUBLE_SHARP: i32 = 2;
pub const FLAT: i32 = -1;
pub const DOUBLE_FLAT: i32 = -2;

pub type Octave = u32;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NoteName {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Note {
    pub name: NoteName,
    pub accidentals: Accidentals,
    pub octave: Octave,
}

impl Note {
    /// Returns the next note name. Accidentals are updated such that the new note is
    /// `semitones` away from `self`.
    pub fn next(&self, semitones: i32) -> Note {
        use NoteName::*;

        match self.name {
            C => Note {
                name: D,
                accidentals: self.accidentals + semitones - 2,
                octave: self.octave,
            },
            D => Note {
                name: E,
                accidentals: self.accidentals + semitones - 2,
                octave: self.octave,
            },
            E => Note {
                name: F,
                accidentals: self.accidentals + semitones - 1,
                octave: self.octave,
            },
            F => Note {
                name: G,
                accidentals: self.accidentals + semitones - 2,
                octave: self.octave,
            },
            G => Note {
                name: A,
                accidentals: self.accidentals + semitones - 2,
                octave: self.octave,
            },
            A => Note {
                name: B,
                accidentals: self.accidentals + semitones - 2,
                octave: self.octave,
            },
            B => Note {
                name: C,
                accidentals: self.accidentals + semitones - 1,
                octave: self.octave + 1,
            },
        }
    }

    pub fn prev(&self, semitones: i32) -> Note {
        use NoteName::*;

        match self.name {
            C => Note {
                name: B,
                accidentals: self.accidentals - semitones + 1,
                octave: self.octave - 1,
            },
            D => Note {
                name: C,
                accidentals: self.accidentals - semitones + 2,
                octave: self.octave,
            },
            E => Note {
                name: D,
                accidentals: self.accidentals - semitones + 2,
                octave: self.octave,
            },
            F => Note {
                name: E,
                accidentals: self.accidentals - semitones + 1,
                octave: self.octave,
            },
            G => Note {
                name: F,
                accidentals: self.accidentals - semitones + 2,
                octave: self.octave,
            },
            A => Note {
                name: G,
                accidentals: self.accidentals - semitones + 2,
                octave: self.octave,
            },
            B => Note {
                name: A,
                accidentals: self.accidentals - semitones + 2,
                octave: self.octave,
            },
        }
    }

    pub fn leap(&self, interval: Interval) -> Note {
        let mut note = (0..interval.diatonic_steps()).fold(*self, |n, _| n.next(0));
        note.accidentals += interval.size() as i32;
        note
    }

    pub fn fall(&self, interval: Interval) -> Note {
        let mut note = (0..interval.diatonic_steps()).fold(*self, |n, _| n.prev(0));
        note.accidentals -= interval.size() as i32;
        note
    }

    fn accidental_to_string(accidental: Accidentals) -> String {
        match accidental {
            SHARP => "♯".to_string(),
            DOUBLE_SHARP => "𝄪".to_string(),
            FLAT => "♭".to_string(),
            DOUBLE_FLAT => "𝄫".to_string(),
            n if n < 0 => "♭".to_string().repeat(n.unsigned_abs() as usize),
            n => "♯".to_string().repeat(n as usize),
        }
    }
}

impl fmt::Display for Note {
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
        write!(f, "{}{}{}", note_name, accidental_str, self.octave)
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

#[macro_export]
macro_rules! c {
    ($octave:expr) => {
        $crate::notes::Note {
            name: $crate::notes::NoteName::C,
            accidentals: 0,
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! cis {
    ($octave:expr) => {
        $crate::notes::Note {
            name: $crate::notes::NoteName::C,
            accidentals: 1,
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! ces {
    ($octave:expr) => {
        $crate::notes::Note {
            name: $crate::notes::NoteName::C,
            accidentals: -1,
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! d {
    ($octave:expr) => {
        $crate::notes::Note {
            name: $crate::notes::NoteName::D,
            accidentals: 0,
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! dis {
    ($octave:expr) => {
        $crate::notes::Note {
            name: $crate::notes::NoteName::D,
            accidentals: 1,
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! des {
    ($octave:expr) => {
        $crate::notes::Note {
            name: $crate::notes::NoteName::D,
            accidentals: -1,
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! e {
    ($octave:expr) => {
        $crate::notes::Note {
            name: $crate::notes::NoteName::E,
            accidentals: 0,
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! eis {
    ($octave:expr) => {
        $crate::notes::Note {
            name: $crate::notes::NoteName::E,
            accidentals: 1,
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! es {
    ($octave:expr) => {
        $crate::notes::Note {
            name: $crate::notes::NoteName::E,
            accidentals: -1,
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! f {
    ($octave:expr) => {
        $crate::notes::Note {
            name: $crate::notes::NoteName::F,
            accidentals: 0,
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! fis {
    ($octave:expr) => {
        $crate::notes::Note {
            name: $crate::notes::NoteName::F,
            accidentals: 1,
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! fes {
    ($octave:expr) => {
        $crate::notes::Note {
            name: $crate::notes::NoteName::F,
            accidentals: -1,
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! g {
    ($octave:expr) => {
        $crate::notes::Note {
            name: $crate::notes::NoteName::G,
            accidentals: 0,
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! gis {
    ($octave:expr) => {
        $crate::notes::Note {
            name: $crate::notes::NoteName::G,
            accidentals: 1,
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! ges {
    ($octave:expr) => {
        $crate::notes::Note {
            name: $crate::notes::NoteName::G,
            accidentals: -1,
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! a {
    ($octave:expr) => {
        $crate::notes::Note {
            name: $crate::notes::NoteName::A,
            accidentals: 0,
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! ais {
    ($octave:expr) => {
        $crate::notes::Note {
            name: $crate::notes::NoteName::A,
            accidentals: 1,
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! aes {
    ($octave:expr) => {
        $crate::notes::Note {
            name: $crate::notes::NoteName::A,
            accidentals: -1,
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! b {
    ($octave:expr) => {
        $crate::notes::Note {
            name: $crate::notes::NoteName::B,
            accidentals: 0,
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! bis {
    ($octave:expr) => {
        $crate::notes::Note {
            name: $crate::notes::NoteName::B,
            accidentals: 1,
            octave: $octave,
        }
    };
}

#[macro_export]
macro_rules! bes {
    ($octave:expr) => {
        $crate::notes::Note {
            name: $crate::notes::NoteName::B,
            accidentals: -1,
            octave: $octave,
        }
    };
}
