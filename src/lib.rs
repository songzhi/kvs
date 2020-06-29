mod error;

pub use error::{KvsError, Result};
pub use kv::KvStore;

mod kv;