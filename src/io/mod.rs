//! I/O functions

pub use self::io::*;
pub use self::mmio::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use self::pio::*;

mod io;
mod mmio;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod pio;
