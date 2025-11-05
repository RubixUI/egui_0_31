use core::fmt;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
use epaint::{
    CornerRadius,Color32,
    text::{ FontDefinitions, FontData },
    FontFamily, FontId, Shadow, Stroke, Margin,
};
use crate::color_theme::{ColorTheme, CustomColorInput};

#[derive(Debug, Clone, PartialEq)]
pub struct StorkWidthTheme {
    /// RGBA 格式的背景色，取值范围 0.0 ~ 1.0
    pub stroke_width_thin: u8,
    pub stroke_width_thick: u8,
    pub stroke_width_thicker: u8,
    pub stroke_width_thickest: u8,
}

impl Default for StorkWidthTheme {
    fn default() -> Self {
        Self {
            stroke_width_thin: 1,
            stroke_width_thick: 2,
            stroke_width_thicker: 3,
            stroke_width_thickest: 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageSizeTheme {
    pub tiny: u8,
    pub mini: u8,
    pub small: u8,
    pub normal: u8,
    pub big: u8,
    pub large: u8,
    pub huge: u8,
}

impl Default for ImageSizeTheme {
    fn default() -> Self {
        Self {
            tiny: 12,
            mini: 16,
            small: 20,
            normal: 24,
            big: 28,
            large: 32,
            huge: 48,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShadowStyle {
    pub shadow2: Vec<Shadow>,
    pub shadow2_brand: Vec<Shadow>,
    pub shadow4: Vec<Shadow>,
    pub shadow4_brand: Vec<Shadow>,
    pub shadow8: Vec<Shadow>,
    pub shadow8_brand: Vec<Shadow>,
    pub shadow16: Vec<Shadow>,
    pub shadow16_brand: Vec<Shadow>,
    pub shadow28: Vec<Shadow>,
    pub shadow28_brand: Vec<Shadow>,
    pub shadow64: Vec<Shadow>,
    pub shadow64_brand: Vec<Shadow>,
}

// 定义Shadow主题结构体
#[derive(Debug, Clone)]
pub struct ShadowTheme {
    pub light: ShadowStyle,
    pub dark: ShadowStyle,
    pub high_contrast: ShadowStyle,
}

impl Default for ShadowTheme {
    fn default() -> Self {
        Self {
            light: ShadowStyle {
                // Light theme shadows
                shadow2: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 2,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 31), // rgba(0,0,0,0.12)
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 2,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 36), // rgba(0,0,0,0.14)
                    },
                ],
                shadow2_brand: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 2,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 120, 212, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 2,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 120, 212, 36),
                    },
                ],
                shadow4: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 4,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 4,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 36),
                    },
                ],
                shadow4_brand: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 4,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 120, 212, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 4,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 120, 212, 36),
                    },
                ],
                shadow8: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 8,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 8,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 36),
                    },
                ],
                shadow8_brand: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 8,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 120, 212, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 8,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 120, 212, 36),
                    },
                ],
                shadow16: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 16,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 16,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 36),
                    },
                ],
                shadow16_brand: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 16,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 120, 212, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 16,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 120, 212, 36),
                    },
                ],
                shadow28: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 28,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 28,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 36),
                    },
                ],
                shadow28_brand: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 28,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 120, 212, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 28,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 120, 212, 36),
                    },
                ],
                shadow64: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 64,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 64,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 36),
                    },
                ],
                shadow64_brand: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 64,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 120, 212, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 64,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 120, 212, 36),
                    },
                ],
            },
            dark: ShadowStyle {
                // Dark theme shadows
                shadow2: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 2,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 31), // rgba(0,0,0,0.12)
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 2,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 36), // rgba(0,0,0,0.14)
                    },
                ],
                shadow2_brand: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 2,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 2,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 36),
                    },
                ],
                shadow4: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 4,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 4,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 36),
                    },
                ],
                shadow4_brand: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 4,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 4,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 36),
                    },
                ],
                shadow8: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 8,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 8,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 36),
                    },
                ],
                shadow8_brand: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 8,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 8,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 36),
                    },
                ],
                shadow16: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 16,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 16,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 36),
                    },
                ],
                shadow16_brand: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 16,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 16,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 36),
                    },
                ],
                shadow28: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 28,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 28,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 36),
                    },
                ],
                shadow28_brand: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 28,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 28,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 36),
                    },
                ],
                shadow64: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 64,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 64,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 36),
                    },
                ],
                shadow64_brand: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 64,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 64,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 36),
                    },
                ],
            },
            high_contrast: ShadowStyle {
                // High Contrast theme shadows - 完全复制dark主题的定义
                shadow2: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 2,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 2,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 36),
                    },
                ],
                shadow2_brand: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 2,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 2,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 36),
                    },
                ],
                // ... 前面已定义的shadow2和shadow2_brand ...
                shadow4: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 4,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 4,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 36),
                    },
                ],
                shadow4_brand: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 4,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 4,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 36),
                    },
                ],
                shadow8: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 8,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 8,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 36),
                    },
                ],
                shadow8_brand: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 8,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 8,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 36),
                    },
                ],
                shadow16: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 16,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 16,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 36),
                    },
                ],
                shadow16_brand: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 16,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 16,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 36),
                    },
                ],
                shadow28: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 28,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 28,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 36),
                    },
                ],
                shadow28_brand: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 28,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 28,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 36),
                    },
                ],
                shadow64: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 64,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 64,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 36),
                    },
                ],
                shadow64_brand: vec![
                    Shadow {
                        offset: [0, 0],
                        blur: 64,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 31),
                    },
                    Shadow {
                        offset: [0, 1],
                        blur: 64,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(71, 158, 245, 36),
                    },
                ],
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrightnessMode {
    Dark,
    Light,
}

impl BrightnessMode {
    pub fn is_dark(&self) -> bool {
        *self == BrightnessMode::Dark
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CornerRadiusMode {
    Full,
    Large,
    Small,
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpaceMode {
    Loose,
    Compact
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd,Hash)]
pub enum ColorVariantMode {
    TonalSpot,
    Monochrome,
    Neutral,
    Vibrant,
    Expressive,
    Fidelity,
    Content,
    Rainbow,
    FruitSalad
}

impl fmt::Display for ColorVariantMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorVariantMode::TonalSpot => write!(f, "TonalSpot"),
            ColorVariantMode::Monochrome => write!(f, "Monochrome"),
            ColorVariantMode::Neutral => write!(f, "Neutral"),
            ColorVariantMode::Vibrant => write!(f, "Vibrant"),
            ColorVariantMode::Expressive => write!(f, "Expressive"),
            ColorVariantMode::Fidelity => write!(f, "Fidelity"),
            ColorVariantMode::Content => write!(f, "Content"),
            ColorVariantMode::Rainbow => write!(f, "Rainbow"),
            ColorVariantMode::FruitSalad => write!(f, "FruitSalad"),
        }
    }
}

#[derive(Debug, Clone,PartialEq)]
pub struct SizeVariant<T> {
    pub mini: T,
    pub extra_small: T,
    pub small: T,
    pub medium: T,
    pub large: T,
    pub extra_large: T,
    pub huge: T
}

impl SizeVariant<Margin> {
    pub fn mini_same(&self) -> Margin {
        Margin::same(self.mini.left)
    }

    pub fn extra_small_same(&self) -> Margin {
        Margin::same(self.extra_small.left)
    }

    pub fn small_same(&self) -> Margin {
        Margin::same(self.small.left)
    }

    pub fn medium_same(&self) -> Margin {
        Margin::same(self.medium.left)
    }

    pub fn large_same(&self) -> Margin {
        Margin::same(self.large.left)
    }

    pub fn extra_large_same(&self) -> Margin {
        Margin::same(self.extra_large.left)
    }

    pub fn huge_same(&self) -> Margin {
        Margin::same(self.huge.left)
    }
}

#[derive(Debug, Clone,PartialEq)]
pub struct ThemeGap {
    pub horizontal: SizeVariant<f32>,
    pub vertical: SizeVariant<f32>,
}

#[derive(Debug, Clone,PartialEq)]
pub struct ThemeMargin {
    horizontal: SizeVariant<f32>,
    vertical: SizeVariant<f32>,
}

#[derive(Debug, Clone,PartialEq)]
pub struct Theme {
    pub source_color: Color32,
    pub brightness_mode: BrightnessMode,
    pub color_variant_mode: ColorVariantMode,
    pub corner_radius_mode: CornerRadiusMode,
    pub space_mode: SpaceMode,
    pub gap: ThemeGap,
    pub padding: SizeVariant<Margin>,
    pub corner_radius: SizeVariant<CornerRadius>,
    pub stork_width_variable: StorkWidthTheme,
    pub image_size_variable: ImageSizeTheme,
    pub color: ColorTheme,
}

//For padding
impl Theme {
    //For full size Corner Radius.we should extend padding a little
    #[inline]
    fn extend_padding(
        source: Margin,
        mode: CornerRadiusMode,
        amount: Margin
    ) -> Margin {
        match mode {
            CornerRadiusMode::Full => source + amount,
            CornerRadiusMode::Large
            | CornerRadiusMode::Small
            | CornerRadiusMode::None => {
                source
            }
        }
    }
    pub fn padding_mini(&self,mode: CornerRadiusMode) -> Margin {
        Self::extend_padding(
            self.padding.mini,
            mode,
            Margin::symmetric(4,0)
        )
    }
    pub fn padding_extra_small(&self,mode: CornerRadiusMode) -> Margin {
        Self::extend_padding(
            self.padding.extra_small,
            mode,
            Margin::symmetric(4,0)
        )
    }
    pub fn padding_small(&self,mode: CornerRadiusMode) -> Margin {
        Self::extend_padding(
            self.padding.small,
            mode,
            Margin::symmetric(4,0)
        )
    }
    pub fn padding_medium(&self,mode: CornerRadiusMode) -> Margin {
        Self::extend_padding(
            self.padding.medium,
            mode,
            Margin::symmetric(4,4)
        )
    }
    pub fn padding_large(&self,mode: CornerRadiusMode) -> Margin {
        Self::extend_padding(
            self.padding.large,
            mode,
            Margin::symmetric(4,4)
        )
    }
    pub fn padding_extra_large(&self,mode: CornerRadiusMode) -> Margin {
        Self::extend_padding(
            self.padding.extra_large,
            mode,
            Margin::symmetric(4,4)
        )
    }
    pub fn padding_huge(&self,mode: CornerRadiusMode) -> Margin {
        Self::extend_padding(
            self.padding.huge,
            mode,
            Margin::symmetric(4,4)
        )
    }
}

#[derive(Debug, Clone)]
pub struct ThemeBuilder {
    source_color: Color32,//default use blue source color
    brightness_mode: BrightnessMode,
    color_variant_mode: ColorVariantMode,
    corner_radius_mode: CornerRadiusMode,
    space_mode: SpaceMode,
    custom_color: Vec<CustomColorInput>,
}

impl ThemeBuilder {
    pub fn get_source_color(&self) -> Color32 {
        self.source_color
    }

    pub fn get_brightness_mode(&self) -> BrightnessMode {
        self.brightness_mode
    }

    pub fn get_color_variant_mode(&self) -> ColorVariantMode {
        self.color_variant_mode
    }
    pub fn get_color_variant_mode_mut(&mut self) -> &mut ColorVariantMode {
        &mut self.color_variant_mode
    }
    pub fn get_corner_radius_mode(&self) -> CornerRadiusMode {
        self.corner_radius_mode
    }

    pub fn get_space_mode(&self) -> SpaceMode {
        self.space_mode
    }
    pub fn get_custom_color(&self) -> &[CustomColorInput] {
        &self.custom_color
    }

    pub fn is_dark_mode(&self) -> bool {
        self.brightness_mode == BrightnessMode::Dark
    }

    pub fn is_full_size_corner_radius(&self) -> bool {
        self.corner_radius_mode == CornerRadiusMode::Full
    }

    pub fn is_large_corner_radius(&self) -> bool {
        self.corner_radius_mode == CornerRadiusMode::Large
    }

    pub fn is_small_corner_radius(&self) -> bool {
        self.corner_radius_mode == CornerRadiusMode::Small
    }

    pub fn is_none_corner_radius(&self) -> bool {
        self.corner_radius_mode == CornerRadiusMode::None
    }

    pub fn is_compact(&self) -> bool {
        self.space_mode == SpaceMode::Compact
    }

    pub fn set_color_variant(&mut self,var:ColorVariantMode) {
        self.color_variant_mode = var
    }

    pub fn set_corner_radius_mode(&mut self,var:CornerRadiusMode) {
        self.corner_radius_mode = var
    }

    pub fn set_space_mode(&mut self,var:SpaceMode) {
        self.space_mode = var
    }

    pub fn set_source_color(&mut self,var:Color32) {
        self.source_color = var
    }

    pub fn set_brightness(&mut self,var:BrightnessMode) {
        self.brightness_mode = var
    }
}

impl ThemeBuilder {
    pub fn source_color(mut self, color: Color32) -> Self  {
        self.source_color = color;
        self
    }
    pub fn light(mut self) -> Self {
        self.brightness_mode = BrightnessMode::Light;
        self
    }
    pub fn dark(mut self) -> Self {
        self.brightness_mode = BrightnessMode::Dark;
        self
    }
    pub fn compact(mut self) -> Self {
        self.space_mode = SpaceMode::Compact;
        self
    }
    pub fn loose(mut self) -> Self {
        self.space_mode = SpaceMode::Loose;
        self
    }
    pub fn full(mut self) -> Self {
        self.corner_radius_mode = CornerRadiusMode::Full;
        self
    }
    pub fn large_rounding(mut self) -> Self {
        self.corner_radius_mode = CornerRadiusMode::Large;
        self
    }
    pub fn small_rounding(mut self) -> Self {
        self.corner_radius_mode = CornerRadiusMode::Small;
        self
    }
    pub fn none_rounding(mut self) -> Self {
        self.corner_radius_mode = CornerRadiusMode::None;
        self
    }
    pub fn color_tonal_spot(mut self) -> Self {
        self.color_variant_mode = ColorVariantMode::TonalSpot;
        self
    }
    pub fn color_monochrome(mut self) -> Self {
        self.color_variant_mode = ColorVariantMode::Monochrome;
        self
    }
    pub fn color_neutral(mut self) -> Self {
        self.color_variant_mode = ColorVariantMode::Neutral;
        self
    }
    pub fn color_vibrant(mut self) -> Self {
        self.color_variant_mode = ColorVariantMode::Vibrant;
        self
    }
    pub fn color_expressive(mut self) -> Self {
        self.color_variant_mode = ColorVariantMode::Expressive;
        self
    }
    pub fn color_fidelity(mut self) -> Self {
        self.color_variant_mode = ColorVariantMode::Fidelity;
        self
    }
    pub fn color_content(mut self) -> Self {
        self.color_variant_mode = ColorVariantMode::Content;
        self
    }
    pub fn color_rainbow(mut self) -> Self {
        self.color_variant_mode = ColorVariantMode::Rainbow;
        self
    }
    pub fn color_fruit_salad(mut self) -> Self {
        self.color_variant_mode = ColorVariantMode::FruitSalad;
        self
    }

    pub fn build_theme(self) -> Theme {
        let Self {
            source_color,
            brightness_mode,
            color_variant_mode,
            corner_radius_mode,
            space_mode,
            custom_color
        } = self;
        Theme {
            source_color,
            brightness_mode,
            color_variant_mode,
            corner_radius_mode,
            space_mode,
            gap: build_gap(),
            padding: build_padding(space_mode),
            corner_radius: build_corner_radius(corner_radius_mode),
            stork_width_variable: Default::default(),
            image_size_variable: Default::default(),
            color: ColorTheme::new(
                source_color,
                color_variant_mode,
                brightness_mode.is_dark(),
                custom_color
            ),
        }
    }
}

fn build_gap() -> ThemeGap {
    ThemeGap {
        horizontal: SizeVariant {
            mini: 4.,
            extra_small: 6.,
            small: 8.,
            medium: 10.,
            large: 12.,
            extra_large: 16.,
            huge: 20.,
        },
        vertical: SizeVariant {
            mini: 2.,
            extra_small: 4.,
            small: 6.,
            medium: 8.,
            large: 10.,
            extra_large: 12.,
            huge: 16.,
        },
    }
}

fn build_padding(
    space_mode: SpaceMode,
) -> SizeVariant<Margin> {
    match space_mode {
        SpaceMode::Loose => {
            SizeVariant {
                mini: Margin::symmetric(8,6),
                extra_small: Margin::symmetric(16,10),
                small: Margin::symmetric(24,16),
                medium: Margin::symmetric(32,20),
                large: Margin::symmetric(40,24),
                extra_large: Margin::symmetric(48,32),
                huge: Margin::symmetric(64,48),
            }
        }
        SpaceMode::Compact => {
            SizeVariant {
                mini: Margin::symmetric(6,2),
                extra_small: Margin::symmetric(8,4),
                small: Margin::symmetric(12,8),
                medium: Margin::symmetric(16,12),
                large: Margin::symmetric(20,16),
                extra_large: Margin::symmetric(24,20),
                huge: Margin::symmetric(32,24),
            }
        }
    }
}

fn build_corner_radius(mode: CornerRadiusMode) -> SizeVariant<CornerRadius> {
    match mode {
        CornerRadiusMode::Full => {
            SizeVariant {
                mini: CornerRadius::same(2),
                extra_small: CornerRadius::same(4),
                small: CornerRadius::same(8),
                medium: CornerRadius::same(12),
                large: CornerRadius::same(16),
                extra_large: CornerRadius::same(20),
                huge: CornerRadius::same(28),
            }
        }
        CornerRadiusMode::Large => {
            SizeVariant {
                mini: CornerRadius::same(3),
                extra_small: CornerRadius::same(3),
                small: CornerRadius::same(6),
                medium: CornerRadius::same(6),
                large: CornerRadius::same(8),
                extra_large: CornerRadius::same(10),
                huge: CornerRadius::same(10),
            }
        }
        CornerRadiusMode::Small => {
            SizeVariant {
                mini: CornerRadius::same(2),
                extra_small: CornerRadius::same(2),
                small: CornerRadius::same(2),
                medium: CornerRadius::same(2),
                large: CornerRadius::same(2),
                extra_large: CornerRadius::same(2),
                huge: CornerRadius::same(4),
            }
        }
        CornerRadiusMode::None => {
            SizeVariant {
                mini: CornerRadius::same(0),
                extra_small: CornerRadius::same(0),
                small: CornerRadius::same(0),
                medium: CornerRadius::same(0),
                large: CornerRadius::same(0),
                extra_large: CornerRadius::same(0),
                huge: CornerRadius::same(0),
            }
        }
    }
}

impl Default for ThemeBuilder {
    fn default() -> Self {
        Self {
            source_color: Color32::from_rgb(0, 140, 255), //default use blue source color
            brightness_mode: BrightnessMode::Dark,
            color_variant_mode: ColorVariantMode::TonalSpot,
            corner_radius_mode: CornerRadiusMode::Full,
            space_mode: SpaceMode::Loose,
            custom_color: vec![],
        }
    }
}
#[derive(Debug, Clone)]
pub struct ArcTheme(Arc<RwLock<Theme>>);

impl PartialEq for ArcTheme {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Default for ArcTheme {
    fn default() -> Self {
        ArcTheme(Arc::new(RwLock::new(ThemeBuilder::default().build_theme())))
    }
}

impl ArcTheme {
    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, Theme> {
        self.0.read().unwrap()
    }

    pub fn write(&mut self, new_theme: Theme) {
        let mut w = self.0.write().unwrap();
        *w = new_theme;
    }
}


