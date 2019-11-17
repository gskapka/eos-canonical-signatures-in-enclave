#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]

#[macro_use]
extern crate sgx_tstd as std;
extern crate secp256k1;
extern crate rand;

use sgx_types::*;
use rand::thread_rng;

use secp256k1::{
    verify,
    Message,
    SecretKey,
    PublicKey,
    sign_canonical_for_eos,
};

#[no_mangle]
pub extern "C" fn run_sample() -> sgx_status_t {
    println!("✔ Running canonical signature sample inside enclave...");
    let private_key = SecretKey::random(&mut thread_rng());
    println!("✔ Private key generated successfully!");
    let public_key = PublicKey::from_secret_key(&private_key);
    println!("✔ Public key generated successfully!");
    let message_arr: [u8; 32] = [6; 32];
    let message = Message::parse_slice(&message_arr)
        .unwrap();
    println!(
        "✔ Message generated successfully!\n{}",
        "✔ Signing message..."
    );
    let (canonical_signature, recovery_id) = sign_canonical_for_eos(
        &message,
        &private_key,
    );
    println!(
        "✔ Message signed successfully!\n{}",
        "✔ Verifying siganture...");
    let signature_is_verified = verify(
        &message,
        &canonical_signature,
        &public_key,
    );
    println!("✔ Signature is verified: {}", signature_is_verified);
    let signature_is_canonical = canonical_signature
        .is_canonical_for_eos();
    println!(
        "✔ Signature recovery ID: {:?}\n✔ Signature is canonical: {}",
        recovery_id,
        signature_is_canonical
    );
    sgx_status_t::SGX_SUCCESS
}
