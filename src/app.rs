use leptos::prelude::*;
use leptos_meta::{MetaTags, Title};
use bevy::prelude::*;

#[lazy]
fn run_app() {
    App::new().run();
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <WebApp />
            </body>
        </html>
    }
}

#[component]
pub fn WebApp() -> impl IntoView {
    LocalResource::new(|| run_app());
    view! {
        <Title text="Welcome to Leptos"/>
    }
}