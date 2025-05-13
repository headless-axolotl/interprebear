use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

// I'm not sure why it gives the warning dead_code,
// when it is clearly used in the implementation of Display.
#[allow(dead_code)]
#[derive(Debug, From)]
pub enum Error {
    #[from]
    Io(std::io::Error),
    NoNextFood,
    StuckAtGate,
    TileNotDirection,
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
impl std::error::Error for Error {}
