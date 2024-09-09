# Groth16 Verifier that depends on arkworks

This branch depends on [ark_bn254](https://crates.io/crates/ark-bn254), [ark_ec](https://crates.io/crates/ark-ec) and [ark_ff](https://crates.io/crates/ark-ec).

Point coordinates and public inputs are represented as [num_bigint::BigUint](https://docs.rs/num-bigint/0.4.6/num_bigint/).

Other branches may depend on `casper_types` e.g. [U256](https://docs.rs/casper-types/latest/casper_types/struct.U256.html) for public inputs and coordinates.

# ECC Api


Addition `G1`:
```rust
pub fn add_g1_as_coordinates(p_x: BigUint, p_y: BigUint, q_x: BigUint, q_y: BigUint) -> G1
```

Scalar multiplication `G1`:
```rust
fn scalar_mul(p_x: BigUint, p_y: BigUint, k: BigUint) -> G1
```

# Todos
- Check that inputs are field elements prior to processing `vk_x`:

```
require(input[i] < SNARK_SCALAR_FIELD, "verifier-gte-snark-scalar-field");
```
[source](https://github.com/tornadocash/tornado-core/blob/1ef6a263ac6a0e476d063fcb269a9df65a1bd56a/contracts/Verifier.sol#L216C7-L216C81)

- Expose `pairing` so that it takes point coordinates instead of Affine points.