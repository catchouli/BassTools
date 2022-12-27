mod routes;
mod components;
mod pages;

use sycamore::prelude::*;

use crate::{pages::*, components::*};

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            MainLayout(children = view! { cx,
                TestPage
            })
        }
    })
}
