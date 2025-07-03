use crate::dynamic_schema::DynamicScheme;
use crate::htc::Hct;
use crate::Palette;
use crate::theme::ColorVariantMode;
use crate::tonal_palette::TonalPalette;

pub struct SchemeMonochrome {
    pub scheme: DynamicScheme,
}

impl SchemeMonochrome {
    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: Option<f64>) -> Self {
        Self {
            scheme: DynamicScheme::new(
                source_color_hct,
                ColorVariantMode::Monochrome,
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
            Palette::Primary
            | Palette::Secondary
            | Palette::Tertiary
            | Palette::Neutral
            | Palette::NeutralVariant => TonalPalette::of(source_color_hct.get_hue(), 0.0),
            Palette::Error => TonalPalette::of(25.0, 84.0),
        }
    }
}