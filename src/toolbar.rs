use leptos::prelude::*;

#[component]
pub fn Toolbar() -> impl IntoView {
    view! { cx,
        <header style="background: #222; color: white; padding: 1rem;">
            <h1>"Test"</h1>
        </header>
    }
}
