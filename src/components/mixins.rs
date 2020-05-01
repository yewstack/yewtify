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
