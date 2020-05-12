use yew::prelude::*;

#[mixin::declare]
#[derive(Properties, Clone, PartialEq)]
pub struct Applicationable {
    #[prop_or_default]
    pub app: bool,
}

#[mixin::declare]
#[derive(Properties, Clone, PartialEq)]
pub struct Themeable {
    #[prop_or_default]
    pub dark: bool,
    #[prop_or_default]
    pub light: bool,
}

#[mixin::expand]
impl Themeable {
    pub fn is_dark(&self) -> bool {
        if self.dark {
            return true;
        } else if self.light {
            return false;
        } else {
            // Get default if extplcit value not set.
            crate::settings::DARK
        }
    }

    pub fn theme_classes(&self) -> Classes {
        if self.is_dark() {
            Classes::from("theme--dark")
        } else {
            Classes::from("theme--light")
        }
    }
}

#[mixin::declare]
pub struct RightToLeft {}

#[mixin::expand]
impl RightToLeft {
    pub fn is_rtl(&self) -> bool {
        crate::settings::RTL
    }
}

#[mixin::declare]
#[derive(Properties, Clone, PartialEq)]
pub struct Sizeable {
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub small: bool,
    #[prop_or_default]
    pub x_large: bool,
    #[prop_or_default]
    pub x_small: bool,
}

#[mixin::expand]
impl Sizeable {
    pub fn sizeable_classes(&self) -> Classes {
        let mut classes = Classes::new();
        let medium = !self.x_small && ! self.small && !self.large && !self.x_large;
        classes.push_if(self.large, "v-size--large");
        classes.push_if(medium, "v-size--default");
        classes.push_if(self.small, "v-size--small");
        classes.push_if(self.x_large, "v-size--x-large");
        classes.push_if(self.x_small, "v-size--x-small");
        return classes;
    }
}

#[mixin::declare]
#[derive(Properties, Clone, PartialEq)]
pub struct Positionable {
    #[prop_or_default]
    pub absolute: bool,
    #[prop_or_default]
    pub bottom: bool,
    #[prop_or_default]
    pub fixed: bool,
    #[prop_or_default]
    pub left: bool,
    #[prop_or_default]
    pub right: bool,
    #[prop_or_default]
    pub top: bool,
}

#[mixin::declare]
#[derive(Properties, Clone, PartialEq)]
struct Elevatable {
    #[prop_or(-1)]
    pub elevation: i8,
}

#[mixin::expand]
impl Elevatable {
    pub fn has_elevation(&self) -> bool {
        0 <= self.elevation && self.elevation <= 24
    }
    pub fn elevation_classes(&self) -> Classes {
        if self.has_elevation() {
            Classes::from(format!("elevation-{}", self.elevation))
        } else {
            Classes::new()
        }
    }
}
