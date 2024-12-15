mod time;

use std::marker::PhantomData;

use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys::ArrayBuffer, wasm_bindgen::JsCast, AudioBuffer, AudioBufferSourceNode, AudioContext as WebAudioContext, Response};
use gloo_net::http::Request;
// use thiserror::Error as ThisError;

struct AudioContext(WebAudioContext);

struct Audio<'a> {
    parent: PhantomData<&'a AudioContext>,
    source_node: AudioBufferSourceNode
}

impl Audio<'_> {
    fn start(&self, start_at: f64) {
        self.source_node.start_with_when(start_at).expect("Couldn't start at time!");
    }
}

impl AudioContext {
    async fn load_audio<'a>(&'a self, source: &str) -> Audio<'a> {
        let audio_request = Request::get(source)
            .send()
            .await
            .expect("Failed to fetch from `src`!");

        let raw_response: Response = audio_request.into();
        let array_buffer = JsFuture::from(
            raw_response
                .array_buffer()
                .expect("Couldn't get a promise for the `ArrayBuffer`!")
        )
        .await
        .expect("Failed to load data as an `ArrayBuffer`!")
        .dyn_into::<ArrayBuffer>()
        .expect("Failed to cast data into an `ArrayBuffer`!");

        let audio_buffer = JsFuture::from(
            self.0
                .decode_audio_data(&array_buffer)
                .expect("Couldn't get a promise for the `AudioBuffer`!")
        )
        .await
        .expect("Couldn't decode data as an `AudioBuffer`!")
        .dyn_into::<AudioBuffer>()
        .expect("Couldn't cast data into an `AudioBuffer`!");        

        let source_node = self.0.create_buffer_source().expect("Couldn't create a buffer source!");

        source_node.set_buffer(Some(&audio_buffer));

        Audio {
            parent: PhantomData,
            source_node
        }
    }
}