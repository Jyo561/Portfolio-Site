use stylist::style;

use yew::prelude::*;

#[function_component(News2)]
pub fn news2() -> Html {
    let container_style = style!(
            r#"
            display: flex; 
            gap: 0.5rem;
            "#).unwrap();
    
    let column_left_style = style!(
        r#"
        flex-basis: 80%;
        text-align: justify;
        border-right: 2px solid #3f3f46; /* zinc-700 */
        padding-right: 0.5rem;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    "#).unwrap();

    let column_main_style = style!(
            r#"
            flex-basis: 80%; 
            display: flex; 
            gap: 1.25rem;
            "#).unwrap();

    let left_main_style = style!(
            r#"
            flex-basis: 60%; 
            display: flex; 
            flex-direction: column; 
            gap: 0.75rem;
            "#).unwrap();
    
    let mid_text_block_style = style!(
            r#"
            flex: 1; 
            display: flex; 
            flex-direction: column; 
            gap: 0.5rem; 
            text-align: justify; 
            font-size: 0.875rem;"#).unwrap();
    
    let right_img_text_style = style!(
            r#"
            flex-basis: 40%; 
            border: 2px solid #3f3f46; 
            padding: 0.5rem; 
            text-align: justify; 
            display: flex; 
            flex-direction: column; 
            gap: 1rem;"#).unwrap();
    
    let tech_img_style = style!(
            r#"
            width: 24px;
            height: 24px;
            margin-right: 0.5rem;
            filter: grayscale(60%);
            "#).unwrap();
    
    let tech_img_heading_style = style!(
            r#"
            width: 35px;
            height: 35px;
            margin-right: 0.5rem;
            filter: grayscale(60%);
            "#).unwrap();

    let tech_text_heading_style = style!(
        r#"
        font-size: 30px;
        margin-top: 5px;
        font-weight: bold;
        text-transform: uppercase;
        "#).unwrap();


    let tech_text_style = style!(
        r#"
        margin-top: 2.5px;
        font-weight: bold;
        text-transform: uppercase;
        "#).unwrap();
    
    let container = container_style.get_class_name().to_string();
    let column_left = column_left_style.get_class_name().to_string();
    let column_main = column_main_style.get_class_name().to_string();
    let left_main = left_main_style.get_class_name().to_string();
    let mid_text_block = mid_text_block_style.get_class_name().to_string();
    let right_img_text = right_img_text_style.get_class_name().to_string();
    let tech_img = tech_img_style.get_class_name().to_string();
    let tech_text = tech_text_style.get_class_name().to_string();
    let tech_img_heading = tech_img_heading_style.get_class_name().to_string();
    let tech_text_heading = tech_text_heading_style.get_class_name().to_string();



    html! {
        <div>
            <div>
                <h1 style="font-size: 3rem;">{"INSIDE THE DEVELOPER'S TOOLBOX : A Glimpse at the Arsenal Behind Every Line of Code"}</h1>
            </div>
            <p class={classes!("")} style="font-size: 1.5rem;"> {"Jyotiraditya, a quiet force in the coding world, prefers craftsmanship over chaos. His toolbox is well-worn, battle-tested, and perfectly organized."} </p>
            <div class={container.clone()}>
                
                <div class={column_main.clone()}>
                    <div class={classes!("flex", "flex-col", "gap-0.5")}>
                        <p>
                            <p class={classes!("flex","justify-center","items-center")}>
                                <img src="https://raw.githubusercontent.com/devicons/devicon/master/icons/rust/rust-original.svg" alt="Rust Logo" class={tech_img_heading.clone()} />
                                <span class={tech_text_heading.clone()}>{"Rust"}</span> 
                            </p>
                            
                            <p class={classes!("flex")}>           
                                <span class={tech_text.clone()}>{"Frontend:"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <img src="https://yew.rs/img/logo.svg" alt="Yew Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"Yew"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <img src="https://sycamore.dev/logo.svg" alt="Sycamore Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"Sycamore"}</span> 
                            </p>
                            
                            <p class={classes!("flex")}>           
                                <span class={tech_text.clone()}>{"Backend:"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <img src="https://avatars.githubusercontent.com/u/106361765?s=48&v=4" alt="Rocket Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"Rocket"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <img src="https://avatars.githubusercontent.com/u/20248544?s=48&v=4" alt="Rocket Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"AXUM"}</span> 
                            </p>

                        </p>
                            
                    </div>
                </div>
                <div class={column_main.clone()}>
                    <div class={classes!("flex", "flex-col", "gap-0.5")}>
                        <p>
                            <p class={classes!("flex","justify-center","items-center")}>
                                <img src="https://raw.githubusercontent.com/devicons/devicon/master/icons/python/python-original.svg" alt="Rust Logo" class={tech_img_heading.clone()} />
                                <span class={tech_text_heading.clone()}>{"Python"}</span> 
                            </p>
                            <p class={classes!("flex")}>           
                                <span class={tech_text.clone()}>{"Frontend:"}</span> 
                            </p>

                            <p class={classes!("flex","ml-5")}>
                                <img src="https://avatars.githubusercontent.com/u/8774340?s=48&v=4" alt="Brython Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"Brython"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <img src="https://avatars.githubusercontent.com/u/106191177?s=48&v=4" alt="ReactPy Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"ReactPy"}</span> 
                            </p>
                            <p class={classes!("flex")}>           
                                <span class={tech_text.clone()}>{"Backend:"}</span> 
                            </p>

                            <p class={classes!("flex","ml-5")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/django/django-plain.svg" alt="Django Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"Django"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/fastapi/fastapi-original.svg" alt="FastAPI Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"FastAPI"}</span> 
                            </p>

                        </p>
                            
                    </div>
                </div>
                <div class={column_main.clone()}>
                    <div class={classes!("flex", "flex-col", "gap-0.5")}>
                        <p>
                            <p class={classes!("flex","justify-center","items-center")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/typescript/typescript-plain.svg" alt="TS Logo" class={tech_img_heading.clone()} />
                                <span class={tech_text_heading.clone()}>{"T-Script"}</span> 
                            </p>
                            <p class={classes!("flex")}>
                                <span class={tech_text.clone()}>{"Frontend:"}</span> 
                            </p>

                            <p class={classes!("flex","ml-5")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/react/react-original.svg" alt="React Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"React"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/vuejs/vuejs-original.svg" alt="Vue Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"Vue"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/nextjs/nextjs-original.svg" alt="Next Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"NextJS"}</span> 
                            </p>
                            <p class={classes!("flex")}>
                                <span class={tech_text.clone()}>{"Backend:"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/express/express-original.svg" alt="Express Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"ExpressJS"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/nestjs/nestjs-original.svg" alt="Nest Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"NestJS"}</span> 
                            </p>


                        </p>
                            
                    </div>
                </div> 
                <div class={column_main.clone()}>
                    <div class={classes!("flex", "flex-col", "gap-0.5")}>
                        <p>
                            <p class={classes!("flex","justify-center","items-center")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/linux/linux-original.svg" alt="Linux Logo" class={tech_img_heading.clone()} />
                                <span class={tech_text_heading.clone()}>{"Linux"}</span>
                            </p>

                            <p class={classes!("flex")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/opensuse/opensuse-original.svg" alt="openSUSE Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"OpenSUSE"}</span> 
                            </p>
                            <p class={classes!("flex")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/archlinux/archlinux-original.svg" alt="Rust Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"ArchLinux"}</span> 
                            </p>
                            <p class={classes!("flex")}>
                                <img src="https://manjaro.org/logo.svg" alt="Rust Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"Manjaro"}</span> 
                            </p>
                            <p class={classes!("flex")}>
                                <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/4/4b/EndeavourOS_Logo.svg/150px-EndeavourOS_Logo.svg.png" alt="Rust Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"Endeavour"}</span> 
                            </p>
                            <p class={classes!("flex")}>
                                <img src="https://avatars.githubusercontent.com/u/22484687?s=200&v=4" alt="Rust Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"Bliss OS"}</span> 
                            </p>

                            <p class={classes!("flex")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/debian/debian-original.svg" alt="Rust Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"ChromeOS-Debian"}</span> 
                            </p>
                            <p class={classes!("flex")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/android/android-original.svg" alt="Rust Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"Android-Termux"}</span> 
                            </p>
                        </p>
                            
                    </div>
                </div>           
                <div class={column_main.clone()}>
                    <div class={classes!("flex", "flex-col", "gap-0.5")}>
                        <p>
                            <p class={classes!("flex","justify-center","items-center")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/linux/linux-original.svg" alt="Rust Logo" class={tech_img_heading.clone()} />
                                <span class={tech_text_heading.clone()}>{"Tools"}</span>
                            </p>

                            <p class={classes!("flex")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/git/git-original.svg" alt="Rust Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"Git"}</span> 
                            </p>
                            <p class={classes!("flex")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/github/github-original.svg" alt="Rust Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"Github"}</span> 
                            </p>
                            <p class={classes!("flex")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/gitlab/gitlab-original.svg" alt="Rust Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"Gilab"}</span> 
                            </p>
                            <p class={classes!("flex")}>
                                <img src="https://upload.wikimedia.org/wikipedia/en/d/d2/Sublime_Text_3_logo.png" alt="Rust Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"Sublime-Text"}</span> 
                            </p>
                            <p class={classes!("flex")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/opencv/opencv-original.svg" alt="Rust Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"OpenCV"}</span> 
                            </p>

                            <p class={classes!("flex")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/amazonwebservices/amazonwebservices-original-wordmark.svg" alt="Rust Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"AWS"}</span> 
                            </p>
                            <p class={classes!("flex")}>
                                <img src="https://nvchad.com/logo.svg" alt="Rust Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"NvChad"}</span> 
                            </p>
                        </p>
                            
                    </div>
                </div>                       
                <div class={column_main.clone()}>
                    <div class={classes!("flex", "flex-col", "gap-0.5")}>
                        <p>
                            <p class={classes!("flex","justify-center","items-center")}>
                                <img src="https://cdn0.iconfinder.com/data/icons/essentials-solid-glyphs-vol-1/100/Data-Database-Storage-128.png" alt="Rust Logo" class={tech_img_heading.clone()} />
                                <span class={tech_text_heading.clone()}>{"Databases"}</span>
                            </p>

                            <p class={classes!("flex")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/mysql/mysql-original.svg" alt="Rust Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"MySQL"}</span> 
                            </p>
                            <p class={classes!("flex")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/postgresql/postgresql-original.svg" alt="Rust Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"PostgreSQL"}</span> 
                            </p>
                            <p class={classes!("flex")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/sqlite/sqlite-original.svg" alt="Rust Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"SQLite"}</span> 
                            </p>
                            <p class={classes!("flex")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/mongodb/mongodb-original.svg" alt="Rust Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"MongoDB"}</span> 
                            </p>
                            <p class={classes!("flex")}>
                                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/firebase/firebase-original.svg" alt="Rust Logo" class={tech_img.clone()} />
                                <span class={tech_text.clone()}>{"Firebase"}</span> 
                            </p>
                        </p>
                            
                    </div>
                </div>                       
             
            </div>
        </div>
    }
}

