extern crate sgx_types;
extern crate sgx_urts;
extern crate dirs;

use sgx_types::*;

pub mod enclave_api;
pub mod get_enclave;

use crate::{
    enclave_api::run_sample,
    get_enclave::get_enclave,
};

fn main() {
    let enclave = get_enclave();
    let result = unsafe {
        run_sample(
            enclave.geteid(),
            &mut sgx_status_t::SGX_SUCCESS,
        )
    };
    match result {
        sgx_status_t::SGX_SUCCESS =>
            println!("✔ Sample finished successfully!"),
        _ => {
            println!("✘ ECALL Failed: {}", result);
            return;
        }
    };
    enclave.destroy();
}
