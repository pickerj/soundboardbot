use std::path::PathBuf;
use std::sync::mpsc::{self, Sender};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use log::debug;

use crate::Error;

pub struct MediaPlayer {
    /// Handle for the media player thread
    _handle: JoinHandle<()>,
    /// Sender for adding filepaths to media player thread's queue
    filepath_tx: Sender<Arc<PathBuf>>,
}

impl MediaPlayer {
    /// Start media player thread with default sound device
    pub fn start_on_default_device() -> Result<Self, Error> {
        // Init thread for medida player
        let (filepath_tx, filepath_rx) = mpsc::channel::<Arc<PathBuf>>();
        let handle = thread::spawn(move || {
            use awedio::sounds::{open_file, MemorySound};
            use std::collections::HashMap;
            let mut sound_cache: HashMap<Arc<PathBuf>, MemorySound> = HashMap::new();
            let (mut manager, _backend) = awedio::start().expect("Could not start media player backend");
            loop {
                if let Ok(path) = filepath_rx.recv() {
                    if !sound_cache.contains_key(&path) {
                        let sound = open_file(path.as_path()).expect("Error opening audio file");
                        let mem_sound = MemorySound::from_sound(sound).expect("Error loading audio file into memory");
                        sound_cache.insert(path.clone(), mem_sound);
                    }
                    debug!(
                        "Media player thread attempting to play sound {}",
                        &path.to_string_lossy()
                    );
                    manager.play(Box::new(sound_cache.get(&path).expect("Sound was somehow not present in the cache").clone()));
                } else {
                    // Error is returned when no more senders exist, so exit thread
                    return;
                }
            }
        });
        Ok(Self { filepath_tx, _handle: handle })
    }
}

impl MediaPlayer {
    pub fn queue_sound(&self, path: Arc<PathBuf>) -> Result<(), Error> {
        match self.filepath_tx.send(path.clone()) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::AudioPlaybackError),
        }
    }
}
