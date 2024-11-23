use leptos::prelude::*;
use leptos_style::Style;

#[component]
fn Button(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] style: Style,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class={class}
            id={id}
            style={style.with_defaults([
                ("padding", "0.5rem")
            ])}
        >
            {children()}
        </button>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Button
            id="button"
            style={[
                ("color", "white"),
                ("background-color",  "gray"),
                ("border", "1px solid black")
            ]}
        >
            "Click me"
        </Button>
    }
}
