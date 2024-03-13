use std::collections::HashSet;

use crate::{
    intervals::Interval,
    notes::{self, Note, Notes},
};

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
    fn intervals(&self) -> HashSet<Interval>;
    fn root(&self) -> Note;
}

pub struct Major {
    pub root: Note,
}

impl Scale for Major {
    fn intervals(&self) -> HashSet<Interval> {
        let mut intervals = HashSet::new();
        intervals.insert(Interval::Unison);
        intervals.insert(Interval::MajorSecond);
        intervals.insert(Interval::MajorThird);
        intervals.insert(Interval::PerfectFourth);
        intervals.insert(Interval::PerfectFifth);
        intervals.insert(Interval::MajorSixth);
        intervals.insert(Interval::MajorSeventh);
        intervals
    }

    fn root(&self) -> Note {
        self.root
    }
}

pub fn spell<S: Scale>(scale: S) -> Notes {
    let mut notes = vec![];

    let mut intervals: Vec<Interval> = scale.intervals().iter().cloned().collect();
    intervals.sort();

    for interval in intervals {
        notes.push(scale.root().leap(interval));
    }

    Notes(notes)
}
