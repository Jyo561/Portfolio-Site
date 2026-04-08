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
            <div class={container.clone()}>
                <div class={column_left.clone()}>
                    <div>
                        <h1 style="font-size: 2rem;">{"EYEWITNESS ACCOUNT: SIGNAL, SYSTEMS, SCIENCE"}</h1>
                    </div>

                    <div class={classes!("flex", "flex-col", "gap-0.5")}>
                            <p class="dropcap">
                                <span class="first-letter">{"F"}</span>{"rom September 2025, Jyotiraditya entered the halls of"}<strong>{" Indira Gandhi Centre for Atomic Research "}</strong>{"not with spectacle — but with systems."}
                            </p>
                            <p>
                                {"As a "}<strong>{"Junior Research Fellow"}</strong>{", his work sits at the intersection of computation and physical reality. Signals are not just numbers here; they are representations of electrochemical behavior, sensor drift, and system integrity. ‘The challenge wasn’t just processing data,’ he reflects. ‘It was understanding what the data meant under real-world constraints.’"}
                            </p>
                            <p>
                                {"He designed pipelines that transform raw experimental inputs into structured, interpretable outputs — filtering noise, stabilizing signals, and enabling reliable downstream analysis. His approach blends algorithmic precision with engineering pragmatism, ensuring that models are not only accurate but deployable in long-running, safety-critical environments."}
                            </p>
                            <p>
                                {"Working within tightly controlled research systems, he contributed to fault detection strategies and early-warning mechanisms, applying machine learning techniques where uncertainty is high and tolerance for error is low."}
                            </p>
                            <blockquote>
                                {"‘Validation wasn’t optional. Every assumption had to be tested, every output defensible.’"}
                            </blockquote>
                            <p>
                                {"Collaboration here was deliberate and methodical. Discussions revolved around reproducibility, robustness, and system longevity rather than speed alone. Jyotiraditya operated within these constraints, contributing to software systems that are built not just to run — but to endure."}
                            </p>
        <p>
                                {"His work reflects a shift: from building applications for users, to engineering systems that interact with the physical world — quietly, continuously, and critically."}
                            </p>
                    </div>
                </div>
                <div class={column_main.clone()}>
                    <div class={classes!("flex","justify-center","items-center","text-center","w-full")}>
                        <div>{"TIMELINE"}<br />{"Sep 2025 - Present"}</div>
                    </div>
                </div>
            </div>
        </div>
    }
}

