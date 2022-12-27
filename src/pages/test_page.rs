use std::time::Duration;

use sycamore::{prelude::*, motion::create_tweened_signal, easing};

#[component]
pub fn TestPage<'a, G: Html>(cx: Scope<'a>) -> View<G> {
    // Create counter and list signals.
    let counter = create_signal(cx, 0);
    let list = create_signal(cx, vec![]);

    // Create a tweened counter signal. I use create_effect to update it, as I'm not sure if
    // there's a shorter way to do it.
    let counter_tweened = create_tweened_signal(cx,
                          0,
                          Duration::from_millis(100),
                          easing::quad_out);

    create_effect(cx, || {
        counter_tweened.set(*counter.get());
    });

    // Create a version of the counter that's doubled.
    let counter_tweened_doubled = create_memo(cx, || *counter_tweened.get() * 2);

    view! { cx,
        section(class="block") {
            div(class="block") {
                p(class="title") { "Test Page" }
                p(class="subtitle") { "Bottom text" }
            }
            div(class="block") {
                p { "Welcome to my websight, counter value: " (counter_tweened_doubled.get()) }
            }
            div(class="block") {
                button(class="button is-primary", on:click=|_| {
                    let count = *counter.get();
                    counter.set(count + 100);
                    list.modify().push(count);
                }) { "Test" }
            }
            div(class="block") {
                (if *counter.get() == 0 {
                    view! { cx, "(no base counter value)" }
                }
                else {
                    view! { cx, "(base counter value: " (counter.get()) ")" }
                })
            }
            div(class="block") {
                Keyed(
                    iterable=list,
                    view=|cx, x| view! { cx,
                        li { (x) }
                    },
                    key=|x| *x
                )
            }
        }
    }
}
