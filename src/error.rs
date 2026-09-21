#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("k is too large: {k} > 255")]
    KTooLarge { k: usize },

    #[error("max_mismatches is too large: {max_mismatches} > 255")]
    MaxMismatchesTooLarge { max_mismatches: usize },
}
