use yew::prelude::*;

pub struct Btn {
    props: Props,
}

#[mixin::insert(Themeable, Sizeable, Positionable, Elevatable)]
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub block: bool,
    #[prop_or_default]
    pub depressed: bool,
    #[prop_or_default]
    pub fab: bool,
    #[prop_or_default]
    pub icon: bool,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub rounded: bool,
    #[prop_or_default]
    pub text: bool,
    #[prop_or_default]
    pub tile: bool,
    #[prop_or_default]
    pub children: Children,
}

impl Component for Btn {
    type Message = ();
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
        let mut classes = Classes::from("v-btn theme--dark")
            .extend(self.props.theme_classes())
            .extend(self.props.sizeable_classes())
            .extend(self.props.elevation_classes());
        let flat = self.props.icon || self.props.text || self.props.outlined;
        let contained = !flat && !self.props.depressed && !self.props.has_elevation();
        let round = self.props.icon || self.props.fab;
        classes.push_if(self.props.absolute, "v-btn--absolute");
        classes.push_if(self.props.block, "v-btn--block");
        classes.push_if(self.props.bottom, "v-btn--bottom");
        classes.push_if(contained, "v-btn--contained");
        classes.push_if(
            self.props.depressed || self.props.outlined,
            "v-btn--depressed",
        );
        classes.push_if(self.props.fab, "v-btn--fab");
        classes.push_if(self.props.fixed, "v-btn--fixed");
        classes.push_if(flat, "v-btn--flat");
        classes.push_if(self.props.icon, "v-btn--icon");
        classes.push_if(self.props.loading, "v-btn--loading");
        classes.push_if(self.props.left, "v-btn--left");
        classes.push_if(self.props.outlined, "v-btn--outlined");
        classes.push_if(self.props.right, "v-btn--right");
        classes.push_if(round, "v-btn--round");
        classes.push_if(self.props.rounded, "v-btn--rounded");
        classes.push_if(self.props.text, "v-btn--text");
        classes.push_if(self.props.tile, "v-btn--tile");
        classes.push_if(self.props.top, "v-btn--top");

        html! {
            <button type="button" class=classes>
                <span class="v-btn__content">
                    { self.props.children.render() }
                    {
                        if self.props.loading {
                            html!{ <span class="v-btn__loader">/* TODO <ProgressCircular/> */</span> }
                        } else {
                            html!{}
                        }
                    }
                </span>
            </button>
        }
    }
}
