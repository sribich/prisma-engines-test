#[cfg(feature = "mysql-native")]
mod native;
#[cfg(feature = "mysql-native")]
pub use native::*;

use super::{Circumstances, Params};
