mod extract_duplicates;
mod remove_duplicates_fast;
mod remove_duplicates_slow;

pub use extract_duplicates::DuplicatesExtractor;
pub use remove_duplicates_fast::DuplicatesRemoverMem;
pub use remove_duplicates_slow::DuplicatesRemoverSlow;
