use base64::{engine::general_purpose::STANDARD, Engine as _};
use multitag::data::Picture;
use multitag::Tag;
use qol::{Center, Wrapper};
use std::io::Cursor;
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
    sycamore::render(|| {
        // did not realize that signals have to go in the sycamore render context lmao
        let title_signal = create_signal(String::new());
        let artist_signal = create_signal(String::new());
        let album_signal = create_signal(String::new());
        let img_signal = create_signal(None);

        let update_fields_from_file = move |path: &str, data: &Vec<u8>| {
            let cursor = Cursor::new(data);
            if let Ok(tag) = Tag::read_from(path, cursor) {
                // skill issue from the multitag author here
                // i wonder who that was
                title_signal.set(tag.title().unwrap_or_default().into());
                artist_signal.set(tag.artist().unwrap_or_default());

                if let Some(album) = tag.get_album_info() {
                    album_signal.set(album.title.unwrap_or_default());
                    img_signal.set(album.cover);
                } else {
                    img_signal.set(None);
                    album_signal.set(String::new());
                }
            }
        };

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
                                update_fields_from_file(&file.name(), &data);
                            });
                        }) {}
                    div(class="row") {
                        div(class="column") {
                            Wrapper {
                                img(class="thumb", width="150", height="150", src=img_signal.with(|maybe_pic: &Option<Picture>| {
                                    if let Some(pic) = maybe_pic {
                                        format!("data:{};base64,{}", pic.mime_type, STANDARD.encode(&pic.data))
                                    } else {
                                        String::new()
                                    }
                                })) {}
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
                                input(r#type="text", id="title", name="title", bind:value=title_signal) {}
                                label(r#for="artist") { "Artist" }
                                input(r#type="text", id="artist", name="artist", bind:value=artist_signal) {}
                                label(r#for="album") { "Album" }
                                input(r#type="text", id="album", name="album", bind:value=album_signal) {}
                            }
                        }
                    }
                }
            }
        }
    });
}
