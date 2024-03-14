use std::collections::HashSet;

use crate::{
    intervals::{ImperfectInterval, Interval, PerfectInterval},
    notes::{Note, Notes},
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
        use ImperfectInterval::*;
        use Interval::*;
        use PerfectInterval::*;
        let mut intervals = HashSet::new();
        intervals.insert(Perfect(Unison));
        intervals.insert(Major(Second));
        intervals.insert(Major(Third));
        intervals.insert(Perfect(Fourth));
        intervals.insert(Perfect(Fifth));
        intervals.insert(Major(Sixth));
        intervals.insert(Major(Seventh));
        intervals
    }

    fn root(&self) -> Note {
        self.root
    }
}

pub fn spell<S: Scale>(scale: S) -> Notes {
    let mut notes = vec![];

    let mut intervals: Vec<Interval> = scale.intervals().iter().cloned().collect();
    intervals.sort_by_key(|a| a.size());

    for interval in intervals {
        notes.push(scale.root().leap(interval));
    }

    Notes(notes)
}
