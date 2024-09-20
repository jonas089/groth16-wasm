// Written for the Casper Blockchain with casper_types 5
use crate::BASE_FIELD_MODULUS;
use bn::{AffineG1, Fq, Fr, Group, G1};
#[cfg(feature = "casper")]
use casper_contract::contract_api::builtins::altbn128::{
    alt_bn128_add, alt_bn128_mul, alt_bn128_pairing, Fq, Pair, G1,
};
use casper_types::U256;

pub fn compute_vk(ics: Vec<AffineG1>, inputs: Vec<U256>) -> (U256, U256) {
    let mut vk: AffineG1 = ics[0];
    for (idx, ic) in ics.into_iter().enumerate().skip(1) {
        let ic_scalar: (U256, U256) = alt_bn128_mul(
            fq_to_u256(ic.x()),
            fq_to_u256(ic.y()),
            inputs[idx - 1].clone(),
        );
        println!("processed input: {}, with ic: {}", inputs[idx - 1], idx);
        let coords = alt_bn128_add(
            fq_to_u256(vk.x()),
            fq_to_u256(vk.y()),
            ic_scalar.0,
            ic_scalar.1,
        );
        vk = AffineG1::new(fq_from_u256(coords.0), fq_from_u256(coords.1)).unwrap();
    }
    (fq_to_u256(vk.x()), fq_to_u256(vk.y()))
}

pub fn negate_g1_affine(x: U256, y: U256) -> (U256, U256) {
    let base_field_modulus_biguint = U256::from_str_radix(BASE_FIELD_MODULUS, 10).unwrap();
    if y == U256::zero() && x == U256::zero() {
        (x, y)
    } else {
        let neg_y_coord = (base_field_modulus_biguint.clone() - y) % base_field_modulus_biguint;
        (x, neg_y_coord)
    }
}

fn point_from_coords(x: U256, y: U256) -> G1 {
    let px = Fq::from_slice(&x.to_be_bytes()).unwrap();
    let py = Fq::from_slice(&y.to_be_bytes()).unwrap();

    if px == Fq::zero() && py == Fq::zero() {
        G1::zero()
    } else {
        AffineG1::new(px, py).unwrap().into()
    }
}

pub fn fq_to_u256(fq: Fq) -> U256 {
    let mut buf = [0u8; 32];
    fq.to_big_endian(&mut buf).unwrap();
    U256::from_big_endian(&buf)
}

pub fn fq_from_u256(u256: U256) -> Fq {
    let mut buf = [0u8; 32];
    u256.to_big_endian(&mut buf);
    let reconstructed = U256::from_be_bytes(buf.clone());
    assert_eq!(reconstructed, u256);

    let fq = Fq::from_slice(&buf).unwrap();
    let u256_rec = fq_to_u256(fq);
    assert_eq!(u256_rec, u256);

    Fq::from_slice(&buf).unwrap()
}

#[cfg(feature = "casper")]
pub fn alt_bn128_add() {}

#[cfg(feature = "casper")]
pub fn alt_bn128_mul() {}

#[cfg(not(feature = "casper"))]
pub fn alt_bn128_add(x1: U256, y1: U256, x2: U256, y2: U256) -> (U256, U256) {
    let p1 = point_from_coords(x1, y1);
    let p2 = point_from_coords(x2, y2);

    let mut x = U256::zero();
    let mut y = U256::zero();

    if let Some(sum) = AffineG1::from_jacobian(p1 + p2) {
        x = fq_to_u256(sum.x());
        y = fq_to_u256(sum.y());
    }
    (x, y)
}

#[cfg(not(feature = "casper"))]
pub fn alt_bn128_mul(x: U256, y: U256, scalar: U256) -> (U256, U256) {
    let p = point_from_coords(x, y);

    let mut x = U256::zero();
    let mut y = U256::zero();
    let fr = Fr::from_slice(&scalar.to_be_bytes()).unwrap();

    if let Some(product) = AffineG1::from_jacobian(p * fr) {
        x = fq_to_u256(product.x());
        y = fq_to_u256(product.y());
    }
    (x, y)
}

pub fn alt_bn128_pairing(values: Vec<(U256, U256, U256, U256, U256, U256)>) -> bool {
    let mut pairs = Vec::with_capacity(values.len());
    for (ax, ay, bax, bay, bbx, bby) in values {
        let ax = fq_from_u256(ax);
        let ay = fq_from_u256(ay);
        let bax: Fq = fq_from_u256(bax);
        let bbx: Fq = fq_from_u256(bbx);
        let bay = fq_from_u256(bay);
        let bby = fq_from_u256(bby);

        let g1_a = {
            if ax.is_zero() && ay.is_zero() {
                bn::G1::zero()
            } else {
                bn::AffineG1::new(ax, ay).unwrap().into()
            }
        };
        let g1_b = {
            let ba = bn::Fq2::new(bax, bay);
            let bb = bn::Fq2::new(bbx, bby);

            if ba.is_zero() && bb.is_zero() {
                bn::G2::zero()
            } else {
                bn::AffineG2::new(ba, bb).unwrap().into()
            }
        };
        pairs.push((g1_a, g1_b));
    }

    bn::pairing_batch(pairs.as_slice()) == bn::Gt::one()
}
