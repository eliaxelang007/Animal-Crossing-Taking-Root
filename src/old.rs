use leptos::{
    prelude::*,
    ev,
    logging::log,
    html::Audio
};


use leptos_router::{components::{Route, Router, Routes}, path};
use leptos_use::use_event_listener;

use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{js_sys::Date, HtmlAudioElement};

use std::{ops::Deref, sync::{Arc, Mutex}};
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Sub;

use crate::time::{Clock, TimeUnit};

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

#[component]
pub fn VideoPlayer() -> impl IntoView {
    let clock = Clock::new(window().performance().expect("Old performance!!!"));
    let audio_ref: NodeRef<Audio> = NodeRef::new();

    // let current_hour = set_to_ten(Date::new_0()).get_hours();

    let on_loaded_metadata = move |_| {
        let audio = audio_ref.get().expect("Couldn't load a song for the current hour.");

        audio.set_loop(true);
  
        spawn_local(async move {
            JsFuture::from(audio.play().expect("Couldn't play audio!")).await.expect("Couldn't await play of the audio!");
        });
    };

    let on_time_update = move |_| {
        let audio = audio_ref.get().expect("Couldn't load a song for the current hour.");

        let duration_s = audio.duration();

        let since_hour = clock.since_epoch().since_last(TimeUnit::Hour);
        let target_time = since_hour.as_secs_f64().rem_euclid(duration_s);

        let correction = target_time - audio.current_time();
        let difference = correction.abs();

        if difference <= 0.05 {
            return;
        }

        let correction_weight = 0.2;
        
        log!("ASSSSS {}", correction);

        audio.set_current_time(target_time + (correction * correction_weight));
    };


    view! {
        <audio controls src={format!("/assets/10_am_video.oga")} preload="metadata" node_ref=audio_ref on:loadeddata=on_loaded_metadata on:timeupdate=on_time_update/>
        <div>
            "Playing Video :D"
        </div>
    }
}

