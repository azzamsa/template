use dioxus::prelude::*;

pub fn home(cx: Scope) -> Element {
    let count = use_state(cx, || 0);

    cx.render(rsx!(
        section { class: "my-0 mx-auto max-w-3xl text-center",
            h2 { class: "p-6 text-4xl",
                "Welcome to Dioxus with Tailwind"
            }
            p { class: "px-10 pb-10 text-left",
                "Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."
            }
            button {
                class: "py-3 px-5 text-white bg-amber-600 rounded-lg hover:bg-sky-700",
                onclick: move |_| *count.make_mut() += 1,
                "Something's here | ",
                if *count.get() == 0 { "Click me!".to_string() } else { count.get().to_string() }
            }
        }
    ))
}
