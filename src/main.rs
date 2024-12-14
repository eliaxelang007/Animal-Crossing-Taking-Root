use leptos::{html::Video, prelude::*};
use web_sys::js_sys::Date;
use wasm_bindgen_futures::{JsFuture, spawn_local};
use leptos::html::Audio;
use leptos::logging::log;
use leptos_router::{components::{Route, Router, Routes}, path};

fn set_to_ten(date: Date) -> Date {
    Date::new_with_year_month_day_hr_min_sec_milli(
        date.get_full_year(), 
        date.get_month() as i32,
        date.get_date() as i32,
        10,
        date.get_minutes() as i32,
        date.get_seconds() as i32,
        date.get_milliseconds() as i32
    )
}

fn f64_eq_epsilon(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

#[component]
fn Player() -> impl IntoView {
    let audio_ref: NodeRef<Audio> = NodeRef::new();

    const HOUR_BELLS_DURATION_MILLIS: f64 = 14033.333;

    let current_hour = set_to_ten(Date::new_0()).get_hours();

    let on_loaded_metadata = move |_| {
        let audio = audio_ref.get().expect("Couldn't load a song for the current hour.");

        audio.set_loop(true);
  
        spawn_local(async move {
            JsFuture::from(audio.play().expect("Couldn't play audio!")).await.expect("Couldn't await play of the audio!");
        });
    };

    let on_time_update = move |_| {
        let audio = audio_ref.get().expect("Couldn't load a song for the current hour.");

        let audio_duration = audio.duration();

        let current_time = set_to_ten(Date::new_0());
        let hour_start = Date::new_with_year_month_day_hr_min_sec_milli(
            current_time.get_full_year(), 
            current_time.get_month() as i32,
            current_time.get_date() as i32,
            current_time.get_hours() as i32,
            0,
            0,
            0
        );

        let millis_from_hour_start: f64 = current_time.get_time() - hour_start.get_time();
        let target_time = ((millis_from_hour_start - HOUR_BELLS_DURATION_MILLIS) / 1000.0).rem_euclid(audio_duration);

        let correction = target_time - audio.current_time();
        let difference = correction.abs();

        log!("Diff: {difference}");

        if f64_eq_epsilon(difference, 0.05, 0.01) {
            return;
        }

        log!("Resyncing...");

        let resync_correction = if difference > 0.5 { 0.0 } else { correction };
        let resync_weight = 1.0;

        // log!("{resync_correction} * {resync_weight} = {}", resync_correction * resync_weight);

        audio.set_current_time(target_time + (resync_correction * resync_weight));
    };


    view! {
        <audio controls src={format!("/assets/{current_hour}.oga")} preload="metadata" node_ref=audio_ref on:loadeddata=on_loaded_metadata on:timeupdate=on_time_update/>
        <div>
            "Playing :D"
        </div>
    }
}

// #[component]
// fn VideoPlayer() -> impl IntoView {
//     let audio_ref: NodeRef<Audio> = NodeRef::new();

//     // let current_hour = set_to_ten(Date::new_0()).get_hours();

//     let on_loaded_metadata = move |_| {
//         let audio = audio_ref.get().expect("Couldn't load a song for the current hour.");

//         audio.set_loop(true);
  
//         spawn_local(async move {
//             JsFuture::from(audio.play().expect("Couldn't play audio!")).await.expect("Couldn't await play of the audio!");
//         });
//     };

//     let on_time_update = move |_| {
//         let audio = audio_ref.get().expect("Couldn't load a song for the current hour.");

//         let audio_duration = audio.duration();

//         let current_time = set_to_ten(Date::new_0());
//         let hour_start = Date::new_with_year_month_day_hr_min_sec_milli(
//             current_time.get_full_year(), 
//             current_time.get_month() as i32,
//             current_time.get_date() as i32,
//             current_time.get_hours() as i32,
//             0,
//             0,
//             0
//         );

//         let millis_from_hour_start: f64 = current_time.get_time() - hour_start.get_time();
//         let target_time = ((millis_from_hour_start) / 1000.0).rem_euclid(audio_duration);

//         let correction = target_time - audio.current_time();
//         let difference = correction.abs();

//         if f64_eq_epsilon(difference, 0.05, 0.025) {
//             return;
//         }

//         log!("Resyncing...");

//         let resync_correction = if difference > 0.5 { 0.0 } else { correction };
//         let resync_weight = 0.9;

//         log!("{resync_correction} * {resync_weight} = {}", resync_correction * resync_weight);

//         audio.set_current_time(target_time + (resync_correction * resync_weight));
//     };


//     view! {
//         <audio controls src={format!("/assets/10_am_video.oga")} preload="metadata" node_ref=audio_ref on:loadeddata=on_loaded_metadata on:timeupdate=on_time_update/>
//         <div>
//             "Playing Video :D"
//         </div>
//     }
// }

#[component]
fn RequireInteraction(children: ChildrenFn) -> impl IntoView {
    let (read_has_interacted, write_has_interacted) = signal(false);

    move || {
        if read_has_interacted.get() {
            children()
        } else {
            view! {
                <div id="pre-interaction" on:click=move |_| { write_has_interacted.set(true); }>
                    "Click me to start the player!"
                </div>
            }.into_any()
        }
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
                // <Route path=path!("/test") view=|| view! {
                //     <RequireInteraction>
                //         <VideoPlayer/>
                //     </RequireInteraction>
                // }/>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(
        || view! { <App/> }
    )
}