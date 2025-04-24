use bincode::{
    config::{AllowTrailing, VarintEncoding, WithOtherIntEncoding, WithOtherTrailing},
    DefaultOptions, Options,
};
use once_cell::sync::Lazy;

pub const API_HMAC_KEY: &[u8] = b"Testing key";

pub static BINCODE_OPTIONS: Lazy<
    WithOtherTrailing<WithOtherIntEncoding<DefaultOptions, VarintEncoding>, AllowTrailing>,
> = Lazy::new(|| {
    bincode::DefaultOptions::new()
        .with_varint_encoding()
        .allow_trailing_bytes()
});
