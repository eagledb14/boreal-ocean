use leptos::*;
use stylers::*;
// use leptos_router::*;

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {

    view! { cx, 
        <Title/>
    }
}


#[component]
fn Title(cx: Scope) -> impl IntoView {
    let title = style! {"Title",
        h1 {
            background-color: blue;
            color: white;
        }
    };

    view! { cx, class=title,
        // <h1 style="color:white; background-color:blue; ">
        <h1>
            "Boreal Ocean"
        </h1>
    }
}

