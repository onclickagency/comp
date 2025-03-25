use leptos::*;
use leptos_meta::Style;

const RESET_CSS: &str = include_str!("./style/reset.css");
const THEME_CSS: &str = include_str!("./style/theme.css");

#[component]
pub fn InjectTheme() -> impl IntoView {
    view! {
        <Style id="reset">{RESET_CSS}</Style>
        <Style id="theme">{THEME_CSS}</Style>
    }
}