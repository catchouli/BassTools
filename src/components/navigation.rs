use sycamore::prelude::*;
use crate::resources::*;

/// The main navigation bar.
/// TODO: The nav bar is missing the burger menu for phones.
#[component]
pub fn MainNavigation<G: Html>(cx: Scope) -> View<G> {
    view!(cx,
        header {
            nav(class="navbar", role="navigation", aria-label="main navigation") {
                div(class="navbar-brand") {
                    a(id="site-logo", class="navbar-item title", href="/") {
                        img(src=SITE_LOGO)
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
