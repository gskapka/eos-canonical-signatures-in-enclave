# :lock_with_ink_pen: EOS Canonical Signatures in Intel SGX Enclave Example

&nbsp;

A sample demonstrating canonical EOS signatures in an Intel SGX enclave.

***

&nbsp;

### :wrench: Build It

Prerequisites:

 - An SGX capable machine.
 - Intel SGX SDK 2.5 for Linux installed
 - Ubuntu 18.04
 - Docker.

Relies on: __`rust-sgx-sdk v1.0.9`__!

1) Clone the Rust SGX SDK:

__`❍ git clone https://github.com/apache/mesatee-sgx.git --branch v1.0.9`__

2) Clone this repo:

__`❍ git clone https://github.com/gskapka/eos-canonical-signatures-in-enclave.git ./mesatee-sgx/samplecode/`__

3) Pull the correct SGX docker image:

__`❍ docker pull baiduxlab/sgx-rust-stable:1804-1.0.9`__

4) Start docker container pointing it to the SDK on your machine:

__`❍ docker run -v /your/path/to/rust-sgx:/root/sgx -ti --device /dev/isgx baiduxlab/sgx-rust`__

5) Start the AESM service inside docker:

__`❍ LD_LIBRARY_PATH=/opt/intel/libsgx-enclave-common/aesm /opt/intel/libsgx-enclave-common/aesm/aesm_service &`__

6) Enter EOS sample dir:

__`❍ cd sgx/samplecode/eos-canonical-signatures-in-enclave`__

7) Build!

__`❍ make`__

&nbsp;

***

### :point_right: Run it:

7) After the above build, simply:

__`❍ cd bin && ./app`__

```
root@734a3b324bba# cd bin && ./app

✔ Home dir is /root
✘ Open token file /root/enclave.token error! Will create one.
✔ Running canonical signature sample inside enclave...
✔ Private key generated successfully!
✔ Public key generated successfully!
✔ Message generated successfully!
✔ Signing message...
✔ Message signed successfully!
✔ Verifying siganture...
✔ Signature is verified: true
✔ Signature recovery ID: RecoveryId(1)
✔ Signature is canonical: true
✔ Sample finished successfully!

```

&nbsp;

***

&nbsp;

### :black_nib: Notes

The above build steps are monstrously fragile. Hopefully there's enough version-specific information above plus pinned dependencies inside the example to make it less so.

&nbsp;

***

&nbsp;

### :guardsman: Tests

&nbsp;

There are no tests yet! :S

***

&nbsp;

### :black_nib: To Do:

- [ ] Tests!
- [ ] Use GH repos for sgx-paths in `Cargo.toml`
- [ ] Extract from the SDK dir structure.
