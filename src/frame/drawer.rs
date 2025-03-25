use leptos::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DrawerPosition {
    Left,
    Right,
    Top,
    Bottom,
}

#[component]
pub fn Drawer(position: DrawerPosition, children: Children) -> impl IntoView {
    let class = match position {
        DrawerPosition::Left => "drawer drawer-left",
        DrawerPosition::Right => "drawer drawer-right",
        DrawerPosition::Top => "drawer drawer-top",
        DrawerPosition::Bottom => "drawer drawer-bottom",
    };

    view! {
        <aside class={class}>
            {children()}
        </aside>
    }
}
