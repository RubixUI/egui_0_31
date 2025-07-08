use std::collections::BTreeMap;
use std::sync::Arc;
use epaint::{FontFamily, FontId};
use epaint::text::{FontData, FontDefinitions};
#[derive(Default,Debug, Clone,PartialEq)]
pub struct Font {
    pub line_height: f32,
    pub font_size: f32,
    pub font_id: FontId,
}

#[derive(Default, Debug, Hash, Ord, Clone, PartialEq, Eq, PartialOrd)]
pub enum FontSize {
    #[default]
    Medium,
    Small,
    Large
}

#[derive(Debug, Hash, Ord, Clone, PartialEq, Eq, PartialOrd)]
pub struct FontVariable {
    bold: bool,
    italic: bool,
    size: FontSize
}
impl Default for FontVariable {
    fn default() -> Self {
        Self {
            bold: false,
            italic: false,
            size: Default::default(),
        }
    }
}
impl FontVariable {
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
    pub fn small(mut self) -> Self {
        self.size = FontSize::Small;
        self
    }
    pub fn large(mut self) -> Self {
        self.size = FontSize::Large;
        self
    }
}
#[derive(Debug, Clone, Hash, Ord, Eq, PartialEq, PartialOrd)]
pub enum FontStyles {
    Display(FontVariable),
    Headline(FontVariable),
    Title(FontVariable),
    Body(FontVariable),
    Label(FontVariable),
    Monospace(FontVariable),
    Name(String)
}

impl FontStyles {
    pub fn label() -> Self {
        FontStyles::Label(FontVariable::default())
    }
    pub fn display() -> Self {
        FontStyles::Display(FontVariable::default())
    }
    pub fn body() -> Self {
        FontStyles::Body(FontVariable::default())
    }

    pub fn title() -> Self {
        FontStyles::Title(FontVariable::default())
    }
    pub fn headline() -> Self {
        FontStyles::Headline(FontVariable::default())
    }
    pub fn monospace() -> Self {
        FontStyles::Monospace(FontVariable::default())
    }
    pub fn name(name:String) -> Self {
        FontStyles::name(name)
    }
    pub fn get_variable(&self) -> FontVariable {
        match self {
            FontStyles::Display(v)
            | FontStyles::Headline(v)
            | FontStyles::Title(v)
            | FontStyles::Body(v)
            | FontStyles::Label(v)
            | FontStyles::Monospace(v) => {
                v.clone()
            }
            FontStyles::Name(_) => FontVariable::default(),
        }
    }
    fn set_variable(self,var:FontVariable) -> Self {
        match self {
            FontStyles::Display(_) => {
                FontStyles::Display(var)
            },
            FontStyles::Headline(_) => {
                FontStyles::Headline(var)
            },
            FontStyles::Body(_) => {
                FontStyles::Body(var)
            },
            FontStyles::Label(_) => {
                FontStyles::Label(var)
            },
            FontStyles::Monospace(_) => {
                FontStyles::Monospace(var)
            },
            FontStyles::Name(_) => self,
            FontStyles::Title(_) => {
                FontStyles::Title(var)
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
#[derive(Debug, Clone,PartialEq)]
pub struct FontTheme {
    pub font_map: BTreeMap<FontStyles, Font>,
}
impl FontTheme {
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
    fn get_font_id(&self, style: FontStyles) -> FontId {
        self.font_map.get(&style)
            .map(|t| t.font_id.clone())
            .unwrap_or_default()
    }

    fn get_font(&self, style: FontStyles) -> Font {
        self.font_map.get(&style)
            .map(|t| t.clone())
            .unwrap_or_default()
    }


    /// Get font_id for a custom named typography design
    pub fn font_id_named(&self, name: &str) -> FontId {
        self.get_font_id(FontStyles::Name(name.to_string()))
    }

    /// Helper function to get line height for a typography design
    fn get_line_height(&self, style: FontStyles) -> Option<f32> {
        self.font_map.get(&style)
            .map(|t| t.line_height)
    }
}
impl Default for FontTheme {
    fn default() -> Self {
        let mut font_map = BTreeMap::new();
        default_typography(&mut font_map);
        Self { font_map }
    }
}

impl FontTheme {
    pub fn font_label(&self) -> Font {
        self.font_map.get(&FontStyles::label()).unwrap().clone()
    }

    pub fn font_body(&self) -> Font {
        self.font_map.get(&FontStyles::body()).unwrap().clone()
    }
}
#[derive(Debug, Clone)]
pub struct ArcFontTheme(Arc<FontTheme>);

impl ArcFontTheme {
    pub fn get_font(&self, style: FontStyles) -> Font {
        self.0.get_font(style)
    }
    pub fn get_font_id(&self, style: FontStyles) -> FontId {
        self.0.get_font_id(style)
    }
}

impl PartialEq for ArcFontTheme {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Default for ArcFontTheme {
    fn default() -> Self {
        ArcFontTheme(Arc::new(FontTheme::default()))
    }
}

#[cfg(feature = "roboto_font")]
fn font_family(token: &FontStyles) -> FontFamily {
    match &token {
        FontStyles::Display(t)
        | FontStyles::Headline(t)
        | FontStyles::Title(t)
        | FontStyles::Body(t)
        | FontStyles::Label(t) => {
            match (t.bold, t.italic) {
                (true, true) => FontFamily::Name("semibold_italic".into()),
                (true, false) => FontFamily::Name("semibold".into()),
                (false, true) => FontFamily::Name("regular_italic".into()),
                (false, false) => FontFamily::Name("regular".into()),
            }
        }
        FontStyles::Monospace(t) => {
            match (t.bold, t.italic) {
                (true, true) => FontFamily::Name("mono_bold_italic".into()),
                (true, false) => FontFamily::Name("mono_bold".into()),
                (false, true) => FontFamily::Name("mono_italic".into()),
                (false, false) => FontFamily::Name("mono".into()),
            }
        }
        FontStyles::Name(_) => {
            FontFamily::Name("regular".into())
        }
    }
}

#[cfg(not(feature = "roboto_font"))]
fn font_family(token: &FontStyles) -> FontFamily {
    match &token {
        FontStyles::Display(t)
        | FontStyles::Headline(t)
        | FontStyles::Title(t)
        | FontStyles::Body(t)
        | FontStyles::Label(t) => {
            FontFamily::Proportional
        }
        FontStyles::Monospace(t) => {
            FontFamily::Monospace
        }
        FontStyles::Name(_) => {
            FontFamily::Name("regular".into())
        }
    }
}

fn default_typography(typography: &mut BTreeMap<FontStyles, Font>) {
    let update_font_id = |
        typography: &mut BTreeMap<FontStyles, Font>,
        token: FontStyles| {
        let (font_size,line_height) = match &token {
            FontStyles::Display(t) => {
                match t.size {
                    FontSize::Large => (57.,64.),
                    FontSize::Medium => (45.,52.),
                    FontSize::Small => (36.,44.),
                }
            }
            FontStyles::Headline(t) => {
                match t.size {
                    FontSize::Large => (32.,40.),
                    FontSize::Medium => (28.,36.),
                    FontSize::Small => (24.,32.),
                }
            }
            FontStyles::Title(t) => {
                match t.size {
                    FontSize::Large => (22.,28.),
                    FontSize::Medium => (16.,24.),
                    FontSize::Small => (14.,20.),
                }
            }
            FontStyles::Body(t) | FontStyles::Monospace(t) => {
                match t.size {
                    FontSize::Large => (16.,24.),
                    FontSize::Medium => (14.,20.),
                    FontSize::Small => (12.,16.),
                }
            }
            FontStyles::Label(t) => {
                match t.size {
                    FontSize::Large => (14.,20.),
                    FontSize::Medium => (12.,16.),
                    FontSize::Small => (11.,16.),
                }
            }
            FontStyles::Name(t) => {
                (14.,20.)
            }
        };
        let font_f = font_family(&token);
        typography.insert(token, Font { font_id:FontId::new(font_size,font_f), line_height, font_size });
    };
    let bools = [true, false];
    let sizes = [FontSize::Small, FontSize::Medium, FontSize::Large];
    for &bold in &bools {
        for &italic in &bools {
            for size in &sizes {
                let t = FontVariable { bold, italic, size: size.clone() };
                update_font_id(typography, FontStyles::Display(t.clone()));
                update_font_id(typography, FontStyles::Headline(t.clone()));
                update_font_id(typography, FontStyles::Title(t.clone()));
                update_font_id(typography, FontStyles::Body(t.clone()));
                update_font_id(typography, FontStyles::Label(t.clone()));
                update_font_id(typography, FontStyles::Monospace(t.clone()));
            }
        }
    }
}
