//! Theme dependencies for in-browser search. Not included in mdbook when
//! the "search" cargo feature is disabled.

pub static JS: &[u8] = include_bytes!("searcher.js");
pub static MARK_JS: &[u8] = include_bytes!("mark.min.js");
pub static ELASTICLUNR_JS: &[u8] = include_bytes!("elasticlunr.min.js");
pub static ELASTICLUNR_STEMMER: &[u8] = include_bytes!("lunr.stemmer.support.min.js");
pub static ELASTICLUNR_RU_JS: &[u8] = include_bytes!("lunr.ru.min.js");
