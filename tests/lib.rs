#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use scales::{
        a, aes, ais, b, bes, bis, c, ces, cis, d, des, dis, e, ees, eis, f, fes, fis, g, ges, gis,
        intervals::{ImperfectInterval::*, Interval::*, PerfectInterval::*},
        note_a, note_b, note_c, note_ces, note_d, note_dis, note_e, note_ees, note_f, note_g,
        notes::{octave, pitches, pretty_pitches},
        scales::{spell, Aeolian, Dorian, Ionian, Locrian, Lydian, Major, Mixolydian, Phrygian},
    };

    #[test]
    fn test_note_steps() {
        let note = octave(c!(), 4);
        assert_eq!(note.next(2), octave(d!(), 4));
        assert_eq!(note.prev(1), octave(b!(), 3));
    }

    #[test]
    fn test_diatonic() {
        let pitch = c!();

        let c = pitch.leap(Perfect(Unison));
        assert_eq!(c, c!());
        let d = pitch.leap(Major(Second));
        assert_eq!(d, d!());
        let e = pitch.leap(Major(Third));
        assert_eq!(e, e!());
        let f = pitch.leap(Perfect(Fourth));
        assert_eq!(f, f!());
        let g = pitch.leap(Perfect(Fifth));
        assert_eq!(g, g!());
        let a = pitch.leap(Major(Sixth));
        assert_eq!(a, a!());
        let b = pitch.leap(Major(Seventh));
        assert_eq!(b, b!());

        let c = pitch.fall(Perfect(Unison));
        assert_eq!(c, c!());
        let b = pitch.fall(Minor(Second));
        assert_eq!(b, b!());
        let a = pitch.fall(Minor(Third));
        assert_eq!(a, a!());
        let g = pitch.fall(Perfect(Fourth));
        assert_eq!(g, g!());
        let f = pitch.fall(Perfect(Fifth));
        assert_eq!(f, f!());
        let e = pitch.fall(Minor(Sixth));
        assert_eq!(e, e!());
        let d = pitch.fall(Minor(Seventh));
        assert_eq!(d, d!());
    }

    #[test]
    fn test_spell() {
        assert_eq!(
            pitches(&spell(Major { root: note_c!(4) })),
            HashSet::from_iter(vec![c!(), d!(), e!(), f!(), g!(), a!(), b!()])
        );

        println!("D# Major: {}", spell(Major { root: note_dis!(4) }));

        assert_eq!(
            pitches(&spell(Major { root: note_ees!(4) })),
            HashSet::from_iter(vec![ees!(), f!(), g!(), aes!(), bes!(), c!(), d!()])
        );
    }

    #[test]
    fn test_circle_of_fifths() {
        let all_keys = [
            [ges!(), aes!(), bes!(), ces!(), des!(), ees!(), fes!()],
            [ges!(), aes!(), bes!(), ces!(), des!(), ees!(), f!()],
            [des!(), ees!(), f!(), ges!(), aes!(), bes!(), c!()],
            [aes!(), bes!(), c!(), des!(), ees!(), f!(), g!()],
            [ees!(), f!(), g!(), aes!(), bes!(), c!(), d!()],
            [bes!(), c!(), d!(), ees!(), f!(), g!(), a!()],
            [f!(), g!(), a!(), bes!(), c!(), d!(), e!()],
            [c!(), d!(), e!(), f!(), g!(), a!(), b!()],
            [g!(), a!(), b!(), c!(), d!(), e!(), fis!()],
            [d!(), e!(), fis!(), g!(), a!(), b!(), cis!()],
            [a!(), b!(), cis!(), d!(), e!(), fis!(), gis!()],
            [e!(), fis!(), gis!(), a!(), b!(), cis!(), dis!()],
            [b!(), cis!(), dis!(), e!(), fis!(), gis!(), ais!()],
            [fis!(), gis!(), ais!(), b!(), cis!(), dis!(), eis!()],
            [fis!(), gis!(), ais!(), bis!(), cis!(), dis!(), eis!()],
        ];
        let mut root = note_ces!(4);

        for key in all_keys {
            let pitches = pitches(&spell(Major { root }));
            println!("{} Major: {}", root.pitch, pretty_pitches(&pitches));
            assert_eq!(HashSet::from_iter(key), pitches);

            root = root.leap(Perfect(Fifth));
        }
    }

    #[test]
    fn test_modes() {
        let c_pitches: HashSet<_> = [c!(), d!(), e!(), f!(), g!(), a!(), b!()]
            .iter()
            .cloned()
            .collect();

        let modes = [
            spell(Lydian { root: note_f!(4) }),
            spell(Ionian { root: note_c!(4) }),
            spell(Mixolydian { root: note_g!(4) }),
            spell(Dorian { root: note_d!(4) }),
            spell(Aeolian { root: note_a!(4) }),
            spell(Phrygian { root: note_e!(4) }),
            spell(Locrian { root: note_b!(4) }),
        ];

        for mode in modes {
            assert_eq!(pitches(&mode), c_pitches);
        }
    }
}
