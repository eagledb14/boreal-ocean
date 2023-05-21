use leptos::*;
// use stylers::*;
//use leptos_meta::*;
// use leptos_router::*;

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    // let (count, set_count) = create_signal(cx, 0);
    // provide_meta_context(cx);

    view! { cx, 
        // <Stylesheet id="leptos" href="style.css"/>
        // <h1> "Hi There" </h1>
    
        <Title/>
    }
}


#[component]
fn Title(cx: Scope) -> impl IntoView {
    // let styles = style!{"Title",
    //     h1 {
    //         background-color: blue;
    //         color: while;
    //         padding: 100px;
    //     }
    // };

    // view! { cx, class = styles, 
    view! { cx, 
        <h1>
            "Boreal Ocean"
        </h1>
    }
}

