use crate::utils::PushIf;
use yew::prelude::*;

pub struct App {
    props: Props,
}

pub struct Msg {}

#[mixin::insert(Themeable, RightToLeft)]
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub id: Option<String>,
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
        let classes = Classes::from("v-application");
        let mut classes = classes.extend(self.props.theme_classes());
        classes.push_if_or(
            self.props.is_rtl(),
            "v-application--is-rtl",
            "v-application--is-ltr",
        );
        html! {
            <div class=classes>
                <div class="v-application--wrap">
                    { self.props.children.render() }
                </div>
            </div>
        }
    }
}
