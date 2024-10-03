#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

pub const TAILWIND_STYLE: &str = asset!("./assets/tailwind.css");

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        head::Link { rel: "stylesheet", href: TAILWIND_STYLE }
        Script { type: "module", src: "https://unpkg.com/@splinetool/viewer@1.9.28/build/spline-viewer.js" }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "w-screen h-screen",
            spline-viewer {
                url: "https://prod.spline.design/ZIZAOyFZJ65Ku581/scene.splinecode" }
         }
    }
}
