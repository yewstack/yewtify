use yew::prelude::*;

#[mixin::declare]
#[derive(Properties, Clone, PartialEq)]
pub struct Applicationable {
    #[prop_or_default]
    pub app: bool,
}
