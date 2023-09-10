use leptos::*;
use leptos_dom::IntoView;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_use::{use_interval_fn_with_options, utils::Pausable, UseIntervalFnOptions};
use mrvillage_ui::{Button, ButtonColor, NumberInput};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/timer.css" />
        <Stylesheet id="mrvillage-ui" href="/mrvillage-ui.css" />
        <Title text="Timer" />
        <div class="dark">
            <main class="text-center pt-4 min-h-[100vh] w-full !max-w-full mu-main-bg mu-prose">
            </main>
        </div>
    }
}
