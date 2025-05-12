use yew::prelude::*;
use stylist::style;

#[function_component(News5)]
pub fn news5() -> Html {
    let container_style = style!(
        r#"
        margin-top: 1rem;
        display: flex;
        flex-direction: row;
        gap: 1.25rem;
        text-align: justify;
        "#).unwrap();
    
    let imgtype_style = style!(r#"
        border-radius: 20px;
        filter: grayscale(80%);
        "#).unwrap(); 

    let container =  container_style.get_class_name().to_string();
    let imgtype = imgtype_style.get_class_name().to_string();    

    html! {
        <div >
            // Section 1
            <h1 style="font-size: 2rem;text-align: center;">{"FROM THE LAB OF JYOTIRADITYA"}</h1>
            <div class={container.clone()}>
            <div class={classes!("flex-1", "flex", "flex-col", "gap-2")}>
                <div>
                    <h1 class={classes!("text-4xl","mb-1")}>
                        {"IS MACHINE VISION READY FOR EQUATIONS?"}
                    </h1>
                    <p>{"Inside the Experimental MathOCR Engine"}</p>
                </div>
                <div class={classes!("flex", "gap-3", "text-sm")}>
                    <div class={classes!("flex-1", "flex", "flex-col", "gap-1")}>
                        <p class={classes!("mt-1","mb-1")}><span class="high">{"A"}</span>{" vision-powered tool deciphers handwritten math with OpenCV and Python."}</p>
                        <p class={classes!("mt-1","mb-1")}><strong>{"🛠️ Tech Stack: OpenCV, Python"}</strong></p>
                        <p class={classes!("mt-1","mb-1")}>{"Developed a symbol segmentation and recognition prototype that extracts and interprets handwritten math expressions from scanned images. A small step toward machine-read math education."}</p>
                        <p class={classes!("mt-2","mb-2")}>
                            <a href="https://github.com/Jyo561/placement-cell-frontend">{" GitHub Repository [7]"}</a>
                        </p>
                    </div>
                </div>
            </div>

            // Section 2
            <div class={classes!("flex-1", "flex", "flex-col", "gap-2")}>
                <div><h1 class={classes!("text-3xl","mb-1")}>{"WILL RUST TAKE OVER CHAT AUTOMATION?"}</h1><p>{"Behind the Scenes of a Discord Expense Bot"}</p></div>
                <div class={classes!("flex", "gap-3", "text-sm")}>
                    <div class={classes!("flex-1", "flex", "flex-col", "gap-1")}>
                        <p class={classes!("mt-1","mb-1")}><span class="high">{"R"}</span>{"ust meets personal finance in this fast, safe automation tool."}</p>
                        <p class={classes!("mt-1","mb-1")}><strong>{"🛠️ Tech Stack: Rust, Google Sheets API"}</strong></p>
                        <p class={classes!("mt-1","mb-1")}>{"Engineered a Discord bot that logs expenses via DMs and syncs with Google Sheets. Emphasis on low-latency NLP processing and memory-safe code execution."}</p>
                        <p class={classes!("mt-2","mb-2")}>
                            <a href="https://github.com/Jyo561/DiscordBot">{" GitHub Repository [8]"}</a>
                        </p>

                    </div>
                </div>
            </div>

            // Section 3
            <div class={classes!("flex-1", "flex", "flex-col", "gap-2")}>
                <div>
                    <h1 class={classes!("text-4xl")}>
                        {"CAN CODE BECOME ART?"}
                    </h1>
                    <p>{"Exploring Generative Creativity with Rust"}</p>
                </div>
                <div class={classes!("flex", "gap-3", "text-sm")}>
                    <div class={classes!("flex-1", "flex", "flex-col", "gap-1")}>
                        <p class={classes!("mt-1","mb-1")}><span class="high">{"C"}</span>{"ode-based brushstrokes render visual randomness into form."}</p>
                        <p class={classes!("mt-1","mb-1")}><strong>{"🛠️ Tech Stack: Rust, image-rs"}</strong></p>
                        <p class={classes!("mt-1","mb-1")}>{"Created a generative art engine that crafts patterns using curves, randomness, and symmetry—all written in performant Rust. Focused on minimalism and visual storytelling through pixels."}</p>
                        <p class={classes!("mt-2","mb-2")}>
                            <a href="https://github.com/Jyo561/ArtGenerator-Rust">{" GitHub Repository [9]"}</a>
                        </p>
                    </div>
                </div>
            </div>

            
            </div>
        </div>
    }
}

