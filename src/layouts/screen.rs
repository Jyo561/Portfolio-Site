use yew::prelude::*;
use gloo::timers::callback::Timeout;
use std::rc::Rc;
use std::cell::RefCell;
use web_sys::window;

//use yew::prelude::*;

//use yew::prelude::*;
//use gloo::timers::callback::Timeout;
//use web_sys::window;

#[derive(Properties, PartialEq)]
pub struct ScreenProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Screen)]
pub fn screen(props: &ScreenProps) -> Html {
    
    html! {
        <div class={classes!("animate")}>
            { for props.children.iter() }
        </div>
    }
}



