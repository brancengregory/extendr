mod macros;
mod rbool;
mod rfloat;
mod rint;
pub use rbool::RBool;
pub use rfloat::Rfloat;
pub use rint::RInt;

#[deprecated(note = "Use RInt instead", since = "0.9.0")]
pub use rint::Rint;

#[deprecated(note = "Use RBool instead", since = "0.9.0")]
pub use rbool::Rbool;

#[cfg(feature = "num-complex")]
mod rcplx_full;

#[cfg(feature = "num-complex")]
pub use rcplx_full::{c64, Rcplx};

#[cfg(not(feature = "num-complex"))]
mod rcplx_default;

#[cfg(not(feature = "num-complex"))]
pub use rcplx_default::{c64, Rcplx};
