use crate::styles::Color;
use crate::utils::PushIf;
use yew::prelude::*;

pub struct AppBar {
    props: Props,
}

pub struct Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub color: Color,
    #[prop_or_default]
    pub children: Children,
}

impl Component for AppBar {
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
        let mut classes = Classes::from("v-app-bar");
        // TODO: Split to mixins
        classes.push("v-sheet v-sheet--tile theme--dark v-toolbar v-toolbar--dense v-app-bar--clipped v-app-bar--fixed");
        if !self.props.color.is_default() {
            classes.push(self.props.color.as_ref());
        }

        let header_style =
            "height: 48px; margin-top: 0px; transform: translateY(0px); left: 0px; right: 0px;";
        let content_style = "height: 48px;";
        html! {
            <header class=classes style=header_style>
                <div class="v-toolbar__content" style=content_style data-booted=true>
                    { self.props.children.render() }
                </div>
            </header>
        }
    }
}
