use crate::mdi_icon::MdiIcon;
use yew::prelude::*;

pub struct Icon {
    props: Props,
}

pub struct Msg {}

#[mixin::insert(Themeable, RightToLeft)]
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub icon: MdiIcon,
    #[prop_or_default]
    pub large: bool,
}

impl Component for Icon {
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
        let mut classes = Classes::from("v-icon");
        classes.push("mdi");
        classes.push(self.props.icon.as_ref());
        // TODO: How `large` attribute should work?
        html! {
            <i class=classes />
        }
    }
}
