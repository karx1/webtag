use qol::{Center, Wrapper};
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};

mod qol;

macro_rules! wasm_import {
    ($($tt:tt)*) => {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen]
            pub fn $($tt)*;
        }
    };
}
macro_rules! wasm_import_with_ns {
    ($ns: ident, $($tt:tt)*) => {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = $ns)]
            pub fn $($tt)*;
        }
    };
}

wasm_import_with_ns!(console, log(s: String));
wasm_import!(arrayFromArrayBuffer(buf: JsValue) -> Vec<u8>);

fn main() {
    let update_fields_from_file = move |data: &Vec<u8>| {
        let magic = String::from_utf8_lossy(&data.as_slice()[0..4]);
        log(magic.into());
    };
    sycamore::render(|| {
        view! {
            Wrapper {
                Center {
                    h1 { "WebTag" }
                    label(r#for="song_upload") {"Select an audio file: "}
                    input(
                        r#type="file",
                        id="song_upload",
                        name="song_upload",
                        accept="audio/mpeg, audio/wav, audio/aiff, audio/flac, audio/mp4, audio/ogg, audio/opus",
                        on:change=move |e: sycamore::web::events::Event| {
                            let input = e.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
                            let file_list = input.files().unwrap();
                            let file = file_list.get(0).unwrap();
                            log(file.name());
                            spawn_local(async move {
                                let buf_raw = JsFuture::from(file.array_buffer()).await; // theoretically should never be Err but we'll see
                                let data: Vec<u8> = arrayFromArrayBuffer(buf_raw.unwrap());
                                update_fields_from_file(&data);
                            });
                        }) {}
                    div(class="row") {
                        div(class="column") {
                            Wrapper {
                                img(class="thumb", width="150", height="150") {}
                                input(
                                    r#type="file",
                                    id="thumb_upload",
                                    name="thumb_upload",
                                    accept="image/jpeg, image/png, image/webp, image/gif") {}
                            }
                        }
                        div(class="column") {
                            Wrapper {
                                label(r#for="title") { "Title" }
                                input(r#type="text", id="title", name="title") {}
                                label(r#for="artist") { "Artist" }
                                input(r#type="text", id="artist", name="artist") {}
                                label(r#for="album") { "Album" }
                                input(r#type="text", id="album", name="album") {}
                            }
                        }
                    }
                }
            }
        }
    });
}
