use yew::prelude::*;

#[mixin::declare]
#[derive(Properties, Clone, PartialEq)]
pub struct Applicationable {
    #[prop_or_default]
    pub app: bool,
}

#[mixin::declare]
pub struct Themeable {}

#[mixin::expand]
impl Themeable {
    pub fn is_dark(&self) -> bool {
        crate::settings::DARK
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
