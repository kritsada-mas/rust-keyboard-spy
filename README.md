# MacOS Keypress Sound Player

*For thoses (Me) who can't afford thoses querky mechanical keyboards.*

This Rust program captures key press events on macOS and plays a sound file each time a key is pressed. The sound file to be played can be specified as a command-line argument when running the program.

## Prerequisites

- Rust and Cargo installed on your system. You can install Rust using [rustup](https://rustup.rs/).
- A sound file (e.g., `key01.mp3`) that you want to play on each key press event.

## Installation

1. Clone the repository or create a new Rust project:

```sh
git clone https://github.com/your-username/keypress_sound_player.git
cd keypress_sound_player
```

2. Add the required dependencies to your `Cargo.toml` file:

```toml
[dependencies]
cocoa = "0.25.0"
rdev = "0.5.3"
rodio = "0.19.0"
```

## Usage

1. Place your sound file (e.g., `key01.mp3`) in a directory accessible to your project.
2. Build and run the program, specifying the path to the sound file as a command-line argument:

```sh
cargo run -- /path/to/your/key01.mp3
```

### Example

If your sound file is located in the `src` directory of your project, you can run the program with:

```sh
cargo run -- src/key01.mp3
```

## Code Explanation

The program consists of the following main parts:

1. **Command-line Argument Parsing:**
   - The program uses `std::env::args` to get the MP3 file path from the command-line arguments.

2. **Sound Playing Thread:**
   - A separate thread is created to handle sound playback using the `rodio` crate.
   - The sound file is loaded and played each time a key press event is detected.

3. **Key Press Event Listener:**
   - The `rdev` crate is used to listen for key press events.
   - When a key press event is detected, a signal is sent through a channel to the sound playing thread to play the sound.

## Notes

- Ensure you have the necessary permissions for the terminal or executable to capture keyboard events and play sound on macOS. You may need to grant these permissions in `System Preferences -> Security & Privacy -> Privacy -> Input Monitoring`.
- This program is designed for macOS. Compatibility with other operating systems may require modifications.