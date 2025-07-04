use std::collections::BTreeMap;
use epaint::Color32;
use crate::color_variant::content::SchemeContent;
use crate::color_variant::expressive::SchemeExpressive;
use crate::color_variant::fidelity::SchemeFidelity;
use crate::color_variant::fruit_salad::SchemeFruitSalad;
use crate::color_variant::monochrome::SchemeMonochrome;
use crate::color_variant::neutral::SchemeNeutral;
use crate::color_variant::rainbow::SchemeRainbow;
use crate::color_variant::tonal_spot::SchemeTonalSpot;
use crate::color_variant::vibrant::SchemeVibrant;
use crate::dynamic_schema::DynamicScheme;
use crate::htc::Hct;
use crate::theme::ColorVariantMode;

#[derive(Clone, Debug, Copy, PartialEq)]
//DynamicColor is dynamic for different mode: light or dark
pub struct DynamicColor {
    pub bg: Color32,
    pub fg: Color32,
    pub bg_container: Color32,
    pub fg_container: Color32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FixedColor {
    bg: Color32,
    fg: Color32,
    bg_dim: Color32,
    fg_variant: Color32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WidgetColor {
    dyn_color: DynamicColor,
    fixed_color: FixedColor,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceColor {
    surface: Color32,
    surface_bright: Color32,
    surface_container: Color32,
    surface_container_high: Color32,
    surface_container_highest: Color32,
    surface_container_low: Color32,
    surface_container_lowest: Color32,
    surface_dim: Color32,
    surface_tint: Color32,
    surface_variant: Color32,
    on_surface: Color32,
    on_surface_variant: Color32,
}

pub struct CustomColor {
    name: String,
    color: DynamicColor
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColorTheme {
    primary: WidgetColor,
    secondary: WidgetColor,
    tertiary: WidgetColor,
    surface: SurfaceColor,
    error: DynamicColor,
    //inverse
    inverse_on_surface: Color32,
    inverse_primary: Color32,
    inverse_surface: Color32,
    //background
    background: Color32,
    on_background: Color32,
    //outline
    outline: Color32, //Border
    outline_variant: Color32, //Decorative elements, such as dividers
    //others roles
    scrim: Color32, //纱幕
    shadow: Color32,
    //state layer opacity level
    level1: u8,
    level2: u8,
    level3: u8,
    //custom
    custom: BTreeMap<String, DynamicColor>,
}

impl ColorTheme {

    pub fn dyn_primary(&self) -> DynamicColor {
        self.primary.dyn_color
    }

    pub fn dyn_secondary(&self) -> DynamicColor {
        self.secondary.dyn_color
    }

    pub fn dyn_tertiary(&self) -> DynamicColor {
        self.tertiary.dyn_color
    }

    pub fn dyn_error(&self) -> DynamicColor {
        self.error
    }

    pub fn dyn_warning(&self) -> DynamicColor {
        self.custom("warning")
    }

    pub fn dyn_success(&self) -> DynamicColor {
        self.custom("success")
    }
    pub fn primary(&self) -> Color32 { self.primary.dyn_color.bg }
    pub fn on_primary(&self) -> Color32 { self.primary.dyn_color.fg }
    pub fn primary_container(&self) -> Color32 { self.primary.dyn_color.bg_container }
    pub fn on_primary_container(&self) -> Color32 { self.primary.dyn_color.fg_container }
    pub fn primary_fixed(&self) -> Color32 { self.primary.fixed_color.bg }
    pub fn primary_fixed_dim(&self) -> Color32 { self.primary.fixed_color.bg_dim }
    pub fn on_primary_fixed(&self) -> Color32 { self.primary.fixed_color.fg}
    pub fn on_primary_fixed_variant(&self) -> Color32 { self.primary.fixed_color.fg_variant }

    pub fn secondary(&self) -> Color32 { self.secondary.dyn_color.bg }
    pub fn on_secondary(&self) -> Color32 { self.secondary.dyn_color.fg }
    pub fn secondary_container(&self) -> Color32 { self.secondary.dyn_color.bg_container }
    pub fn on_secondary_container(&self) -> Color32 { self.secondary.dyn_color.fg_container }
    pub fn secondary_fixed(&self) -> Color32 { self.secondary.fixed_color.bg }
    pub fn secondary_fixed_dim(&self) -> Color32 { self.secondary.fixed_color.bg_dim }
    pub fn on_secondary_fixed_(&self) -> Color32 { self.secondary.fixed_color.fg}
    pub fn on_secondary_fixed_variant(&self) -> Color32 { self.secondary.fixed_color.fg_variant }

    pub fn tertiary(&self) -> Color32 { self.secondary.dyn_color.bg }
    pub fn on_tertiary(&self) -> Color32 { self.secondary.dyn_color.fg }
    pub fn tertiary_container(&self) -> Color32 { self.secondary.dyn_color.bg_container }
    pub fn on_tertiary_container(&self) -> Color32 { self.secondary.dyn_color.fg_container }
    pub fn tertiary_fixed(&self) -> Color32 { self.secondary.fixed_color.bg }
    pub fn tertiary_fixed_dim(&self) -> Color32 { self.secondary.fixed_color.bg_dim }
    pub fn on_tertiary_fixed(&self) -> Color32 { self.secondary.fixed_color.fg}
    pub fn on_tertiary_fixed_variant(&self) -> Color32 { self.secondary.fixed_color.fg_variant }

    pub fn error(&self) -> Color32 { self.error.bg }
    pub fn on_error(&self) -> Color32 { self.error.fg }
    pub fn error_container(&self) -> Color32 { self.error.bg_container }
    pub fn on_error_container(&self) -> Color32 { self.error.fg_container }

    pub fn warning(&self) -> Color32 { self.custom("warning").bg }
    pub fn on_warning(&self) -> Color32 { self.custom("warning").fg }
    pub fn warning_container(&self) -> Color32 { self.custom("warning").bg_container }
    pub fn on_warning_container(&self) -> Color32 { self.custom("warning").fg_container }

    pub fn success(&self) -> Color32 { self.custom("success").bg }
    pub fn on_success(&self) -> Color32 { self.custom("success").fg }
    pub fn success_container(&self) -> Color32 { self.custom("success").bg_container }
    pub fn on_success_container(&self) -> Color32 { self.custom("success").fg_container }
    //surface
    pub fn surface(&self) -> Color32 { self.surface.surface }
    pub fn surface_dim(&self) -> Color32 { self.surface.surface_dim }
    pub fn surface_bright(&self) -> Color32 { self.surface.surface_bright }
    pub fn surface_container(&self) -> Color32 { self.surface.surface_container }
    pub fn surface_container_low(&self) -> Color32 { self.surface.surface_container_low }
    pub fn surface_container_lowest(&self) -> Color32 { self.surface.surface_container_lowest }
    pub fn surface_container_high(&self) -> Color32 { self.surface.surface_container_high }
    pub fn surface_container_highest(&self) -> Color32 { self.surface.surface_container_highest }
    pub fn on_surface(&self) -> Color32 { self.surface.on_surface }
    pub fn on_surface_variant(&self) -> Color32 { self.surface.on_surface_variant }
    pub fn surface_tint(&self) -> Color32 { self.surface.surface_tint }
    pub fn surface_variant(&self) -> Color32 { self.surface.surface_variant }
    //others

    //inverse
    pub fn inverse_on_surface(&self) -> Color32 { self.inverse_on_surface }
    pub fn inverse_primary(&self) -> Color32 { self.inverse_primary }
    pub fn inverse_surface(&self) -> Color32 { self.inverse_surface }
    pub fn background(&self) -> Color32 { self.background }
    pub fn on_background(&self) -> Color32 { self.on_background }
    pub fn outline(&self) -> Color32 { self.outline }
    pub fn outline_variant(&self) -> Color32 { self.outline_variant }
    pub fn scrim(&self) -> Color32 { self.scrim }
    pub fn shadow(&self) -> Color32 { self.shadow }

    pub fn custom(&self,name: impl Into<String>) -> DynamicColor {
        self.custom.get(&name.into()).unwrap_or(&self.error).clone()
    }
}
#[derive(Debug, Clone)]
pub struct CustomColorInput {
    pub color: Color32,
    pub name: String,
    pub blend: bool,
}

impl CustomColorInput {
    pub fn to_dynamic(self,variant: ColorVariantMode,source_color:Color32,is_dark:bool) -> DynamicColor {
        let Self{color, name, blend } = self;
        let dyn_schema = get_schema(variant, color, is_dark);
        DynamicColor {
            bg: dyn_schema.primary().into(),
            fg: dyn_schema.on_primary().into(),
            bg_container: dyn_schema.primary_container().into(),
            fg_container: dyn_schema.on_primary_container().into(),
        }
    }
}

impl ColorTheme {
    pub fn new(source_color:Color32,variant: ColorVariantMode,is_dark: bool,input_custom_colors: Vec<CustomColorInput>) -> Self {
        let dynamic_schema = get_schema(variant, source_color, is_dark);
        //custom
        let mut static_custom_colors = vec![
            CustomColorInput {
                color: Color32::from_rgb(50,205,88),
                name: "success".to_string(),
                blend: false,
            },
            CustomColorInput {
                color: Color32::from_rgb(255,143,44),
                name: "warning".to_string(),
                blend: false,
            },
        ];
        static_custom_colors.extend(input_custom_colors);
        let custom_colors = static_custom_colors
            .into_iter()
            .fold(
                BTreeMap::new(),
                |mut acc,color| {
                    let name = color.name.clone();
                    let dy = color.to_dynamic(variant,source_color,is_dark);
                    acc.entry(name)
                        .and_modify(|e| *e = dy) // 如果已有，则累加
                        .or_insert(dy);           // 如果没有，则插入
                    acc
                });
        Self {
            primary: WidgetColor {
                dyn_color: DynamicColor {
                    bg: dynamic_schema.primary().into(),
                    fg: dynamic_schema.on_primary().into(),
                    bg_container: dynamic_schema.primary_container().into(),
                    fg_container: dynamic_schema.on_primary_container().into(),
                },
                fixed_color: FixedColor {
                    bg: dynamic_schema.primary_fixed().into(),
                    fg: dynamic_schema.on_primary_fixed().into(),
                    bg_dim: dynamic_schema.primary_fixed_dim().into(),
                    fg_variant: dynamic_schema.on_primary_fixed_variant().into(),
                },
            },
            secondary: WidgetColor {
                dyn_color: DynamicColor {
                    bg: dynamic_schema.secondary().into(),
                    fg: dynamic_schema.on_secondary().into(),
                    bg_container: dynamic_schema.secondary_container().into(),
                    fg_container: dynamic_schema.on_secondary_container().into(),
                },
                fixed_color: FixedColor {
                    bg: dynamic_schema.secondary_fixed().into(),
                    fg: dynamic_schema.on_secondary_fixed().into(),
                    bg_dim: dynamic_schema.secondary_fixed_dim().into(),
                    fg_variant: dynamic_schema.on_secondary_fixed_variant().into(),
                },
            },
            tertiary: WidgetColor {
                dyn_color: DynamicColor {
                    bg: dynamic_schema.tertiary().into(),
                    fg: dynamic_schema.on_tertiary().into(),
                    bg_container: dynamic_schema.tertiary_container().into(),
                    fg_container: dynamic_schema.on_tertiary_container().into(),
                },
                fixed_color: FixedColor {
                    bg: dynamic_schema.tertiary_fixed().into(),
                    fg: dynamic_schema.on_tertiary_fixed().into(),
                    bg_dim: dynamic_schema.tertiary_fixed_dim().into(),
                    fg_variant: dynamic_schema.on_tertiary_fixed_variant().into(),
                },
            },
            surface: SurfaceColor {
                surface: dynamic_schema.surface().into(),
                surface_bright: dynamic_schema.surface_bright().into(),
                surface_container: dynamic_schema.surface_container().into(),
                surface_container_high: dynamic_schema.surface_container_high().into(),
                surface_container_highest: dynamic_schema.surface_container_highest().into(),
                surface_container_low: dynamic_schema.surface_container_low().into(),
                surface_container_lowest: dynamic_schema.surface_container_lowest().into(),
                surface_dim: dynamic_schema.surface_dim().into(),
                surface_tint: dynamic_schema.surface_tint().into(),
                surface_variant: dynamic_schema.surface_variant().into(),
                on_surface: dynamic_schema.on_surface().into(),
                on_surface_variant: dynamic_schema.on_surface_variant().into(),
            },
            error: DynamicColor {
                bg: dynamic_schema.error().into(),
                fg: dynamic_schema.on_error().into(),
                bg_container: dynamic_schema.error_container().into(),
                fg_container: dynamic_schema.on_error_container().into(),
            },
            inverse_on_surface: dynamic_schema.inverse_on_surface().into(),
            inverse_primary: dynamic_schema.inverse_primary().into(),
            inverse_surface: dynamic_schema.inverse_surface().into(),
            background: dynamic_schema.background().into(),
            on_background: dynamic_schema.on_background().into(),
            outline: dynamic_schema.outline().into(),
            outline_variant: dynamic_schema.outline_variant().into(),
            scrim: dynamic_schema.scrim().into(),
            shadow: dynamic_schema.shadow().into(),
            level1: 2,//0.08
            level2: 3,//0.1
            level3: 6,//0.16
            custom: custom_colors,
        }
    }
}

fn get_schema(variant:ColorVariantMode,color:Color32,is_dark:bool) -> DynamicScheme {
    let htc = Hct::new(color.into());
    match variant {
        ColorVariantMode::Monochrome => {
            SchemeMonochrome::new(htc, is_dark,None).scheme
        }
        ColorVariantMode::Neutral => SchemeNeutral::new(htc, is_dark,None).scheme,
        ColorVariantMode::TonalSpot => SchemeTonalSpot::new(htc, is_dark,None).scheme,
        ColorVariantMode::Vibrant => SchemeVibrant::new(htc, is_dark,None).scheme,
        ColorVariantMode::Expressive => {
            SchemeExpressive::new(htc, is_dark,None).scheme
        }
        ColorVariantMode::Fidelity => SchemeFidelity::new(htc, is_dark,None).scheme,
        ColorVariantMode::Content => SchemeContent::new(htc, is_dark,None).scheme,
        ColorVariantMode::Rainbow => SchemeRainbow::new(htc, is_dark,None).scheme,
        ColorVariantMode::FruitSalad => {
            SchemeFruitSalad::new(htc, is_dark,None).scheme
        }
    }
}

