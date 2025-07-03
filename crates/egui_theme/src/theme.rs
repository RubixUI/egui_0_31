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

#[derive(Default,Debug, Clone,PartialEq)]
pub struct Typography {
    pub line_height: f32,
    pub font_id: FontId,
}

#[derive(Default, Debug, Hash, Ord, Clone, PartialEq, Eq, PartialOrd)]
pub enum TypographySize {
    #[default]
    Medium,
    Small,
    Large
}

#[derive(Debug, Hash, Ord, Clone, PartialEq, Eq, PartialOrd)]
pub struct TypographyVariable {
    bold: bool,
    italic: bool,
    size: TypographySize
}
impl Default for TypographyVariable {
    fn default() -> Self {
        Self {
            bold: false,
            italic: false,
            size: Default::default(),
        }
    }
}
impl TypographyVariable {
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
    pub fn small(mut self) -> Self {
        self.size = TypographySize::Small;
        self
    }
    pub fn large(mut self) -> Self {
        self.size = TypographySize::Large;
        self
    }
}
#[derive(Debug, Clone, Hash, Ord, Eq, PartialEq, PartialOrd)]
pub enum TypographyStyles {
    Display(TypographyVariable),
    Headline(TypographyVariable),
    Title(TypographyVariable),
    Body(TypographyVariable),
    Label(TypographyVariable),
    Monospace(TypographyVariable),
    Name(String)
}

impl TypographyStyles {
    pub fn label() -> Self {
        TypographyStyles::Label(TypographyVariable::default())
    }
    pub fn display() -> Self {
        TypographyStyles::Display(TypographyVariable::default())
    }
    pub fn body() -> Self {
        TypographyStyles::Body(TypographyVariable::default())
    }

    pub fn title() -> Self {
        TypographyStyles::Title(TypographyVariable::default())
    }
    pub fn headline() -> Self {
        TypographyStyles::Headline(TypographyVariable::default())
    }
    pub fn monospace() -> Self {
        TypographyStyles::Monospace(TypographyVariable::default())
    }
    pub fn name(name:String) -> Self {
        TypographyStyles::name(name)
    }
    pub fn get_variable(&self) -> TypographyVariable {
        match self {
            TypographyStyles::Display(v)
            | TypographyStyles::Headline(v)
            | TypographyStyles::Title(v)
            | TypographyStyles::Body(v)
            | TypographyStyles::Label(v)
            | TypographyStyles::Monospace(v) => {
                v.clone()
            }
            TypographyStyles::Name(_) => TypographyVariable::default(),
        }
    }
    fn set_variable(self,var:TypographyVariable) -> Self {
        match self {
            TypographyStyles::Display(_) => {
                TypographyStyles::Display(var)
            },
            TypographyStyles::Headline(_) => {
                TypographyStyles::Headline(var)
            },
            TypographyStyles::Body(_) => {
                TypographyStyles::Body(var)
            },
            TypographyStyles::Label(_) => {
                TypographyStyles::Label(var)
            },
            TypographyStyles::Monospace(_) => {
                TypographyStyles::Monospace(var)
            },
            TypographyStyles::Name(_) => self,
            TypographyStyles::Title(_) => {
                TypographyStyles::Title(var)
            }
        }
    }
    pub fn bold(mut self) -> Self {
        let mut var = self.get_variable();
        var = var.bold();
        self.set_variable(var)
    }
    pub fn italic(mut self) -> Self {
        let mut var = self.get_variable();
        var = var.italic();
        self.set_variable(var)
    }
    pub fn small(mut self) -> Self {
        let mut var = self.get_variable();
        var = var.small();
        self.set_variable(var)
    }
    pub fn large(mut self) -> Self {
        let mut var = self.get_variable();
        var = var.large();
        self.set_variable(var)
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
    Circular,
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

#[derive(Debug, Clone,PartialEq)]
pub struct ThemeGap {
    horizontal: SizeVariant<f32>,
    vertical: SizeVariant<f32>,
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
    pub color_theme: ColorTheme,
    pub typography: BTreeMap<TypographyStyles, Typography>,
}

impl Theme {
    pub fn typography_label(&self) -> Typography {
        self.typography.get(&TypographyStyles::label()).unwrap().clone()
    }

    pub fn typography_body(&self) -> Typography {
        self.typography.get(&TypographyStyles::body()).unwrap().clone()
    }
}

impl Theme {
    pub fn roboto_fonts() -> FontDefinitions {
        let mut font_defs = FontDefinitions::default();
        // Add fonts (example - you may need to adjust paths or use embedded fonts)
        // Regular font
        font_defs.font_data.insert(
            "regular".to_string(),
            Arc::new(FontData::from_static(include_bytes!("./data/fonts/Roboto-Regular.ttf"))),
        );

        // SemiBold font
        font_defs.font_data.insert(
            "semibold".to_string(),
            Arc::new(FontData::from_static(include_bytes!("./data/fonts/Roboto-SemiBold.ttf"))),
        );

        font_defs.font_data.insert(
            "regular_italic".to_string(),
            Arc::new(FontData::from_static(include_bytes!("./data/fonts/Roboto-Italic.ttf"))),
        );

        // SemiBold font
        font_defs.font_data.insert(
            "semibold_italic".to_string(),
            Arc::new(FontData::from_static(include_bytes!("./data/fonts/Roboto-SemiBoldItalic.ttf"))),
        );

        font_defs.font_data.insert(
            "mono".to_string(),
            Arc::new(FontData::from_static(include_bytes!("./data/fonts/RobotoMono-Regular.ttf"))),
        );

        // SemiBold font
        font_defs.font_data.insert(
            "mono_italic".to_string(),
            Arc::new(FontData::from_static(include_bytes!("./data/fonts/RobotoMono-Italic.ttf"))),
        );

        font_defs.font_data.insert(
            "mono_bold".to_string(),
            Arc::new(FontData::from_static(include_bytes!("./data/fonts/RobotoMono-SemiBold.ttf"))),
        );

        // SemiBold font
        font_defs.font_data.insert(
            "mono_bold_italic".to_string(),
            Arc::new(FontData::from_static(include_bytes!("./data/fonts/RobotoMono-SemiBoldItalic.ttf"))),
        );


        font_defs.families.insert(
            FontFamily::Name("regular".into()),
            vec!["regular".to_string()],
        );

        font_defs.families.insert(
            FontFamily::Name("semibold".into()),
            vec!["semibold".to_string()],
        );

        font_defs.families.insert(
            FontFamily::Name("regular_italic".into()),
            vec!["regular_italic".to_string()],
        );

        font_defs.families.insert(
            FontFamily::Name("semibold_italic".into()),
            vec!["semibold_italic".to_string()],
        );

        font_defs.families.insert(
            FontFamily::Name("mono".into()),
            vec!["mono".to_string()],
        );

        font_defs.families.insert(
            FontFamily::Name("mono_italic".into()),
            vec!["mono_italic".to_string()],
        );

        font_defs.families.insert(
            FontFamily::Name("mono_bold".into()),
            vec!["mono_bold".to_string()],
        );

        font_defs.families.insert(
            FontFamily::Name("mono_bold_italic".into()),
            vec!["mono_bold_italic".to_string()],
        );
        font_defs
    }

    /// Helper function to get font_id for a typography design
    fn get_font_id(&self, style: TypographyStyles) -> FontId {
        self.typography.get(&style)
            .map(|t| t.font_id.clone())
            .unwrap_or_default()
    }


    /// Get font_id for a custom named typography design
    pub fn font_id_named(&self, name: &str) -> FontId {
        self.get_font_id(TypographyStyles::Name(name.to_string()))
    }

    /// Helper function to get line height for a typography design
    fn get_line_height(&self, style: TypographyStyles) -> Option<f32> {
        self.typography.get(&style)
            .map(|t| t.line_height)
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
    pub fn circular(mut self) -> Self {
        self.corner_radius_mode = CornerRadiusMode::Circular;
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
        let mut typo = BTreeMap::new();
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
            color_theme: ColorTheme::new(
                source_color,
                color_variant_mode,
                brightness_mode.is_dark(),
                custom_color
            ),
            typography: typo,
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
                extra_small: Margin::symmetric(12,10),
                small: Margin::symmetric(20,16),
                medium: Margin::symmetric(16,16),
                large: Margin::symmetric(20,20),
                extra_large: Margin::symmetric(24,24),
                huge: Margin::symmetric(30,30),
            }
        }
        SpaceMode::Compact => {
            SizeVariant {
                mini: Margin::symmetric(6,2),
                extra_small: Margin::symmetric(8,6),
                small: Margin::symmetric(12,10),
                medium: Margin::symmetric(12,12),
                large: Margin::symmetric(16,16),
                extra_large: Margin::symmetric(20,20),
                huge: Margin::symmetric(24,24),
            }
        }
    }
}

fn build_corner_radius(mode: CornerRadiusMode) -> SizeVariant<CornerRadius> {
    match mode {
        CornerRadiusMode::Circular => {
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
#[cfg(feature = "roboto_font")]
fn font_family(token: &TypographyStyles) -> FontFamily {
    match &token {
        TypographyStyles::Display(t)
        | TypographyStyles::Headline(t)
        | TypographyStyles::Title(t)
        | TypographyStyles::Body(t)
        | TypographyStyles::Label(t) => {
            match (t.bold, t.italic) {
                (true, true) => FontFamily::Name("semibold_italic".into()),
                (true, false) => FontFamily::Name("semibold".into()),
                (false, true) => FontFamily::Name("regular_italic".into()),
                (false, false) => FontFamily::Name("regular".into()),
            }
        }
        TypographyStyles::Monospace(t) => {
            match (t.bold, t.italic) {
                (true, true) => FontFamily::Name("mono_bold_italic".into()),
                (true, false) => FontFamily::Name("mono_bold".into()),
                (false, true) => FontFamily::Name("mono_italic".into()),
                (false, false) => FontFamily::Name("mono".into()),
            }
        }
        TypographyStyles::Name(_) => {
            FontFamily::Name("regular".into())
        }
    }
}

#[cfg(not(feature = "roboto_font"))]
fn font_family(token: &TypographyStyles) -> FontFamily {
    match &token {
        TypographyStyles::Display(t)
        | TypographyStyles::Headline(t)
        | TypographyStyles::Title(t)
        | TypographyStyles::Body(t)
        | TypographyStyles::Label(t) => {
            FontFamily::Proportional
        }
        TypographyStyles::Monospace(t) => {
            FontFamily::Monospace
        }
        TypographyStyles::Name(_) => {
            FontFamily::Name("regular".into())
        }
    }
}

fn default_typography(typography: &mut BTreeMap<TypographyStyles, Typography>) {
    let update_font_id = |
        typography: &mut BTreeMap<TypographyStyles, Typography>,
        token: TypographyStyles| {
        let (font_size,line_height) = match &token {
            TypographyStyles::Display(t) => {
                match t.size {
                    TypographySize::Large => (57.,64.),
                    TypographySize::Medium => (45.,52.),
                    TypographySize::Small => (36.,44.),
                }
            }
            TypographyStyles::Headline(t) => {
                match t.size {
                    TypographySize::Large => (32.,40.),
                    TypographySize::Medium => (28.,36.),
                    TypographySize::Small => (24.,32.),
                }
            }
            TypographyStyles::Title(t) => {
                match t.size {
                    TypographySize::Large => (22.,28.),
                    TypographySize::Medium => (16.,24.),
                    TypographySize::Small => (14.,20.),
                }
            }
            TypographyStyles::Body(t) | TypographyStyles::Monospace(t) => {
                match t.size {
                    TypographySize::Large => (16.,24.),
                    TypographySize::Medium => (14.,20.),
                    TypographySize::Small => (12.,16.),
                }
            }
            TypographyStyles::Label(t) => {
                match t.size {
                    TypographySize::Large => (14.,20.),
                    TypographySize::Medium => (12.,16.),
                    TypographySize::Small => (11.,16.),
                }
            }
            TypographyStyles::Name(t) => {
                (14.,20.)
            }
        };
        let font_f = font_family(&token);
        typography.insert(token, Typography { font_id:FontId::new(font_size,font_f), line_height });
    };
    let bools = [true, false];
    let sizes = [TypographySize::Small, TypographySize::Medium, TypographySize::Large];
    for &bold in &bools {
        for &italic in &bools {
            for size in &sizes {
                let t = TypographyVariable { bold, italic, size: size.clone() };
                update_font_id(typography, TypographyStyles::Display(t.clone()));
                update_font_id(typography, TypographyStyles::Headline(t.clone()));
                update_font_id(typography, TypographyStyles::Title(t.clone()));
                update_font_id(typography, TypographyStyles::Body(t.clone()));
                update_font_id(typography, TypographyStyles::Label(t.clone()));
                update_font_id(typography, TypographyStyles::Monospace(t.clone()));
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
            corner_radius_mode: CornerRadiusMode::Circular,
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

    pub fn write(&self, new_theme: Theme) {
        let mut w = self.0.write().unwrap();
        *w = new_theme;
    }
}


