use yew::prelude::*;
use gloo::timers::callback::Interval;
use std::rc::Rc;
use std::cell::RefCell;
use chrono::Local;

#[function_component(Clock)]
pub fn clock() -> Html {
    let time = use_state(|| get_time_string());

    {
        let time = time.clone();
        use_effect_with((), move |_| {
            let interval = Interval::new(1000, move || {
                time.set(get_time_string());
            });

            // Cleanup on unmount
            move || drop(interval)
        });
    }

    html! {
        <div>{ (*time).clone() }</div>
    }
}

fn get_time_string() -> String {
    Local::now().format("%I:%M %p").to_string() // 12-hour format
}

