use core::fmt;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use crate::color::Argb;
use crate::color_variant::content::SchemeContent;
use crate::color_variant::expressive::SchemeExpressive;
use crate::color_variant::fidelity::SchemeFidelity;
use crate::color_variant::fruit_salad::SchemeFruitSalad;
use crate::color_variant::monochrome::SchemeMonochrome;
use crate::color_variant::neutral::SchemeNeutral;
use crate::color_variant::rainbow::SchemeRainbow;
use crate::color_variant::tonal_spot::SchemeTonalSpot;
use crate::color_variant::vibrant::SchemeVibrant;
use crate::htc::Hct;
use crate::{Map, Palette};
use crate::theme::ColorVariantMode;

/// A convenience class for retrieving colors that are constant in hue and
/// chroma, but vary in tone.
#[derive(Clone, Copy, Debug, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct TonalPalette {
    _hue: f64,
    _chroma: f64,
    _key_color: Hct,
}

impl TonalPalette {
    /// Commonly-used tone values.
    const COMMON_TONES: [i32; 13] = [0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 95, 99, 100];

    pub const fn common_size() -> usize {
        Self::COMMON_TONES.len()
    }

    pub const fn hue(&self) -> f64 {
        self._hue
    }

    pub const fn chroma(&self) -> f64 {
        self._chroma
    }

    pub const fn key_color(&self) -> Hct {
        self._key_color
    }

    const fn new(_hue: f64, _chroma: f64, _key_color: Hct) -> Self {
        Self {
            _hue,
            _chroma,
            _key_color,
        }
    }

    /// Create a Tonal Palette from hue and chroma of `hct`.
    pub const fn from_hct(hct: Hct) -> Self {
        Self::new(hct.get_hue(), hct.get_chroma(), hct)
    }

    pub fn by_variant(source_hct: &Hct, scheme: &ColorVariantMode, variant: &Palette) -> Self {
        match scheme {
            ColorVariantMode::Monochrome => SchemeMonochrome::palette(source_hct, variant),
            ColorVariantMode::Neutral => SchemeNeutral::palette(source_hct, variant),
            ColorVariantMode::TonalSpot => SchemeTonalSpot::palette(source_hct, variant),
            ColorVariantMode::Vibrant => SchemeVibrant::palette(source_hct, variant),
            ColorVariantMode::Expressive => SchemeExpressive::palette(source_hct, variant),
            ColorVariantMode::Fidelity => SchemeFidelity::palette(source_hct, variant),
            ColorVariantMode::Content => SchemeContent::palette(source_hct, variant),
            ColorVariantMode::Rainbow => SchemeRainbow::palette(source_hct, variant),
            ColorVariantMode::FruitSalad => SchemeFruitSalad::palette(source_hct, variant),
        }
    }

    /// Create a Tonal Palette from `hue` and `chroma`, which generates a key color.
    pub fn from_hue_and_chroma(hue: f64, chroma: f64) -> Self {
        Self::new(hue, chroma, KeyColor::new(hue, chroma).create())
    }

    /// Create colors using `hue` and `chroma`.
    pub fn of(hue: f64, chroma: f64) -> Self {
        Self::from_hue_and_chroma(hue, chroma)
    }

    /// Returns the Argb representation of an HCT color.
    ///
    /// If the class was instantiated from `_hue` and `_chroma`, will return the
    /// color with corresponding `tone`.
    /// If the class was instantiated from a fixed-size list of color ints, `tone`
    /// must be in `common_mones`.
    pub fn tone(&self, tone: i32) -> Argb {
        Hct::from(self.hue(), self.chroma(), f64::from(tone)).into()
    }

    pub fn get_hct(&self, tone: f64) -> Hct {
        Hct::from(self.hue(), self.chroma(), tone)
    }
}

impl Ord for TonalPalette {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for TonalPalette {
    fn eq(&self, other: &Self) -> bool {
        self._hue == other._hue && self._chroma == other._chroma
    }
}

impl Eq for TonalPalette {}

impl Hash for TonalPalette {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self._hue.to_bits().hash(state);
        self._chroma.to_bits().hash(state);
        self._key_color.hash(state);
    }
}

impl fmt::Display for TonalPalette {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TonalPalette.of({}, {})", self.hue(), self.chroma())
    }
}

/// Key color is a color that represents the hue and chroma of a tonal palette
pub struct KeyColor {
    hue: f64,
    requested_chroma: f64,
    /// Cache that maps tone to max chroma to avoid duplicated HCT calculation.
    chroma_cache: Map<i32, f64>,
}

impl KeyColor {
    const MAX_CHROMA_VALUE: f64 = 200.0;

    pub fn new(hue: f64, requested_chroma: f64) -> Self {
        Self {
            hue,
            requested_chroma,
            chroma_cache: Map::default(),
        }
    }

    /// Creates a key color from a [`hue`] and a [`chroma`].
    /// The key color is the first tone, starting from T50, matching the given hue
    /// and chroma.
    ///
    /// Returns key color in [`Hct`].
    pub fn create(&mut self) -> Hct {
        // Pivot around T50 because T50 has the most chroma available, on average. Thus it is most
        // likely to have a direct answer.
        let pivot_tone = 50;
        let tone_step_size = 1;
        // Epsilon to accept values slightly higher than the requested chroma.
        let epsilon = 0.01;

        // Binary search to find the tone that can provide a chroma that is closest
        // to the requested chroma.
        let mut lower_tone = 0;
        let mut upper_tone = 100;

        while lower_tone < upper_tone {
            let mid_tone = (lower_tone + upper_tone) / 2;
            let is_ascending =
                self.max_chroma(mid_tone) < self.max_chroma(mid_tone + tone_step_size);
            let sufficient_chroma = self.max_chroma(mid_tone) >= self.requested_chroma - epsilon;

            if sufficient_chroma {
                // Either range [lowerTone, midTone] or [midTone, upperTone] has answer, so search in the
                // range that is closer the pivot tone.
                if (lower_tone - pivot_tone).abs() < (upper_tone - pivot_tone).abs() {
                    upper_tone = mid_tone;
                } else if lower_tone == mid_tone {
                    return Hct::from(self.hue, self.requested_chroma, f64::from(lower_tone));
                } else {
                    lower_tone = mid_tone;
                }
            } else if is_ascending {
                // As there is no sufficient chroma in the midTone, follow the direction to the chroma
                // peak.
                lower_tone = mid_tone + tone_step_size;
            } else {
                // Keep midTone for potential chroma peak.
                upper_tone = mid_tone;
            }
        }

        Hct::from(self.hue, self.requested_chroma, f64::from(lower_tone))
    }

    fn max_chroma(&mut self, tone: i32) -> f64 {
        if let Some(chroma) = self.chroma_cache.get(&tone) {
            *chroma
        } else {
            let chroma = Hct::from(self.hue, Self::MAX_CHROMA_VALUE, f64::from(tone)).get_chroma();

            self.chroma_cache.insert(tone, chroma);

            chroma
        }
    }
}