#[component]
pub fn Frame(children: Children) -> impl IntoView {
    view! {
        <div class="oc-frame">
            <Toolbar />
            <Drawer position=DrawerPosition::Left>
                <p>"Left content"</p>
            </Drawer>
            <main class="frame-content">
                {children()}
            </main>
            <Drawer position=DrawerPosition::Right>
                <p>"Right content"</p>
            </Drawer>
        </div>
    }
}
