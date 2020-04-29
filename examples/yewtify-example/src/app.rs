use yew::prelude::*;

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
            <yewtify::App id="inspire">
                <yewtify::NavigationDrawer app=true clipped=true>
                    <yewtify::List dense=true>
                        /*
                        <yewtify::ListItem>
                            <yewtify::ListItemAction>
                                <yewtify::Icon />
                            </yewtify::ListItemAction>
                            <yewtify::ListItemContent>
                            </yewtify::ListItemContent>
                        </yewtify::ListItem>
                        */
                    </yewtify::List>
                </yewtify::NavigationDrawer>
            </yewtify::App>
        }
    }
}
