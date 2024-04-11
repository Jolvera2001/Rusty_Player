use std::fs::File;
use std::io::BufReader;
use rodio::{ Decoder, Sink, Source };

static mut SINK: Option<Sink> = None;

#[tauri::command]
pub fn play_audio(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    unsafe {
        if let Some(sink) = &mut SINK {
            sink.append(source);
        } else {
            let sink = Sink::new(&rodio::default_output_device().unwrap());
            sink.append(source);
            SINK = Some(sink);
        }
        SINK.as_mut().unwrap().play();
    }
}

#[tauri::command]
pub fn pause_audio() {
    unsafe {
        if let Some(sink) = &mut SINK {
            sink.pause();
        }
    }
}