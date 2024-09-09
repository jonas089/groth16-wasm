use std::str::FromStr;

use ark_bn254::{self, Config, Fq, Fq2, Fr, G1Affine, G2Affine};
use ark_ec::{models::bn::Bn, pairing::Pairing, AffineRepr, CurveGroup};
use ark_ff::{BigInteger, BigInteger256, PrimeField, Zero};
use num_bigint::BigUint;

type G1 = ark_bn254::g1::G1Affine;
type G2 = ark_bn254::g2::G2Affine;

const BASE_FIELD_MODULUS: &str =
    "21888242871839275222246405745257275088696311157297823662689037894645226208583";

fn parse_biguint(value: &str) -> BigUint {
    BigUint::parse_bytes(value.as_bytes(), 10).unwrap()
}

// Helper function to parse a string into a field element
fn parse_biguint_to_fq(value: &str) -> Fq {
    let big_int = BigUint::parse_bytes(value.as_bytes(), 10).unwrap();
    Fq::from(big_int)
}

// Helper function to parse two Fq values into Fq2 (for G2 points)
fn parse_biguint_to_fq2(value1: &str, value2: &str) -> Fq2 {
    let fq1 = parse_biguint_to_fq(value1);
    let fq2 = parse_biguint_to_fq(value2);
    Fq2::new(fq1, fq2)
}

fn fq_to_biguint(x_fq: &Fq) -> BigUint {
    let bigint_repr = x_fq.into_bigint();
    BigUint::from_bytes_le(&bigint_repr.to_bytes_le())
}

pub fn negate_g1_affine(p: G1Affine) -> G1Affine {
    let x_fq = p.x().unwrap();
    let y_coord: BigUint = fq_to_biguint(p.y().unwrap());
    let base_field_modulus_biguint = BigUint::from_str(BASE_FIELD_MODULUS).unwrap();
    if y_coord == BigUint::ZERO && fq_to_biguint(x_fq) == BigUint::ZERO {
        G1Affine::new_unchecked(parse_biguint_to_fq("0"), parse_biguint_to_fq("0"))
    } else {
        let neg_y_coord =
            (base_field_modulus_biguint.clone() - y_coord) % base_field_modulus_biguint;
        G1Affine::new_unchecked(*x_fq, Fq::from(neg_y_coord))
    }
}

pub fn add_g1_as_coordinates(p_x: BigUint, p_y: BigUint, q_x: BigUint, q_y: BigUint) -> G1 {
    let p = G1::new_unchecked(
        parse_biguint_to_fq(&p_x.to_string()),
        parse_biguint_to_fq(&p_y.to_string()),
    );
    let q = G1::new_unchecked(
        parse_biguint_to_fq(&q_x.to_string()),
        parse_biguint_to_fq(&q_y.to_string()),
    );
    (p + q).into_affine()
}

pub fn extract_g1_coordinates(p: G1) -> (BigUint, BigUint) {
    (fq_to_biguint(p.x().unwrap()), fq_to_biguint(p.y().unwrap()))
}

#[test]
fn test_negate_point_unchecked() {
    let base_field_modulus_biguint = BigUint::from_str(BASE_FIELD_MODULUS).unwrap();
    let pi_a_x = parse_biguint_to_fq(
        "4619434547164325081923648243067958995814461722276790408259976269673531268875",
    );
    let pi_a_y = parse_biguint_to_fq(
        "17285941344797724749074955491828477791926771489034344863858176130130219822865",
    );
    let pi_a: G1 = G1Affine::new_unchecked(pi_a_x, pi_a_y);

    let a_inv: G1 = negate_g1_affine(pi_a);
    assert_eq!(pi_a.x(), a_inv.x());
    println!(
        "Expect: {:?}",
        (base_field_modulus_biguint.clone()
            - BigUint::from_str(
                "17285941344797724749074955491828477791926771489034344863858176130130219822865"
            )
            .unwrap())
            % base_field_modulus_biguint
    );
    println!("Inverse: {:?}", a_inv);
}

fn scalar_mul(p_x: BigUint, p_y: BigUint, k: BigUint) -> G1 {
    let p = G1::new_unchecked(
        parse_biguint_to_fq(&p_x.to_string()),
        parse_biguint_to_fq(&p_y.to_string()),
    );
    let scalar = Fr::from_be_bytes_mod_order(&k.to_bytes_be());
    (p.into_group() * scalar).into_affine()
}

fn test_multiplier2_verification_circom_groth16() {
    let pi_a_x = parse_biguint_to_fq(
        "4619434547164325081923648243067958995814461722276790408259976269673531268875",
    );
    let pi_a_y = parse_biguint_to_fq(
        "17285941344797724749074955491828477791926771489034344863858176130130219822865",
    );
    let pi_a: G1 = G1Affine::new_unchecked(pi_a_x, pi_a_y);

    let pi_c_x = parse_biguint_to_fq(
        "2224906812514985819002007785400739200833587017118171662746436788881490639334",
    );
    let pi_c_y = parse_biguint_to_fq(
        "17575872684026867761893584228054463905548398624577391451682634656255301190545",
    );
    let pi_c: G1 = G1Affine::new_unchecked(pi_c_x, pi_c_y);

    let pi_b_x = parse_biguint_to_fq2(
        "7493377171278660922342026159516494202893397635160892892797904546053101726860",
        "12257015281543965245685445974249405875916234863299766453693211602557670657219",
    );
    let pi_b_y = parse_biguint_to_fq2(
        "12131353492675488324271920506889811484612170039713745676687476036748951969131",
        "5187697901168563347516107227846365175711629678791848343161631452197878544126",
    );
    let pi_b: G2 = G2Affine::new_unchecked(pi_b_x, pi_b_y);
    let vk_alpha1_x = parse_biguint_to_fq(
        "10246350822467771900076635245792972119666566556250807950902733806864247380952",
    );
    let vk_alpha1_y = parse_biguint_to_fq(
        "608411288378915329930935766447369940767930506471659681097230521603283651905",
    );
    let vk_alpha1: G1 = G1Affine::new_unchecked(vk_alpha1_x, vk_alpha1_y);
    let vk_beta2_x = parse_biguint_to_fq2(
        "6131344741220743386799335429820992680362925873963442544072984714378368926041",
        "15789153394103558986310497145299360386833033851225792260568730098540011835894",
    );
    let vk_beta2_y = parse_biguint_to_fq2(
        "20294744769931145130063498330622344384466672603336352492159120958989063471433",
        "3758612818443493808972214480762460937559058096828360946639526592835030859803",
    );
    let vk_beta2: G2 = G2Affine::new_unchecked(vk_beta2_x, vk_beta2_y);
    let vk_gamma2_x = parse_biguint_to_fq2(
        "10857046999023057135944570762232829481370756359578518086990519993285655852781",
        "11559732032986387107991004021392285783925812861821192530917403151452391805634",
    );
    let vk_gamma2_y = parse_biguint_to_fq2(
        "8495653923123431417604973247489272438418190587263600148770280649306958101930",
        "4082367875863433681332203403145435568316851327593401208105741076214120093531",
    );
    let vk_gamma2 = G2Affine::new_unchecked(vk_gamma2_x, vk_gamma2_y);
    let vk_delta_2_x = parse_biguint_to_fq2(
        "2331685158934782270621884102594249521613050557963549726699028399736205391535",
        "19932904864070474666569306255777842591060844877329635027414969502137306204189",
    );
    let vk_delta_2_y = parse_biguint_to_fq2(
        "18328176957461925860223052153948913273697229957014116201548221893444067392668",
        "4892040004975702242175034718975862230235444061193165072087100231911981786509",
    );
    let vk_delta2 = G2Affine::new_unchecked(vk_delta_2_x, vk_delta_2_y);

    let ic_0_x = parse_biguint_to_fq(
        "21631942485326744232766849971585115612456593023934275850499378648736190910977",
    );
    let ic_0_y = parse_biguint_to_fq(
        "10990468352600828980319524627816836646396500759270877213016615483259184677726",
    );
    let ic_0: G1 = G1Affine::new_unchecked(ic_0_x, ic_0_y);

    let ic_1_x = parse_biguint_to_fq(
        "21229468961321243348662110358869948527418599923035918852855987234632719885365",
    );
    let ic_1_y = parse_biguint_to_fq(
        "14718418867019175107712538434554605791301866350066611533272126162199859274702",
    );
    let ic_1: G1 = G1Affine::new_unchecked(ic_1_x, ic_1_y);

    let mut vk_x: G1 = ic_0;
    let public_output = BigUint::from_str("33").unwrap();
    let vk_x_as_coordinates = extract_g1_coordinates(vk_x);
    let ic_1_as_coordinates = extract_g1_coordinates(ic_1);
    let ic_1_scalar_res: G1 =
        scalar_mul(ic_1_as_coordinates.0, ic_1_as_coordinates.1, public_output);
    let ic_1_scalar_res_as_coordinates = extract_g1_coordinates(ic_1_scalar_res);
    vk_x = add_g1_as_coordinates(
        vk_x_as_coordinates.0,
        vk_x_as_coordinates.1,
        ic_1_scalar_res_as_coordinates.0,
        ic_1_scalar_res_as_coordinates.1,
    );

    let pairing = <Bn<Config> as Pairing>::multi_pairing(
        vec![negate_g1_affine(pi_a), vk_alpha1, vk_x, pi_c],
        vec![pi_b, vk_beta2, vk_gamma2, vk_delta2],
    )
    .is_zero();
    assert!(pairing);
}

#[test]
fn default() {
    test_multiplier2_verification_circom_groth16();
}
