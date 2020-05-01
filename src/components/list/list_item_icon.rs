use crate::mdi_icon::MdiIcon;
use yew::prelude::*;

pub struct ListItemIcon {
    props: Props,
}

pub struct Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub icon: MdiIcon,
}

impl Component for ListItemIcon {
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
        classes.push("notranslate");
        classes.push("mdi");
        classes.push(self.props.icon.as_ref());
        html! {
            <i class=classes aria-hidden=true />
        }
    }
}
