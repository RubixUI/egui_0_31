use crate::{dynamic_schema::{DynamicScheme}, htc::Hct, Palette};
use crate::theme::ColorVariantMode;
use crate::tonal_palette::TonalPalette;

pub struct SchemeNeutral {
    pub scheme: DynamicScheme,
}

impl SchemeNeutral {
    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: Option<f64>) -> Self {
        Self {
            scheme: DynamicScheme::new(
                source_color_hct,
                ColorVariantMode::Neutral,
                is_dark,
                contrast_level,
                Self::palette(&source_color_hct, &Palette::Primary),
                Self::palette(&source_color_hct, &Palette::Secondary),
                Self::palette(&source_color_hct, &Palette::Tertiary),
                Self::palette(&source_color_hct, &Palette::Neutral),
                Self::palette(&source_color_hct, &Palette::NeutralVariant),
                None,
            ),
        }
    }

    pub fn palette(source_color_hct: &Hct, variant: &Palette) -> TonalPalette {
        match variant {
            Palette::Primary => TonalPalette::of(source_color_hct.get_hue(), 12.0),
            Palette::Secondary => TonalPalette::of(source_color_hct.get_hue(), 8.0),
            Palette::Tertiary => TonalPalette::of(source_color_hct.get_hue(), 16.0),
            Palette::Error => TonalPalette::of(25.0, 84.0),
            Palette::Neutral | Palette::NeutralVariant => {
                TonalPalette::of(source_color_hct.get_hue(), 2.0)
            }
        }
    }
}