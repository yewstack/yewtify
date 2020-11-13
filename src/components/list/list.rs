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
        let mut classes = Classes::from("v-list");
        if self.props.dense { classes.push("v-list--dense"); }
        if self.props.disabled { classes.push("v-list--disabled"); }
        if self.props.flat { classes.push("v-list--flat"); }
        if self.props.nav { classes.push("v-list--nav"); }
        if self.props.rounded { classes.push("v-list--rounded"); }
        if self.props.shaped { classes.push("v-list--shaped"); }
        if self.props.subheader { classes.push("v-list--subheader"); }
        if self.props.two_line { classes.push("v-list--two-line"); }
        if self.props.three_line { classes.push("v-list--three-line"); }
        html! {
            <div class=classes>
                { self.props.children.clone() }
            </div>
        }
    }
}
