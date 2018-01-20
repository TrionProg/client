
pub mod error;
pub use self::error::DictionaryError;

pub mod dictionary;
pub use self::dictionary::Dictionary;

pub mod chapter;
pub use self::chapter::IDChapter;

mod global;
pub use self::global::get_dictionary;