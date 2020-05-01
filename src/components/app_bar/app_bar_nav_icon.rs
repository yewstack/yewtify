use crate::utils::PushIf;
use yew::prelude::*;

pub struct AppBarNavIcon {
    // TODO: Use `Button` component internally. (calculate and get classes)
    props: Props,
}

pub struct Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

impl Component for AppBarNavIcon {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("v-app-bar-nav-icon");
        // TODO: Add classes from `Button`
        html! {
            <div class=classes />
        }
    }
}
