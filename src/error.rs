/// SoundboardBot error
///
/// Used to indicate errors that occur in running the GUI, file I/O, and sound playback.
#[derive(Debug)]
pub enum Error {
    /// A rendering issue occurred in the GUI
    GuiError(eframe::Error),
    /// File read or write failed
    IoError(std::io::Error),
    /// Error occurred during audio playback
    FileDecodeError(awedio::Error),
    /// Error occurred sending data to the media player thread
    AudioPlaybackError,
    /// Error in building the cpal backend for awedio
    AudioBackendError(awedio::backends::CpalBackendError),
}

impl From<awedio::Error> for Error {
    fn from(e: awedio::Error) -> Self {
        Self::FileDecodeError(e)
    }
}

impl From<awedio::backends::CpalBackendError> for Error {
    fn from(e: awedio::backends::CpalBackendError) -> Self {
        Self::AudioBackendError(e)
    }
}
