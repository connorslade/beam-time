use bincode::{
    DefaultOptions, Options,
    config::{AllowTrailing, VarintEncoding, WithOtherIntEncoding, WithOtherTrailing},
};
use once_cell::sync::Lazy;

const RAW_API_HMAC_KEY: Option<&str> = option_env!("API_KEY");
pub const API_TESTING: bool = RAW_API_HMAC_KEY.is_none();
pub const API_HMAC_KEY: &[u8] = if let Some(env) = RAW_API_HMAC_KEY {
    env.as_bytes()
} else {
    b"Testing Key"
};

pub static BINCODE_OPTIONS: Lazy<
    WithOtherTrailing<WithOtherIntEncoding<DefaultOptions, VarintEncoding>, AllowTrailing>,
> = Lazy::new(|| {
    bincode::DefaultOptions::new()
        .with_varint_encoding()
        .allow_trailing_bytes()
});
