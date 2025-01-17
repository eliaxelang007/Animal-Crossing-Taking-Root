use std::rc::Rc;

use web_sys::{wasm_bindgen::JsCast, AudioContext, AudioNode};

use gloo_timers::future::TimeoutFuture;

use leptos::{logging::log, prelude::*, task::spawn_local};
use leptos_router::{components::{Router, Routes, Route}, path}; // TODO: If this ends up being a one page app, remove the dependency on Leptos Router.

mod audio;
mod time;
mod old;

use audio::{AudioContextExtension, AudioBufferSourceNodeExtension};
use time::{Clock, TimeSpan, TimeUnit};
use old::VideoPlayer;

#[component]
fn Player() -> impl IntoView {
    spawn_local(
        async {            
            let clock = Clock::new(
                window()
                .performance()
                .expect("Couldn't find [window.performance]!")
            );

            let audio_context = AudioContext::new().expect("Couldn't create an [AudioContext]!");
            let load_audio = |name: &str| {
                let file_path = format!("/assets/{name}.oga");
                async move {
                    audio_context.load_audio(&file_path).await
                }
            };

            let load_song = |hour: u8| {
                load_audio(&hour.to_string())
            };

            let hour_bells = load_audio("hour_bells").await;

            let mut playing_hour = clock.since_epoch().as_date().get_hours();
            let mut hour_song = load_song(playing_hour as u8).await;

            loop {
                let now = clock.since_epoch();

                let since_hour = now.since_last(TimeUnit::Hour);

                if 
            }
        }
    );

    view! {
        <div>
        
        </div>
    }
}

#[component]
fn RequireInteraction(children: ChildrenFn) -> impl IntoView {
    let (interacted, set_interacted) = signal(false);

    view! {
        <Show
            when=move || { interacted.get() }
            fallback=move || { view! {
                <div on:click=move |_| { set_interacted.set(true); }>
                    <h1>"Click to start the player!"</h1>
                </div>
            } }
        >
            {children()}
        </Show>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=|| view! { 
                    <RequireInteraction>
                        <Player/>
                    </RequireInteraction> 
                }/>
                <Route path=path!("/test") view=|| view! {
                    <RequireInteraction>
                        <VideoPlayer/>
                    </RequireInteraction> 
                }/>
            </Routes>
        </Router>
    }
} 

fn main() {
    mount_to_body(|| view! { <App/> });
}