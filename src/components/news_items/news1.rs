use yew::prelude::*;
use stylist::style;

#[function_component(News1)]
pub fn news1() -> Html {
    let container_style = style!(
        r#"
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        gap: 1.25rem;
        margin-top: 2rem;
        margin-bottom: 2rem;
    "#).unwrap();

    let title_style = style!(
        r#"
        font-family: 'Times New Roman', serif;
        font-weight: 800;
        font-size: 3rem;
        padding-bottom: 0.75rem;
    "#).unwrap();

    let subheading_style = style!(
        r#"
        font-size: 1.25rem; 
        font-weight: bold;
        "#).unwrap();
    
    let section_style = style!(
        r#" 
        display: flex; 
        flex-direction: column; 
        gap: 0.5rem;"#).unwrap();
    
    let section_style2 = style!(
        r#"
        flex: 1; 
        display: flex; 
        flex-direction: column; 
        gap: 1rem;"#
    ).unwrap();

    let container = container_style.get_class_name().to_string();
    let title = title_style.get_class_name().to_string();
    let subheading = subheading_style.get_class_name().to_string();
    let section = section_style.get_class_name().to_string();
    let section2 = section_style2.get_class_name().to_string();

    html! {
        <div class={container.clone()}>
            // Left Column
            <div class={classes!("basis-1/2")}>
                <div class={title.clone()}>
                    {"WHO IS JYOTIRADITYA?"} 
                    <span class={classes!("not-italic", "font-bold")}>{" A CLOSER LOOK"}</span>
                </div>
                <div class={classes!("flex", "flex-row", "gap-3", "text-sm", "text-justify")}>
                    // First Text Block
                    <div class={section2.clone()}>
                        <div class={section.clone()}>
                            <p>
                                <span class={classes!("high")}>{"J"}</span>
                                {"yotiraditya Kuanar is a passionate full-stack developer who thrives on building projects from the ground up using a wide range of technologies. He enjoys the challenge of creating complete systems — handling everything from backend infrastructure to frontend interfaces — and believes in learning by doing."}
                            </p>
                            <p>
                                {"What sets him apart is his commitment to exploring unconventional tech stacks and his natural drive to build projects entirely from scratch. He's not just interested in writing code — he’s passionate about crafting well-thought-out, end-to-end solutions."}
                            </p>
                        </div>
                    </div>
                    <div class={section2.clone()}>
                        <div class={section.clone()}>
                            <p>
                                {"Although he hasn't formally worked in the cybersecurity domain, Jyotiraditya is a deeply motivated cybersecurity enthusiast. The principles of security, precision, and resilience heavily influence his development mindset, and continue to inspire the projects he takes on."}
                            </p>
                            <p>
                                {"What sets him apart is his commitment to exploring unconventional tech stacks and his natural drive to build projects entirely from scratch. He's not just interested in writing code — he’s passionate about crafting well-thought-out, end-to-end solutions."}
                            </p>
                        </div>
                    </div>
                    // Second Text Block with Image
                    
                    // Third Text Block
                </div>
            </div>

            // Right Column with image and two text blocks
            <div class={classes!("flex", "flex-col", "gap-4", "basis-1/2")}>
                <img src="/public/IMG/crimeGIFS/a-3.gif" style="height: 500px; object-fit: cover;" />
            </div>
        </div>
    }
}

