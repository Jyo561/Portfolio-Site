use yew::prelude::*;
use chrono::Local;
use chrono::Datelike;
use crate::utils::clock::Clock;
//use crate::components::icons::{TwitterIcon, FacebookIcon, InstagramIcon, GlobeIcon};

#[function_component(Top)]
pub fn top() -> Html {
    let now = Local::now();
    let day = now.weekday().num_days_from_sunday() as usize;
    let month = now.month0() as usize;
    let date = now.day();
    let year = now.year();
        
    let day_arr = vec![
        "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"
    ];
    let month_arr = vec![
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
    ];

    html! {
        <div class={classes!("select-none", "text-black/90", "font-Helvetica", "text-xs", "flex", "flex-row", "justify-between", "items-center")}>
            <div class={classes!("flex", "flex-col", "items-start")}>
                <span class={classes!("text-black/75", "text-base", "font-semibold", "hover:underline", "cursor-pointer")}>
                    <a href="mailto:jyotiradityakuanar34@gmail.com">{ "jyotiradityakuanar34@gmail.com" }</a>
                </span>
                
                <span class={classes!("text-black/75", "text-base", "font-semibold", "cursor-pointer")}>

                    { "Find the resume at:- " }<a href="https://drive.google.com/file/d/1vEk5QPl4zTyPQ8G67X_a6LBV31TiviNq/view?usp=sharing" class={classes!("hover:text-themeOrange","hover:underline")}>{"Resume"}</a>
                </span>
            </div>
            <div class={classes!("flex", "flex-col", "items-end")}>
                <span class={classes!("text-black/75", "font-semibold", "text-sm")}>
                    { "Bhubaneswar, Odisha, India" }
                </span>
                <span class={classes!("text-black/80", "font-semibold", "text-base")}>
                    { format!("{}, {} {}, {}", day_arr[day], month_arr[month], date, year) }
                </span>
                <span class={classes!("flex", "flex-row", "gap-1", "font-sans", "font-semibold")}>
                    { "Local Time " }
                    <Clock />
                </span>
            </div>
        </div>
    }
}

