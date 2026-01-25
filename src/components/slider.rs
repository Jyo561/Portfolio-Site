use yew::prelude::*;

#[function_component(Slider)]
pub fn slider() -> Html {
    html! {
        <div class={classes!("marquee-container", "select-none")}>
            <div class={classes!("marquee")}>
                {"Interesting Isn't It? "}
                <a href="https://linktr.ee/jyotiradityakuanar34" target="_blank">
                    {"Follow Him"}
                </a>
                {"."}
            </div>
        </div>
    }
}

