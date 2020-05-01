use yew::prelude::*;
use yewtify as y;

pub struct YouTubeLayout {}

pub enum Msg {}

impl Component for YouTubeLayout {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <y::App id="inspire">
                <y::NavigationDrawer app=true clipped=true>
                    <y::List dense=true>
                        <y::ListItem link=true>
                            <y::ListItemAction>
                                <y::ListItemIcon icon=y::MdiIcon::TrendingUp />
                            </y::ListItemAction>
                            <y::ListItemContent>
                                <y::ListItemTitle>
                                </y::ListItemTitle>
                            </y::ListItemContent>
                        </y::ListItem>
                        <y::SubHeader>{ "SUBSCRIPTIONS" }</y::SubHeader>
                    </y::List>
                </y::NavigationDrawer>
                <y::AppBar style="height: 48px; margin-top: 0px; transform: translateY(0px); left: 0px; right: 0px;">
                    <y::AppBarNavIcon />
                </y::AppBar>
            </y::App>
        }
    }
}
