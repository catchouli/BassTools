use sycamore::prelude::*;
use crate::components::*;

/// Properties for the main layout component.
#[derive(Props)]
pub struct MainLayoutProps<G: Html> {
    children: View<G>,

    /// Whether or not to include the side menu.
    #[prop(default=true)]
    menu: bool,
}

/// The main layout component.
#[component]
pub fn MainLayout<G: Html>(cx: Scope, props: MainLayoutProps<G>) -> View<G> {
    view!(cx,
        div(class="main-layout") {
            MainNavigation
            div(id="content-container", class="columns") {
                aside(id="side-menu", class="column is-narrow") {
                    (if props.menu {
                        view! { cx, Menu }
                    } else {
                        View::empty()
                    })
                }
                section(id="content", class="column panel") {
                    div {
                        (props.children)
                    }
                }
            }
            Footer
        }
    )
}
