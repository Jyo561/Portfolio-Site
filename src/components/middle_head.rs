use yew::prelude::*;

#[function_component(MiddleHead)]
pub fn middle_head() -> Html {
    html! {
        <div class={classes!("select-none", "flex", "items-center")}>
            <div class={classes!("text-9xl", "font-Canopee", "text-center", "bg-zinc-800", "w-full", "my-10", "text-[#E9E4DB]", "p-2")}>
                <p>{ "CAN ONE DEVELOPER MAKE A DIFFERENCE?" }</p>
            </div>
        </div>
    }
}

