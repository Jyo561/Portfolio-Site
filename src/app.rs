use yew::prelude::*;
use crate::components::{head::Head, middle_head::MiddleHead, top::Top, news::News, warning::Warning, footer::Footer};
use crate::layouts::{screen::Screen};
#[function_component]
pub fn App() -> Html {
    html! {
        <div class="App">
            <Warning />
            <Screen>
                <Top />
                <Head />
                <News />
                <Footer />
            </Screen>
        </div>
    }
}


