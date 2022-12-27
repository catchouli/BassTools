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
        div(class="") {
            MainNavigation
            div(class="columns") {
                aside(id="side-menu") {
                    (if props.menu {
                        view! { cx, Menu }
                    } else {
                        View::empty()
                    })
                }
                section(id="content") {
                    div(class="column panel") {
                        div {
                            (props.children)
                        }
                    }
                }
            }
            Footer
        }
    )
}

/// The main navigation bar.
/// TODO: The nav bar is missing the burger menu for phones.
#[component]
pub fn MainNavigation<G: Html>(cx: Scope) -> View<G> {
    view!(cx,
        header(class="container") {
            nav(class="navbar", role="navigation", aria-label="main navigation") {
                div(class="navbar-brand") {
                    a(class="navbar-item title", href="/") {
                        "BassTools"
                    }
                    a(role="button",
                      class="navbar-burger",
                      aria-label="menu",
                      aria-expanded="false",
                      data-target="navbar")
                    {
                        span(aria-hidden="true") {}
                        span(aria-hidden="true") {}
                        span(aria-hidden="true") {}
                    }
                }
            }
            div(class="navbar-menu", id="navbar") {
                div(class="navbar-start") {
                    a(class="navbar-item", href="/") { "Home" }
                    a(class="navbar-item", href="/exercises") { "Exercises" }
                }
            }
        }
    )
}
