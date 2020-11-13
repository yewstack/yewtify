use yew::prelude::*;

pub struct ListItemAvatar {
    props: Props,
}

pub struct Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub horizontal: bool,
    #[prop_or_default]
    pub children: Children,
}

impl Component for ListItemAvatar {
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
        let mut classes = Classes::from("v-list-item-avatar");
        if self.props.horizontal { classes.push("v-list-item__avatar--horizontal"); }
        html! {
            <div class=classes>
                { self.props.children.clone() }
            </div>
        }
    }
}
