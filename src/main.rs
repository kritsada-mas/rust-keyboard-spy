use rdev::{listen, Event};
use rodio::{Decoder, OutputStream, source::Source};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    // Get the MP3 file path from the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_mp3>", args[0]);
        return;
    }
    let mp3_path = args[1].clone();

    // Create a channel to communicate between the key press callback and the sound thread
    let (tx, rx) = channel();

    // Start the sound playing thread
    thread::spawn(move || {
        // Create an OutputStream and OutputStreamHandle
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        // Play the sound whenever a key press event is received
        for _ in rx {
            // Load the sound file
            let file = BufReader::new(File::open(&mp3_path).unwrap());
            let source = Decoder::new(file).unwrap();

            // Play the sound
            stream_handle.play_raw(source.convert_samples()).unwrap();
        }
    });

    // Listen for key press events
    if let Err(error) = listen(move |event| {
        match event.name {
            Some(key) => {
                println!("Key pressed: {:?}", key);
                // Send a signal to play the sound
                tx.send(()).unwrap();
            }
            None => (),
        }
    }) {
        println!("Error: {:?}", error)
    }
}
