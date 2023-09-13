use leptos::*;
use leptos_dom::IntoView;
use leptos_meta::{provide_meta_context, Stylesheet, Title};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/$$-cli-crate-name-$$.css" />
        <Stylesheet id="mrvillage-ui" href="/mrvillage-ui.css" />
        <Title text="Timer" />
        <div class="dark">
            <main class="text-center pt-4 min-h-[100vh] w-full !max-w-full mu-main-bg mu-prose">
            </main>
        </div>
    }
}
