#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::SoundboardApp;

mod player;
pub use player::MediaPlayer;

mod error;
pub use error::Error;
