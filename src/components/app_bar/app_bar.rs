use crate::utils::PushIf;
use yew::prelude::*;

pub struct AppBar {
    props: Props,
}

pub struct Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub style: String,
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
        classes.push("v-sheet v-sheet--tile theme--dark v-toolbar v-toolbar--dense v-app-bar--clipped v-app-bar--fixed red");
        html! {
            <header class=classes style=self.props.style>
                <div class="v-toolbar__content" data-booted=true>
                    { self.props.children.render() }
                </div>
            </header>
        }
    }
}
