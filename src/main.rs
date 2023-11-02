use rodio::{source::Source, Decoder, OutputStream};
use rust_embed::RustEmbed;
use std::io::{Cursor};

use aintnobody::errors::Error;

#[derive(RustEmbed)]
#[folder = "au"]
struct Audio;

fn load_source(name: &str) -> Result<Decoder<Cursor<Vec<u8>>>, Error> {
    let ogg = Audio::get(name).unwrap();
    let raw = Vec::from(ogg.data.as_ref());

    let cur = Cursor::new(raw);
    Ok(Decoder::new_mp3(cur)?)
}
fn playintnobody(default_duration: std::time::Duration) -> Result<(), Error> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let source: Decoder<Cursor<Vec<u8>>> = load_source("aintnobody.mp3")?;
    let duration = source.total_duration();
    stream_handle.play_raw(source.convert_samples())?;
    match duration {
        Some(duration) => std::thread::sleep(duration),
        None => std::thread::sleep(default_duration),
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    playintnobody(std::time::Duration::from_millis(1284))?;
    Ok(())
}
