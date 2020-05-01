use yew::prelude::*;

pub struct NavigationDrawer {
    props: Props,
}

pub struct Msg {}

#[mixin::insert(Applicationable)]
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub absolute: bool,
    #[prop_or_default]
    pub bottom: bool,
    #[prop_or_default]
    pub clipped: bool,
    #[prop_or_default]
    pub right: bool,
    #[prop_or_default]
    pub temporary: bool,
    #[prop_or_default]
    pub children: Children,
}

impl Component for NavigationDrawer {
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
        let mut classes = Classes::new();
        classes.push("v-navigation-drawer");
        classes.push_if(self.props.absolute, "v-navigation-drawer--absolute");
        classes.push_if(self.props.bottom, "v-navigation-drawer--bottom");
        classes.push_if(self.props.clipped, "v-navigation-drawer--clipped");
        /*
        "v-navigation-drawer--close": !this.isActive,
        */
        //"v-navigation-drawer--fixed": !this.absolute && (this.app || this.fixed),
        /*
        "v-navigation-drawer--floating": this.floating,
        "v-navigation-drawer--is-mobile": this.isMobile,
        "v-navigation-drawer--is-mouseover": this.isMouseover,
        "v-navigation-drawer--mini-variant": this.isMiniVariant,
        "v-navigation-drawer--custom-mini-variant": Number(this.miniVariantWidth) !== 56,
        "v-navigation-drawer--open": this.isActive,
        "v-navigation-drawer--open-on-hover": this.expandOnHover,
        */
        classes.push_if(self.props.right, "v-navigation-drawer--right");
        classes.push_if(self.props.temporary, "v-navigation-drawer--temporary");
        html! {
            <div class=classes>
                <div class="v-navigation-drawer__content">
                    { self.props.children.render() }
                </div>
                <div class="v-navigation-drawer__border" />
            </div>
        }
    }
}
