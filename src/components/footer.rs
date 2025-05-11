use yew::prelude::*;
use crate::components::slider::Slider; // adjust path as needed

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class={classes!("select-none")}>
            <Slider />
            <div class={classes!("py-7", "font-Canopee", "text-2xl", "flex", "flex-row", "justify-between", "px-6")}>
                <a
                    class={classes!("hover:opacity-75", "transition-colors", "duration-300")}
                    href="#"
                    target="_blank"
                >
                    {"Jyotiraditya Kuanar ©"}
                </a>

                <div class={classes!("flex", "gap-2")}>
                    <a
                        class={classes!("hover:opacity-75", "transition-colors", "duration-300")}
                        href="https://x.com/Jyotiradit85974"
                        target="_blank"
                    >
                        {"Twitter"}
                    </a>
                    <span>{"•"}</span>
                    <a
                        class={classes!("hover:opacity-75", "transition-colors", "duration-300")}
                        href="https://github.com/Jyo561"
                        target="_blank"
                    >
                        {"Github"}
                    </a>
                     <span>{"•"}</span>
                    <a
                        class={classes!("hover:opacity-75", "transition-colors", "duration-300")}
                        href="https://gitlab.com/jyotiradityakuanar34/"
                        target="_blank"
                    >
                        {"Gitlab"}
                    </a>

                    <span>{"•"}</span>
                    <a
                        class={classes!("hover:opacity-75", "transition-colors", "duration-300")}
                        href="https://www.linkedin.com/in/jyotiraditya-kuanar-8b39b7207/"
                        target="_blank"
                    >
                        {"LinkedIn"}
                    </a>
                </div>
            </div>
        </div>
    }
}

