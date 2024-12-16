use wasm_bindgen_futures::JsFuture;
use web_sys::{
    js_sys::Promise, 
    wasm_bindgen::{JsCast, JsValue}, 
    AudioBufferSourceNode, 
    AudioContext, 
    Response
};

use gloo_net::http::Request;

pub trait AudioContextExtension {
    async fn load_audio(&self, source: &str) -> AudioBufferSourceNode;
}

impl AudioContextExtension for AudioContext {
    async fn load_audio(&self, source: &str) -> AudioBufferSourceNode {
        async fn from_maybe_promise<T: JsCast>(maybe_promise: Result<Promise, JsValue>) -> T {
            JsFuture::from(
                maybe_promise.expect("Couldn't extract the promise from [maybe_promise]!")
            )
            .await
            .expect("Promise failed!")
            .dyn_into::<T>()
            .expect("[JsCast] failed!")
        }

        let audio_request = Request::get(source)
            .send()
            .await
            .expect("Couldn't fetch the audio from the url [source]!");

        let raw_response: Response = audio_request.into();

        let array_buffer = from_maybe_promise(raw_response.array_buffer()).await;
        let audio_buffer = from_maybe_promise(self.decode_audio_data(&array_buffer)).await;

        let audio_source = self.create_buffer_source().expect("Couldn't create an [AudioBufferSourceNode]!");

        audio_source.set_buffer(Some(&audio_buffer));

        audio_source
    }
}

pub trait AudioBufferSourceNodeExtension {
    fn duration_s(&self) -> f64;
}

impl AudioBufferSourceNodeExtension for AudioBufferSourceNode {
    fn duration_s(&self) -> f64 {
        self.buffer().expect("Bufferless audio source node!").duration()
    }
}