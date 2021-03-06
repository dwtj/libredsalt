use rand::{OsRng, Rng};
use std::slice;
use std::sync::Mutex;

lazy_static! {
    static ref RNG: Mutex<OsRng> =
        Mutex::new(OsRng::new().ok().expect("OsRng failed!"));
}

#[no_mangle]
pub unsafe extern fn randombytes(buf: *mut u8, nbytes: u64) {
    assert!(!buf.is_null());
    RNG.lock().unwrap().fill_bytes(
        slice::from_raw_parts_mut(buf, nbytes as usize)
    );
}
