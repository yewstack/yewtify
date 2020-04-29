use crate::utils::PushIf;
use yew::prelude::*;

pub struct List {
    props: Props,
}

pub struct Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub dense: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub flat: bool,
    #[prop_or_default]
    pub nav: bool,
    #[prop_or_default]
    pub rounded: bool,
    #[prop_or_default]
    pub shaped: bool,
    #[prop_or_default]
    pub subheader: bool,
    #[prop_or_default]
    pub two_line: bool,
    #[prop_or_default]
    pub three_line: bool,
    #[prop_or_default]
    pub children: Children,
}

impl Component for List {
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
        let mut classes = vec!["v-list"];
        classes.push_if(self.props.dense, "v-list--dense");
        classes.push_if(self.props.disabled, "v-list--disabled");
        classes.push_if(self.props.flat, "v-list--flat");
        classes.push_if(self.props.nav, "v-list--nav");
        classes.push_if(self.props.rounded, "v-list--rounded");
        classes.push_if(self.props.shaped, "v-list--shaped");
        classes.push_if(self.props.subheader, "v-list--subheader");
        classes.push_if(self.props.two_line, "v-list--two-line");
        classes.push_if(self.props.three_line, "v-list--three-line");
        html! {
            <div class=classes>
                { self.props.children.render() }
            </div>
        }
    }
}
