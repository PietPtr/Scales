use crate::notes::Note;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Qualities {
    Major,
    Minor,
    Diminshed,
    Sus4,
    Sus2,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tensions {
    Seventh,
    Ninth,
    Eleventh,
    Thirteenth,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MajorMode {
    Lydian,
    Ionian,
    Mixolydian,
    Dorian,
    Aeolian,
    Phrygian,
    Locrian,
}

pub trait Scale {
    fn spell(&self) -> Vec<Note>;
    fn root(&self) -> Note;
}

pub struct Major {
    pub root: Note,
    pub mode: MajorMode,
}

impl Scale for Major {
    fn spell(&self) -> Vec<Note> {
        todo!()
    }

    fn root(&self) -> Note {
        self.root
    }
}
