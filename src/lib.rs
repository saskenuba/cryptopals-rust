pub mod b64;
pub mod hex;
pub mod xor;

pub mod challenges;

pub(crate) type AnyResult<T> = anyhow::Result<T>;

static B64: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
