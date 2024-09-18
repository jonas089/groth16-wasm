# Groth16 Verifier that depends on arkworks

This branch depends on [ark_bn254](https://crates.io/crates/ark-bn254), [ark_ec](https://crates.io/crates/ark-ec) and [ark_ff](https://crates.io/crates/ark-ec).

Point coordinates and public inputs are represented as [num_bigint::BigUint](https://docs.rs/num-bigint/0.4.6/num_bigint/).

Other branches may depend on `casper_types` e.g. [U256](https://docs.rs/casper-types/latest/casper_types/struct.U256.html) for public inputs and coordinates.


Both Arkworks and bn use the alt_bn128 (=bn254) curve.

# Arkworks bn
Arkworks bn is supported, see `bn254.rs`.

# Zeropool bn
Zeropool bn is supported, see `bn.rs`.