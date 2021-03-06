mod chunk;
mod chunk_exact;
mod multiple;
#[cfg(feature = "parallel")]
mod par_multiple;
#[cfg(feature = "parallel")]
mod par_single;
mod single;

#[cfg(feature = "parallel")]
use super::IntoIterator;
use super::{
    AbstractMut, CurrentId, DoubleEndedShiperator, ExactSizeShiperator, IntoAbstract, Shiperator,
};

pub use chunk::*;
pub use chunk_exact::*;
pub use multiple::*;
#[cfg(feature = "parallel")]
pub use par_multiple::*;
#[cfg(feature = "parallel")]
pub use par_single::ParTight1;
pub use single::Tight1;
