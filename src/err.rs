use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("No midi port with the id {0}.")]
    NoMidiPort(String),
    #[error(transparent)]
    Pareg(#[from] pareg::ArgError),
    #[error(transparent)]
    MidiInit(#[from] midir::InitError),
    #[error(transparent)]
    MidiConnect(midir::ConnectError<()>),
    #[error(transparent)]
    MidiSend(#[from] midir::SendError),
    #[error(transparent)]
    Termal(#[from] termal::error::Error),
}

impl<T> From<midir::ConnectError<T>> for Error {
    fn from(value: midir::ConnectError<T>) -> Self {
        Self::MidiConnect(midir::ConnectError::new(value.kind(), ()))
    }
}