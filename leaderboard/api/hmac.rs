use common::consts::API_HMAC_KEY;
use hmac::{Hmac, Mac, digest::MacError};
use sha2::Sha256;

pub fn hash(val: &[u8]) -> Vec<u8> {
    let mut hmac = Hmac::<Sha256>::new_from_slice(API_HMAC_KEY).unwrap();
    hmac.update(val);

    hmac.finalize().into_bytes().to_vec()
}

pub fn verify(val: &[u8], hash: &[u8]) -> Result<(), MacError> {
    let mut hmac = Hmac::<Sha256>::new_from_slice(API_HMAC_KEY).unwrap();
    hmac.update(val);

    hmac.verify_slice(hash)
}
