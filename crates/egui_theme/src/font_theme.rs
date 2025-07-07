use std::collections::BTreeMap;
use std::sync::Arc;
use epaint::{FontFamily, FontId};
use epaint::text::{FontData, FontDefinitions};
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
#[derive(Debug, Clone,PartialEq)]
pub struct FontTheme {
    pub typography: BTreeMap<TypographyStyles, Typography>,
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
impl Default for FontTheme {
    fn default() -> Self {
        let mut typography = BTreeMap::new();
        default_typography(&mut typography);
        Self { typography }
    }
}

impl FontTheme {
    pub fn typography_label(&self) -> Typography {
        self.typography.get(&TypographyStyles::label()).unwrap().clone()
    }

    pub fn typography_body(&self) -> Typography {
        self.typography.get(&TypographyStyles::body()).unwrap().clone()
    }
}
#[derive(Debug, Clone)]
pub struct ArcFontTheme(Arc<FontTheme>);

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
