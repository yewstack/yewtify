use strum::AsRefStr;
use yew::Classes;

#[derive(AsRefStr, Debug, Clone, PartialEq, Eq, Hash)]
#[strum(serialize_all = "kebab-case")]
pub enum Color {
    Red,
    Pink,
    Purple,
    DeepPurple,
    Indigo,
    Blue,
    LightBlue,
    Cyan,
    Teal,
    Green,
    LightGreen,
    Lime,
    Yellow,
    Amber,
    Orange,
    DeepOrange,
    Brown,
    BlueGrey,
    Grey,
    Black,
    White,
    Transparent,
    Default,
}

impl Color {
    pub fn is_default(&self) -> bool {
        *self == Self::Default
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(AsRefStr, Debug, Clone, PartialEq, Eq, Hash)]
#[strum(serialize_all = "kebab-case")]
pub enum Intensity {
    Lighten5,
    Lighten4,
    Lighten3,
    Lighten2,
    Lighten1,
    Darken1,
    Darken2,
    Darken3,
    Darken4,
    Accent1,
    Accent2,
    Accent3,
    Accent4,
}

pub struct ColorSet {
    pub bg_tone: Option<Color>,
    pub bg_intensity: Option<Intensity>,
    pub text_tone: Option<Color>,
    pub text_intensity: Option<Intensity>,
}

impl Into<Classes> for ColorSet {
    fn into(self) -> Classes {
        let mut classes = Classes::new();
        if let Some(bg_tone) = self.bg_tone {
            let value = bg_tone.as_ref();
            classes.push(value);
        }
        if let Some(bg_intensity) = self.bg_intensity {
            let value = bg_intensity.as_ref();
            classes.push(value);
        }
        todo!();
        classes
    }
}

/*
declare_tone! {
    Red Lighten Darken Accent,
    Pink Lighten Darken Accent,
    Purple Lighten Darken Accent,
    DeepPurple Lighten Darken Accent,
    Indigo Lighten Darken Accent,
    Blue Lighten Darken Accent,
    LightBlue Lighten Darken Accent,
    Cyan Lighten Darken Accent,
    Teal Lighten Darken Accent,
    Green Lighten Darken Accent,
    LightGreen Lighten Darken Accent,
    Lime Lighten Darken Accent,
    Yellow Lighten Darken Accent,
    Amber Lighten Darken Accent,
    Orange Lighten Darken Accent,
    DeepOrange Lighten Darken Accent,
    Brown Lighten Darken Accent,
    BlueGrey Lighten Darken Accent,
    Grey Lighten Darken,
    Black,
    White,
    Transparent,
}
*/
