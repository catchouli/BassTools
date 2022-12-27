use sycamore::prelude::*;
use crate::resources::*;

/// The footer.
#[component]
pub fn Footer<G: Html>(cx: Scope) -> View<G> {
    view!(cx, footer(class="footer") {
        div(class="content has-text-centered") {
            p {
                a(href=GITHUB_URL) { (SITE_NAME) }
                " by "
                a(href=AUTHOR_URL) { (AUTHOR_NAME) }
                ". The source code is licensed "
                a(href=LICENSE_URL) { (LICENSE_NAME) }
                "."
            }
            img(id="ferris", src=FERRIS_IMAGE_PATH)
        }
    })
}
