use yew::prelude::*;

#[function_component(TheEnd)]
pub fn the_end() -> Html {
    html! {
        <div class={classes!("select-none", "flex", "items-center", "my-8")}>
            <div class={classes!("flex-grow", "space-y-0.5")}>
                <hr class={classes!("border-t-[5px]", "border-black/85")} />
                <hr class={classes!("border-t-[2px]", "border-black/90")} />
            </div>
            <span class={classes!("mx-2", "text-7xl", "font-OldLondon")}>{"The End"}</span>
            <div class={classes!("flex-grow", "space-y-0.5")}>
                <hr class={classes!("border-t-[5px]", "border-black/85")} />
                <hr class={classes!("border-t-[2px]", "border-black/90")} />
            </div>
        </div>
    }
}

