#![allow(clippy::too_many_lines)]

use base64::{Engine as _, engine::general_purpose::STANDARD};
use multitag::Tag;
use multitag::data::Album;
use multitag::data::Picture;
use qol::{Center, Wrapper};
use std::io::Cursor;
use std::io::Seek;
use std::path::Path;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, spawn_local};

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
wasm_import!(clearInput(id: &str));
wasm_import!(downloadFile(download: &str, filename: &str));

fn main() {
    sycamore::render(|| {
        // did not realize that signals have to go in the sycamore render context lmao
        let title_signal = create_signal(String::new());
        let artist_signal = create_signal(String::new());
        let album_signal = create_signal(String::new());
        let img_signal = create_signal(None);
        let file_data_signal = create_signal(Vec::new());
        let file_type_signal = create_signal(String::new());
        let file_name_signal = create_signal(String::new());
        let extension_signal = create_signal(String::new());

        let clear_all = move || {
            title_signal.set(String::new());
            artist_signal.set(String::new());
            album_signal.set(String::new());
            img_signal.set(None);
            file_data_signal.set(Vec::new());
            file_type_signal.set(String::new());
            file_name_signal.set(String::new());
            extension_signal.set(String::new());

            clearInput("song_upload");
            clearInput("thumb_upload");
        };

        let update_fields_from_file = move |path: &str, data: &Vec<u8>| {
            file_data_signal.set(data.clone());
            file_name_signal.set(path.to_string());
            let extension = Path::new(path).extension().unwrap().to_str().unwrap();
            extension_signal.set(extension.to_string());
            let cursor = Cursor::new(data);
            if let Ok(tag) = Tag::read_from(extension, cursor) {
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

                clearInput("thumb_upload");
            }
        };

        let assemble_download = move |_| {
            let mut output = file_data_signal.get_clone();
            let mut cursor = Cursor::new(&mut output);

            // TODO: make this more sane
            if let Ok(mut tag) = Tag::read_from(&extension_signal.get_clone(), &mut cursor) {
                tag.set_title(&title_signal.get_clone());
                tag.set_artist(&artist_signal.get_clone());

                let album = Album {
                    artist: Some(artist_signal.get_clone()),
                    title: Some(album_signal.get_clone()),
                    cover: img_signal.get_clone(),
                };

                tag.set_album_info(album).unwrap(); // idk why this would fail tbh

                cursor.rewind().unwrap(); // this shouldn't fail either
                tag.write_to_vec(&mut output).unwrap(); // nor should this (except for flac apparently)
            }

            let download = format!(
                "data:{};base64,{}",
                file_type_signal.get_clone(),
                STANDARD.encode(&output)
            );

            downloadFile(&download, &file_name_signal.get_clone_untracked());

            clear_all();
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
                            let filename = file.name();
                            file_type_signal.set(file.type_());
                            spawn_local(async move {
                                let buf_raw = JsFuture::from(file.array_buffer()).await; // theoretically should never be Err but we'll see
                                let data: Vec<u8> = arrayFromArrayBuffer(buf_raw.unwrap());
                                update_fields_from_file(&filename, &data);
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
                                    accept="image/jpeg, image/png, image/webp, image/gif",
                                    on:change=move |e: sycamore::web::events::Event| {
                                        let input = e.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                        let file_list = input.files().unwrap();
                                        let file = file_list.get(0).unwrap();
                                        log(file.name());
                                        log(file.type_());
                                        spawn_local(async move {
                                            let buf_raw = JsFuture::from(file.array_buffer()).await;
                                            let data: Vec<u8> = arrayFromArrayBuffer(buf_raw.unwrap());
                                            img_signal.set(Some(Picture {
                                                mime_type: file.type_(),
                                                data
                                            }));
                                        });
                                    }
                                ) {}
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
                    button(on:click=assemble_download) { "Save" }
                    a(style="display:none", id="download_anchor") {}
                }
            }
        }
    });
}
