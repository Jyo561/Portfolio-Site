use stylist::style;

use yew::prelude::*;

#[function_component(News4)]
pub fn news4() -> Html {
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
            flex-basis: 30%; 
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
            <div class={container.clone()}>
                <div class={column_left.clone()}>
                    <div>
                        <h1 style="font-size: 2rem;">{"EYEWITNESS ACCOUNT: CODE, COMMIT, CONQUER"}</h1>
                    </div>

                    <div class={classes!("flex", "flex-col", "gap-0.5")}>
                            <p class="dropcap">
                                <span class="first-letter">{"B"}</span>{"etween November 2024 and January 2025, Jyotiraditya stepped into"}<strong>{" SubNub Technologies "}</strong>{"not with a roar — but with code."}
                            </p>
                            <p>
                                {"As a "}<strong>{"Software Developer Intern"}</strong>{", his days were spent bridging vision and execution. 'The goal wasn’t just to make something that works,' he notes. 'It had to "}<em>{"feel right"}</em>{" — responsive, accessible, and visually clean.' "}
                            </p>
                            <p>
                                {"He tackled browser quirks, streamlined UI behaviors, and integrated REST APIs into sleek, intuitive frontends. His work, though remote, never felt disconnected."}
                            </p>
                            <blockquote>
                                {"'Slack was my office door, GitHub my chalkboard.'"}
                            </blockquote>
                            <p>
                                {"Working shoulder to shoulder with backend engineers, Jyotiraditya helped standardize UI/UX practices, participated in team-wide code reviews, and left behind components built to scale — and stand."}
                            </p>
                    </div>
                </div>
                <div class={column_main.clone()}>
                    <div class={classes!("flex","justify-center","items-center","text-center","w-full")}>
                        <div>{"TIMELINE"}<br />{"Nov 2024 - Jan 2025"}</div>
                    </div>
                </div>
            </div>
        </div>
    }
}

