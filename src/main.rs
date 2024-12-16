// mod audio;
// mod time;

// use audio::AudioContext;
// use time::Clock;

// use leptos::{
//     prelude::*,
//     ev,
//     logging::log
// };

// use leptos_router::{components::{Route, Router, Routes}, path};
// use leptos_use::use_event_listener;

// use wasm_bindgen_futures::{JsFuture, spawn_local};
// use web_sys::{js_sys::Date, HtmlAudioElement};

// use std::sync::{Arc, Mutex};
// use std::ops::Sub;

// #[derive(Clone)]
// struct HtmlAudioElementWrapper(HtmlAudioElement);

// unsafe impl Send for HtmlAudioElementWrapper {}
// unsafe impl Sync for HtmlAudioElementWrapper {}

// trait FloatEpsilonEq where 
//     Self: Sized + PartialOrd,
//     for <'a> &'a Self: Sub<Output = Self> {
//     fn eq_epsilon(&self, other: &Self, epsilon: &Self) -> bool {
//         let (mut greater, mut lesser) = (self, other);

//         if lesser > greater {
//             (greater, lesser) = (lesser, greater);
//         }

//         (greater - lesser) < *epsilon
//     }
// }

// impl FloatEpsilonEq for f64 {}

// #[component]
// fn Player() -> impl IntoView {
//     // let performance = window().performance().expect("No performance in window.");

//     // let clock = Clock::new(performance.clone());

//     // let current_time = clock.since_epoch_ms();
//     // let current_js_time = Date::new_0().get_time();

//     // log!("{current_time}\n{current_js_time}");

//     // let audios = (0)

//     let audio_context = AudioContext::new();

//     spawn_local(
//         move || {

//         }
//     );
//     let audio = audio_context.load_audio("SKI");

//     let a = signal(audio);


//     let audios = (0..24).map(
//         |hour| {
//             Arc::new(
//                 Mutex::new(
//                     HtmlAudioElementWrapper(
//                         HtmlAudioElement::
//                             new_with_src(&format!("/assets/{hour}.oga"))
//                             .expect("Couldn't create an `audio` html element!")
//                     )
//                 )
//             )
//         }
//     ).collect::<Vec<_>>();

//     let (song, set_song) = signal::<Arc<Mutex<HtmlAudioElementWrapper>>>(
//         Arc::clone(&audios[Date::new_0().get_hours() as usize])
//     );

//     let mut remove_on_song_start: Box<dyn Fn() + Send + Sync> = Box::new(|| {});
//     let mut remove_on_song_update: Box<dyn Fn() + Send + Sync> = Box::new(|| {});

//     Effect::new(
//         move || {
//             remove_on_song_start();
//             remove_on_song_update();

//             let song = {
//                 let threadsafe_song = song.get();
//                 let song_wrapper = threadsafe_song.lock().expect("Couldn't get the HtmlAudioElement!");

//                 song_wrapper.0.clone()
//             };

//             remove_on_song_start = Box::new(use_event_listener(
//                 song.clone(), 
//                 ev::loadedmetadata, 
//                 {
//                     let song = song.clone();
        
//                     move |_| {
//                         song.set_loop(true);
        
//                         let song = song.clone();
                
//                         spawn_local(async move {
//                             JsFuture::from(song.play().expect("Couldn't play audio!")).await.expect("Couldn't await play of the audio!");
//                         });
//                     }
//                 }
//             ));
        
//             remove_on_song_update = Box::new(use_event_listener(
//                 song.clone(), 
//                 ev::timeupdate, 
//                 move |_| {
//                     let audio_duration = song.duration();
                
//                     let current_time = Date::new_0();
//                     let hour_start = Date::new_with_year_month_day_hr_min_sec_milli(
//                         current_time.get_full_year(), 
//                         current_time.get_month() as i32,
//                         current_time.get_date() as i32,
//                         current_time.get_hours() as i32,
//                         0,
//                         0,
//                         0
//                     );
                
//                     const HOUR_BELLS_DURATION_MILLIS: f64 = 14033.333;
                
//                     let millis_from_hour_start: f64 = current_time.get_time() - hour_start.get_time();
//                     let target_time = ((millis_from_hour_start - HOUR_BELLS_DURATION_MILLIS) / 1000.0).rem_euclid(audio_duration);
                
//                     let correction = target_time - song.current_time();
//                     let difference = correction.abs();
                
//                     // log!("Diff: {difference}");
                
//                     if difference.eq_epsilon(&0.05, &0.01) {
//                         return;
//                     }
                
//                     // log!("Resyncing...");
                
//                     let resync_correction = if difference > 0.5 { 0.0 } else { correction };
//                     let resync_weight = 0.5;
                
//                     // log!("{resync_correction} * {resync_weight} = {}", resync_correction * resync_weight);
                
//                     song.set_current_time(target_time + (resync_correction * resync_weight));
//                 }
//             ));
//         }
//     );


//     view! {
//         <div>
//             "WHarg! Playing :D"
//         </div>
//     }
// }

// #[component]
// fn RequireInteraction(children: ChildrenFn) -> impl IntoView {
//     let (read_has_interacted, write_has_interacted) = signal(false);

//     move || {
//         if read_has_interacted.get() {
//             children()
//         } else {
//             view! {
//                 <div id="pre-interaction" on:click=move |_| { write_has_interacted.set(true); }>
//                     "Click me to start the player!"
//                 </div>
//             }.into_any()
//         }
//     }
// }

// TODO: If this ends up being a one page app, remove the dependency on Leptos Router
use leptos::{prelude::*, logging::log};
use leptos_router::{components::{Router, Routes, Route}, path};

use web_sys::{wasm_bindgen::JsCast, AudioContext, AudioNode};

use std::rc::Rc;
mod audio;
use audio::{AudioContextExtension, AudioBufferSourceNodeExtension};

mod time;
use time::Clock;

#[component]
fn Player() -> impl IntoView {
    let performance = window().performance().expect("Couldn't find [window.performance]!");
    
    let clock = Clock::new(performance);
    // let start_time = clock.since_epoch_ms();
    
    let audio_context = AudioContext::new().expect("Couldn't create an [AudioContext]!");
    let audio_destination = audio_context
        .destination()
        .dyn_into::<AudioNode>()
        .expect("Failed to cast the [AudioDestinationNode] into an [AudioNode]!");

    let song_fetcher = |source: Rc<dyn Fn() -> String>| {
        let audio_context = audio_context.clone();
        
        move || {
            let audio_context = audio_context.clone();
            let source = Rc::clone(&source);

            async move {
                audio_context.load_audio(&source()).await
            }
        }
    };

    let hour_bells = LocalResource::new(
        song_fetcher(Rc::new(|| "/assets/hour_bells.oga".to_string()))
    );

    let (current_song, current_song_hour, set_current_song_hour) = {
        let (current_song_hour, set_current_song_hour) = signal(0u8);

        let current_song = LocalResource::new(
            song_fetcher(Rc::new(move || format!("/assets/{}.oga", current_song_hour.get())))
        );

        let set_current_song_hour = move |song_hour: u8| {
            if let Some(current_song) = current_song.get().as_deref() {
                current_song.disconnect().expect("Failed to disconnect [current_song]!");
            }
    
            set_current_song_hour.set(song_hour);
        };

        (current_song, current_song_hour, set_current_song_hour)
    };

    Effect::new(move || {
        if let Some(current_song) = current_song.get().as_deref() {
            current_song
                .connect_with_audio_node(
                    &audio_destination
                )
                .expect("Failed to connect to connect [AudioBufferSourceNode] to [AudioContext]!");

            const HOUR_BELLS_DURATION_MILLIS: f64 = 14033.333;

            let duration_s = current_song.duration_s();
            let millis_from_hour_start: f64 = clock.since_epoch_ms() - clock.epoch_to_hour_start_ms();
            let target_offset = (((millis_from_hour_start - HOUR_BELLS_DURATION_MILLIS) / 1000.0) + 1.0).rem_euclid(duration_s);

            current_song.set_loop(true);
            current_song.start_with_when_and_grain_offset(0, target_offset).expect("Failed to start audio!");
        }
    });


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
            </Routes>
        </Router>
    }
} 

fn main() {
    mount_to_body(|| view! { <App/> });
}