use yew::prelude::*;

pub struct ExampleApp {}

pub enum Msg {}

impl Component for ExampleApp {
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
            <yewtify::App>
                <yewtify::NavigationDrawer>
                </yewtify::NavigationDrawer>
            </yewtify::App>
        }
    }
}
