//! Theme dependencies for in-browser search. Not included in mdbook when
//! the "search" cargo feature is disabled.

pub(crate) static JS: &[u8] = include_bytes!("../../front-end/searcher/searcher.js");
pub(crate) static MARK_JS: &[u8] = include_bytes!("../../front-end/searcher/mark.min.js");
pub(crate) static ELASTICLUNR_JS: &[u8] =
    include_bytes!("../../front-end/searcher/elasticlunr.min.js");
pub(crate) static ELASTICLUNR_STEMMER: &[u8] =
    include_bytes!("../../front-end/searcher/lunr.stemmer.support.min.js");
pub(crate) static ELASTICLUNR_RU_JS: &[u8] =
    include_bytes!("../../front-end/searcher/lunr.ru.min.js");
