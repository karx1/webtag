use qol::{Center, Wrapper};
use sycamore::prelude::*;

mod qol;

fn main() {
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
                        accept="audio/mpeg, audio/wav, audio/aiff, audio/flac, audio/mp4, audio/ogg, audio/opus") {}
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
