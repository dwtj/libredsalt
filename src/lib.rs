extern crate libc;
extern crate rand;

mod ffi;

pub mod crypto_box;
pub mod crypto_secretbox;
pub mod crypto_scalarmult;
pub mod crypto_onetimeauth;
pub mod crypto_stream;
pub mod randombytes;
pub mod crypto_hash;
pub mod crypto_sign;

/// Returns true iff the two 16 byte strings are equivalent.
///
/// ```
/// let x: [42u8; 16];
/// let y: [42u8; 16];
/// assert!(crypto_verify_16(x, y) == true);
/// ```
pub fn crypto_verify_16(x: &[u8], y: &[u8]) -> bool {
    assert!(x.len() == 16, "`x` must be 16 bytes.");
    assert!(y.len() == 16, "`y` must be 16 bytes.");
    unsafe {
        match ffi::crypto_verify_16_tweet(x.as_ptr(), y.as_ptr()) {
            0 => true,
            -1 => false,
            _ => unreachable!("Unexpected value produced by `crypto_verify_16_tweet()`.")
        }
    }
}

/// Returns true iff the two 32 byte strings are equivalent.
/// ```
/// let x: [42u8; 32];
/// let y: [42u8; 32];
/// assert!(crypto_verify_32(x, y) == true);
/// ```
pub fn crypto_verify_32(x: &[u8], y: &[u8]) -> bool {
    assert!(x.len() == 32, "`x` must be 32 bytes.");
    assert!(y.len() == 32, "`y` must be 32 bytes.");
    unsafe {
        match ffi::crypto_verify_32_tweet(x.as_ptr(), y.as_ptr()) {
            0 => true,
            -1 => false,
            _ => unreachable!("Unexpected value produced by `crypto_verify_32_tweet()`.")
        }
    }
}
