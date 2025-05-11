use yew::prelude::*;

#[function_component(Head)]
pub fn head() -> Html {
    html! {
        <div class="select-none">
            // Top horizontal lines with "the" in the center
            <div class={classes!("flex", "items-center")}>
                <div class={classes!("flex-grow", "space-y-0.5")}>
                    <hr class={classes!("border-t-[5px]", "border-black/85")} />
                    <hr class={classes!("border-t-[2px]", "border-black/90")} />
                </div>
                <span class={classes!("mx-2", "text-5xl", "font-OldLondon", "text-themeOrange")}>
                    { "Software Developer" }
                </span>
                <div class={classes!("flex-grow", "space-y-0.5")}>
                    <hr class={classes!("border-t-[5px]", "border-black/85")} />
                    <hr class={classes!("border-t-[2px]", "border-black/90")} />
                </div>
            </div>

            // Title and tagline
            <div class={classes!("flex", "flex-col", "gap-4", "items-center", "pb-6")}>
                <div class={classes!("text-8xl", "font-semibold", "font-OldLondon", "text-black/90")}>
                    { "Jyotiraditya Kuanar" }
                </div>
                <div class={classes!("font-EditorialNew")}>
                    { "Working on innovative projects using Off Stack Technologies" }
                </div>
            </div>

            // Bottom horizontal lines
            <div class={classes!("flex", "flex-col", "space-y-0.5")}>
                <hr class={classes!("border-t-[2px]", "border-black/90")} />
                <hr class={classes!("border-t-[5px]", "border-black/85")} />
            </div>
        </div>
    }
}

