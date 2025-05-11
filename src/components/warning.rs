use yew::prelude::*;
use yew::functional::*;

#[function_component(Warning)]
pub fn warning() -> Html {
    let vis = use_state(|| true);

    let toggle_visibility = {
        let vis = vis.clone();
        Callback::from(move |_| vis.set(!*vis))
    };

    if !*vis {
        return html! {};
    }

    html! {
        <section class={classes!("md:hidden", "fixed", "top-10", "z-50", "w-full", "m-2")}>
            <div class={classes!("flex", "flex-col", "items-center", "gap-4", "bg-zinc-800", "text-white", "p-4", "font-sans", "w-[50%]", "mx-auto", "border", "rounded-xl", "border-zinc-800", "shadow-2xl")}>
                <p class={classes!("text-xl", "text-center")}>
                    {"For the best UI experience, please open this website on a desktop. "}
                    {"The site contains numerous elements that may lose their visual "}
                    {"appeal if fully optimized for responsiveness."}
                </p>
                <button
                    onclick={toggle_visibility}
                    class={classes!("text-lg", "w-fit", "font-medium", "border", "px-3", "py-1", "rounded-md")}
                >
                    {"Okay"}
                </button>
            </div>
        </section>
    }
}

