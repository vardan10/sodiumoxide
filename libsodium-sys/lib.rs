#![allow(non_upper_case_globals)]

#[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
extern crate libc;

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
mod libc {
    pub type c_void = u8;
    pub type c_int = i32;
    pub type c_ulonglong = u64;
    pub type c_char = i8;
    pub type size_t = usize;
    pub type uint64_t = u64;
}

use libc::{c_char, c_int, c_ulonglong, c_void, size_t, uint64_t};

include!("src/core.rs");

include!("src/crypto_aead_chacha20poly1305.rs");

include!("src/crypto_auth.rs");
include!("src/crypto_auth_hmacsha256.rs");
include!("src/crypto_auth_hmacsha512.rs");
include!("src/crypto_auth_hmacsha512256.rs");

include!("src/crypto_box.rs");
include!("src/crypto_box_curve25519xsalsa20poly1305.rs");

include!("src/crypto_core_hsalsa20.rs");
include!("src/crypto_core_salsa20.rs");
include!("src/crypto_core_salsa2012.rs");
include!("src/crypto_core_salsa208.rs");

include!("src/crypto_generichash.rs");
include!("src/crypto_generichash_blake2b.rs");

include!("src/crypto_hash.rs");
include!("src/crypto_hash_sha256.rs");
include!("src/crypto_hash_sha512.rs");

include!("src/crypto_onetimeauth.rs");
include!("src/crypto_onetimeauth_poly1305.rs");

include!("src/crypto_pwhash_scryptsalsa208sha256.rs");

include!("src/crypto_scalarmult.rs");
include!("src/crypto_scalarmult_curve25519.rs");

include!("src/crypto_secretbox.rs");
include!("src/crypto_secretbox_xsalsa20poly1305.rs");
include!("src/crypto_shorthash_siphash24.rs");
include!("src/crypto_sign_ed25519.rs");

include!("src/crypto_stream.rs");
include!("src/crypto_stream_chacha20.rs");
include!("src/crypto_stream_salsa20.rs");
include!("src/crypto_stream_salsa2012.rs");
include!("src/crypto_stream_salsa208.rs");
include!("src/crypto_stream_xsalsa20.rs");
include!("src/crypto_stream_xchacha20.rs");

include!("src/crypto_verify_16.rs");
include!("src/crypto_verify_32.rs");
include!("src/crypto_verify_64.rs");

include!("src/randombytes.rs");
include!("src/utils.rs");
include!("src/version.rs");
