mod error;
mod messaging;
mod behaviours;

pub use self::error::*;
pub use self::messaging::*;
pub use self::behaviours::*;

pub type Failable = Result<(), Error>;
