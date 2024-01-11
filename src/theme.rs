use termion::color;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Colors {
    pub numbers: Color,
    pub strings: Color,
    pub matches: Color,
    pub characters: Color,
    pub comments: Color,
    pub primary_keywords: Color,
    pub secondary_keywords: Color,
    pub texts: Color,
    pub background: Color,
    pub text_status_bar: Color,
    pub background_status_bar: Color,
    pub methods: Color,
    pub attributes: Color,
    pub separator: Color,
    pub operators: Color,
}

impl Colors {
    pub fn default() -> Self {
        Self {
            numbers: Color { r: 220, g: 163, b: 163 },
            strings: Color { r: 211, g: 54, b: 130 },
            matches: Color { r: 38, g: 139, b: 210 },
            characters: Color { r: 108, g: 113, b: 196 },
            comments: Color { r: 133, g: 153, b: 0 },
            primary_keywords: Color { r: 181, g: 137, b: 0 },
            secondary_keywords: Color { r: 42, g: 161, b: 152 },
            texts: Color { r: 200, g: 200, b: 200 },
            background: Color { r: 12, g: 12, b: 12 },
            text_status_bar: Color { r: 63, g: 63, b: 63 },
            background_status_bar: Color { r: 239, g: 239, b: 239 },
            methods: Color { r: 255, g: 0, b: 0 },
            attributes: Color { r: 243, g: 171, b: 251 },
            separator: Color { r: 0, g: 0, b: 255 },
            operators: Color { r: 0, g: 255, b: 0 },
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum ThemeType {
    None,
    Number,
    Match,
    String,
    Character,
    Comment,
    MultilineComment,
    PrimaryKeywords,
    SecondaryKeywords,
    Method,
    Separator,
    Attribute,
    Operator,
}

#[derive(PartialEq)]
pub enum Themes {
    Light,
    Dark,
    Matrix,
    Neon,
    Ocean,
    Original,
    None,
}

impl From<&str> for Themes {
    fn from(theme: &str) -> Self {
        match theme {
            "light" => Self::Light,
            "dark" => Self::Dark,
            "neon" => Self::Neon,
            "matrix" => Self::Matrix,
            "ocean" => Self::Ocean,
            "original" => Self::Original,
            _ => Self::None,
        }
    }
}

pub struct Theme {
    pub name: String,
    pub colors: Colors,
}

impl Theme {
    pub fn default() -> Self {
        get_theme_colors(Themes::Dark)
    }

    pub fn get_possibles_themes() -> String {
        let mut themes = String::new();
        themes.push_str("light, dark, neon, matrix, ocean, original");
        themes
    }

    pub fn fg_reset(&self) -> color::Rgb {
        let color = &self.colors.texts;
        color::Rgb(color.r, color.g, color.b)
    }

    pub fn bg_reset(&self) -> color::Rgb {
        let color = &self.colors.background;
        color::Rgb(color.r, color.g, color.b)
    }

    pub fn to_color(&self, &theme_type: &ThemeType) -> impl color::Color {
        let color = match theme_type {
            ThemeType::Number => &self.colors.numbers,
            ThemeType::Match => &self.colors.matches,
            ThemeType::String => &self.colors.strings,
            ThemeType::Character => &self.colors.characters,
            ThemeType::Comment | ThemeType::MultilineComment => &self.colors.comments,
            ThemeType::PrimaryKeywords => &self.colors.primary_keywords,
            ThemeType::SecondaryKeywords => &self.colors.secondary_keywords,
            ThemeType::Method => &self.colors.methods,
            ThemeType::Attribute => &self.colors.attributes,
            ThemeType::Separator => &self.colors.separator,
            ThemeType::Operator => &self.colors.operators,
            _ => &self.colors.texts,
        };

        color::Rgb(color.r, color.g, color.b)
    }

    pub fn change_theme(&mut self, theme: Themes) {
        *self = get_theme_colors(theme);
    }

    
}

pub fn get_theme_colors(theme: Themes) -> Theme {
    match theme {
        Themes::Light => Theme {
            name: String::from("light"),
            colors: Colors {
                numbers: Color { r: 80, g: 100, b: 120 },
                strings: Color { r: 50, g: 120, b: 80 },
                matches: Color { r: 0, g: 102, b: 204 },
                characters: Color { r: 70, g: 70, b: 200 },
                comments: Color { r: 128, g: 128, b: 128 },
                primary_keywords: Color { r: 0, g: 0, b: 255 },
                secondary_keywords: Color { r: 0, g: 128, b: 128 },
                texts: Color { r: 0, g: 0, b: 0 },
                background: Color { r: 220, g: 220, b: 220 },
                text_status_bar: Color { r: 0, g: 0, b: 0 },
                background_status_bar: Color { r: 180, g: 180, b: 180 },
                methods: Color { r: 255, g: 0, b: 0 },
                attributes: Color { r: 243, g: 171, b: 251 },
                separator: Color { r: 0, g: 0, b: 255 },
                operators: Color { r: 0, g: 255, b: 0 },
            },
        },
        Themes::Dark => Theme {
            name: String::from("dark"),
            colors: Colors {
                numbers: Color { r: 150, g: 150, b: 150 },
                strings: Color { r: 80, g: 180, b: 120 },
                matches: Color { r: 0, g: 204, b: 102 },
                characters: Color { r: 120, g: 120, b: 250 },
                comments: Color { r: 90, g: 90, b: 90 },
                primary_keywords: Color { r: 255, g: 100, b: 0 },
                secondary_keywords: Color { r: 0, g: 200, b: 200 },
                texts: Color { r: 255, g: 255, b: 255 },
                background: Color { r: 30, g: 30, b: 30 },
                text_status_bar: Color { r: 255, g: 255, b: 255 },
                background_status_bar: Color { r: 50, g: 50, b: 50 },
                methods: Color { r: 255, g: 0, b: 0 },
                attributes: Color { r: 243, g: 171, b: 251 },
                separator: Color { r: 0, g: 0, b: 255 },
                operators: Color { r: 0, g: 255, b: 0 },
            },
        },
        Themes::Neon => Theme {
            name: String::from("neon"),
            colors: Colors {
                numbers: Color { r: 255, g: 128, b: 0 },
                strings: Color { r: 31, g: 81, b: 255 },
                matches: Color { r: 255, g: 0, b: 255 },
                characters: Color { r: 0, g: 255, b: 255 },
                comments: Color { r: 128, g: 128, b: 128 },
                primary_keywords: Color { r: 255, g: 49, b: 49 },
                secondary_keywords: Color { r: 0, g: 255, b: 0 },
                texts: Color { r: 255, g: 255, b: 255 },
                background: Color { r: 0, g: 0, b: 0 },
                text_status_bar: Color { r: 255, g: 255, b: 255 },
                background_status_bar: Color { r: 30, g: 30, b: 30 },
                methods: Color { r: 255, g: 0, b: 0 },
                attributes: Color { r: 243, g: 171, b: 251 },
                separator: Color { r: 0, g: 0, b: 255 },
                operators: Color { r: 0, g: 255, b: 0 },
            },
        },
        Themes::Matrix => Theme {
            name: String::from("matrix"),
            colors: Colors {
                numbers: Color { r: 0, g: 150, b: 0 },
                strings: Color { r: 0, g: 100, b: 0 },
                matches: Color { r: 0, g: 200, b: 0 },
                characters: Color { r: 0, g: 120, b: 0 },
                comments: Color { r: 0, g: 80, b: 0 },
                primary_keywords: Color { r: 0, g: 170, b: 0 },
                secondary_keywords: Color { r: 0, g: 140, b: 0 },
                texts: Color { r: 0, g: 255, b: 0 },
                background: Color { r: 0, g: 20, b: 0 },
                text_status_bar: Color { r: 0, g: 180, b: 0 },
                background_status_bar: Color { r: 0, g: 30, b: 0 },
                methods: Color { r: 255, g: 0, b: 0 },
                attributes: Color { r: 243, g: 171, b: 251 },
                separator: Color { r: 0, g: 0, b: 255 },
                operators: Color { r: 0, g: 255, b: 0 },
            },
        },
        Themes::Ocean => Theme {
            name: String::from("ocean"),
            colors: Colors {
                numbers: Color { r: 100, g: 120, b: 255 },
                strings: Color { r: 0, g: 170, b: 170 },
                matches: Color { r: 255, g: 150, b: 0 },
                characters: Color { r: 200, g: 200, b: 200 },
                comments: Color { r: 80, g: 80, b: 80 },
                primary_keywords: Color { r: 0, g: 150, b: 120 },
                secondary_keywords: Color { r: 150, g: 0, b: 150 },
                texts: Color { r: 230, g: 230, b: 230 },
                background: Color { r: 30, g: 40, b: 60 },
                text_status_bar: Color { r: 180, g: 180, b: 180 },
                background_status_bar: Color { r: 50, g: 60, b: 80 },
                methods: Color { r: 255, g: 0, b: 0 },
                attributes: Color { r: 243, g: 171, b: 251 },
                separator: Color { r: 0, g: 0, b: 255 },
                operators: Color { r: 0, g: 255, b: 0 },
            },
        },
        Themes::Original | Themes::None => Theme {
            name: String::from("dark"),
            colors: Colors::default(),
        }
    }
}