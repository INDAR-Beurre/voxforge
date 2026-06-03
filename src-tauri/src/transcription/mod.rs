pub mod local;
pub mod cloud;
pub mod engine;

pub use engine::{TranscriptionEngine, TranscriptionResult, TranscriptionProvider};
pub use local::WhisperLocal;
pub use cloud::CloudProvider;
