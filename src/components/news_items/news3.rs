use yew::prelude::*;
use stylist::style;

#[function_component(News3)]
pub fn news3() -> Html {
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
            <h1 style="font-size: 2rem;text-align: center;">{"FROM CONCEPT TO CODE: A Chronicle of Creations"}</h1>
            <div class={container.clone()}>
            <div class={classes!("flex-1", "flex", "flex-col", "gap-2")}>
                <div>
                    <h1 class={classes!("text-4xl")}>
                        {"IS STUDENT SOFTWARE THE NEXT FRONTIER?"}
                    </h1>
                    <p>{"Inside the Making of a Job Portal for Campuses"}</p>
                </div>
                <div class={classes!("flex", "gap-3", "text-sm")}>
                    <div class={classes!("flex-1", "flex", "flex-col", "gap-1")}>
                        <p class={classes!("mt-1","mb-1")}><span class="high">{"N"}</span>{"ovice developer creates seamless frontend experiences with dynamic job listings."}</p>
                        <p class={classes!("mt-1","mb-1")}><strong>{"🛠️ Tech Stack: NextJS, ExpressJS,PostgreSQL, Gemini AI"}</strong></p>
                        <p class={classes!("mt-1","mb-1")}>{"Designed responsive job listing pages and crafted reusable components for campus placement workflows. API endpoints were integrated to deliver real-time data—bridging students and recruiters in one view."}</p>
                        <p class={classes!("mt-2","mb-2")}>
                            <a href="https://placement-cell-frontend-two.vercel.app/">{"Live Preview [1]"}</a>
                            {" | "}
                            <a href="https://github.com/Jyo561/placement-cell-frontend">{" GitHub Repository [2]"}</a>
                        </p>
                        <img src="/public/IMG/placement.png" class={imgtype.clone()} />
                    </div>
                </div>
            </div>

            // Section 2
            <div class="flex-1 flex flex-col gap-2">
                <div><img src="/public/IMG/scholarship.png" alt="news_img" class={imgtype.clone()} /></div>
                <div><h1 class="text-3xl">{"WHO DESERVES A CHANCE?"}</h1><p>{"Building a Smarter Scholarship Finder for the Overlooked"}</p></div>
                <div class="flex gap-3 text-sm">
                    <div class="flex-1 flex flex-col gap-1">
                        <p class={classes!("mt-1","mb-1")}><span class="high">{"E"}</span>{"mpowering students with contextual, inclusive AI-driven search."}</p>
                        <p class={classes!("mt-1","mb-1")}><strong>{"🛠️ Tech Stack: Brython, FastAPI, Gemini AI"}</strong></p>
                        <p class={classes!("mt-1","mb-1")}>{"Developed a web application that helps students discover scholarships tailored to their caste, religion, and region using AI-powered semantic search. Leveraged Gemini AI to interpret user queries beyond keyword matching—making discovery more accessible for underserved groups. Combined client-side Python (Brython) with a FastAPI backend for lightweight deployment and responsiveness."}</p>
                        <p class={classes!("mt-2","mb-2")}>
                            <a href="https://scholarship-search.onrender.com/">{"Live Preview [3]"}</a>
                            {" | "}
                            <a href="https://github.com/Jyo561/ScholarShip-Search">{" GitHub Repository [4]"}</a>
                        </p>

                    </div>
                </div>
            </div>

            // Section 3
            <div class={classes!("flex-1", "flex", "flex-col", "gap-2")}>
                <div>
                    <h1 class={classes!("text-4xl")}>
                        {"CAN PYTHON RUN IN THE BROWSER? ONE LIBRARY SAYS YES"}
                    </h1>
                    <p>{"An Open-Source Initiative to Redefine UI Components"}</p>
                </div>
                <div class={classes!("flex", "gap-3", "text-sm")}>
                    <div class={classes!("flex-1", "flex", "flex-col", "gap-1")}>
                        <p class={classes!("mt-1","mb-1")}><span class="high">{"E"}</span>{"xploring the possibilities of frontends in Python."}</p>
                        <p class={classes!("mt-1","mb-1")}><strong>{"🛠️ Tech Stack: 🛠️ Tech Stack: Brython, HTML/CSS"}</strong></p>
                        <p class={classes!("mt-1","mb-1")}>{"Contributed responsive design improvements and cross-component standardization in a unique open-source UI library built using Python-in-the-browser. Real-time UI without JavaScript? This project says: 'Why not.'"}</p>
                        <p class={classes!("mt-2","mb-2")}>
                            <a href="https://web3-bharat-iter.github.io/UI-Component-Library/">{"Live Preview [5]"}</a>
                            {" | "}
                            <a href="https://github.com/Jyo561Python/UI-Component-Library">{" GitHub Repository [6]"}</a>
                        </p>
                        <img src="/public/IMG/UIComponent.png" class={imgtype.clone()} />
                    </div>
                </div>
            </div>

            
            </div>
        </div>
    }
}

