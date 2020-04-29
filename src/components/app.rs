use yew::prelude::*;

pub struct App {
    props: Props,
}

pub struct Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub is_dark: bool,
    #[prop_or_default]
    pub children: Children,
}

impl Component for App {
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
        let theme = if self.props.is_dark {
            "theme--dark"
        } else {
            "theme--light"
        };
        let rtl = if crate::settings::RTL {
            "v-application--is-rtl"
        } else {
            "v-application--is-ltr"
        };
        html! {
            <div class=("v-application", rtl, theme)>
                <div class="v-application--wrap">
                    { self.props.children.render() }
                </div>
            </div>
        }
    }
}
