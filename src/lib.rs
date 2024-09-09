use std::str::FromStr;

use ark_bn254::{self, Config, Fq, Fr, G1Affine, G2Affine};
use ark_ec::{models::bn::Bn, pairing::Pairing, AffineRepr, CurveGroup};
use ark_ff::{BigInteger, PrimeField, Zero};
use num_bigint::BigUint;

type G1 = ark_bn254::g1::G1Affine;
const BASE_FIELD_MODULUS: &str =
    "21888242871839275222246405745257275088696311157297823662689037894645226208583";

fn parse_biguint_to_fq(value: &str) -> Fq {
    let big_int = BigUint::parse_bytes(value.as_bytes(), 10).unwrap();
    Fq::from(big_int)
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

fn scalar_mul(p_x: BigUint, p_y: BigUint, k: BigUint) -> G1 {
    let p = G1::new_unchecked(
        parse_biguint_to_fq(&p_x.to_string()),
        parse_biguint_to_fq(&p_y.to_string()),
    );
    let scalar = Fr::from_be_bytes_mod_order(&k.to_bytes_be());
    (p.into_group() * scalar).into_affine()
}

// todo: generic public inputs
pub fn verify_groth16_proof(
    pi_a: G1Affine,
    pi_b: G2Affine,
    pi_c: G1Affine,
    vk_alpha1: G1Affine,
    vk_beta2: G2Affine,
    vk_gamma2: G2Affine,
    vk_delta2: G2Affine,
    ics: Vec<G1Affine>,
    inputs: Vec<BigUint>,
) -> bool {
    let mut vk_x: G1 = ics[0];
    for (idx, ic) in ics.into_iter().enumerate().skip(1) {
        let ic_coords = extract_g1_coordinates(ic);
        let ic_scalar: G1 = scalar_mul(ic_coords.0, ic_coords.1, inputs[idx - 1].clone());
        println!("processed input: {}, with ic: {}", inputs[idx - 1], idx);
        let ic_scalar_coords = extract_g1_coordinates(ic_scalar);
        let vk_x_as_coordinates = extract_g1_coordinates(vk_x);
        let vk_x_as_coords = vk_x_as_coordinates.clone();
        vk_x = add_g1_as_coordinates(
            vk_x_as_coords.0,
            vk_x_as_coords.1,
            ic_scalar_coords.0,
            ic_scalar_coords.1,
        );
    }

    // compute pairing result and return is_zero?
    <Bn<Config> as Pairing>::multi_pairing(
        vec![negate_g1_affine(pi_a), vk_alpha1, vk_x, pi_c],
        vec![pi_b, vk_beta2, vk_gamma2, vk_delta2],
    )
    .is_zero()
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use ark_bn254::{Fq2, G1Affine, G2Affine};
    use num_bigint::BigUint;

    use crate::{parse_biguint_to_fq, verify_groth16_proof, G1};
    type G2 = ark_bn254::g2::G2Affine;

    fn parse_biguint_to_fq2(value1: &str, value2: &str) -> Fq2 {
        let fq1 = parse_biguint_to_fq(value1);
        let fq2 = parse_biguint_to_fq(value2);
        Fq2::new(fq1, fq2)
    }
    #[test]
    fn circom_multiplier_2() {
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
        assert!(verify_groth16_proof(
            pi_a,
            pi_b,
            pi_c,
            vk_alpha1,
            vk_beta2,
            vk_gamma2,
            vk_delta2,
            vec![ic_0, ic_1],
            vec![BigUint::from_str("33").unwrap()]
        ));
    }
    #[test]
    fn circuit_with_public_inputs() {
        let inputs = vec![
            BigUint::from_str("33").unwrap(),
            BigUint::from_str("3").unwrap(),
            BigUint::from_str("5").unwrap(),
        ];
        let pi_a_x = parse_biguint_to_fq(
            "19392468517452974577942618696005895384800799906042106318697233463721693766857",
        );
        let pi_a_y = parse_biguint_to_fq(
            "11733184222349063754296049194104702852248466442201114423019855124829727281495",
        );
        let pi_a: G1 = G1Affine::new_unchecked(pi_a_x, pi_a_y);

        let pi_c_x = parse_biguint_to_fq(
            "14537178142063348772247784963013529007912999377457777806993774035571456724739",
        );
        let pi_c_y = parse_biguint_to_fq(
            "17288173778642609314695611486482435460623347370761147350405389833042911834390",
        );
        let pi_c: G1 = G1Affine::new_unchecked(pi_c_x, pi_c_y);

        let pi_b_x = parse_biguint_to_fq2(
            "7870180900678843028456178167017451907138106017914540035097663772922052759069",
            "2676154602589869463817353172490741301223256773047921497031846934197445742235",
        );
        let pi_b_y = parse_biguint_to_fq2(
            "14244550656158180977726930281401023179485400919911817896878773580119256293941",
            "9995198113125036563130298991985119281424711885618696805083921479233677642060",
        );
        let pi_b: G2 = G2Affine::new_unchecked(pi_b_x, pi_b_y);
        let vk_alpha1_x = parse_biguint_to_fq(
            "1492340889437497096222099246540603464242089375646843408401381497321297191805",
        );
        let vk_alpha1_y = parse_biguint_to_fq(
            "11206096956007645304738557692578347108012874917451451037218479742065106409283",
        );
        let vk_alpha1: G1 = G1Affine::new_unchecked(vk_alpha1_x, vk_alpha1_y);
        let vk_beta2_x = parse_biguint_to_fq2(
            "6819705648602020464830649412138262446645951538756802487947753732543012497761",
            "11219895958388416928800243793178587081231733551464793980171225783205073571066",
        );
        let vk_beta2_y = parse_biguint_to_fq2(
            "16232931317995312889893177026572807048495149241311423376955082994080106409796",
            "221661055415397359078497694134150575803375790398012292192745950633940107116",
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
            "5808924139029823792446683085355576723597107871161321088950475604373452728409",
            "794006949025015063691630962823267254566632109771507942299080649574885489297",
        );
        let vk_delta_2_y = parse_biguint_to_fq2(
            "8755580072416395880353332329707061182225307801858931969661521444593294405758",
            "6753206114197090706093517144874887058584442501305676249216528764670697270591",
        );
        let vk_delta2 = G2Affine::new_unchecked(vk_delta_2_x, vk_delta_2_y);

        let ic_0_x = parse_biguint_to_fq(
            "10271593014494639556154917775587497160139512735158233514771987430693691505171",
        );
        let ic_0_y = parse_biguint_to_fq(
            "820244293775287856216015804235186748836699371502118506034976181750078184820",
        );
        let ic_0: G1 = G1Affine::new_unchecked(ic_0_x, ic_0_y);

        let ic_1_x = parse_biguint_to_fq(
            "2280705947019161452433451373159244292742431715288144611519626933019071363786",
        );
        let ic_1_y = parse_biguint_to_fq(
            "14167304281910676563969694680310119449755461008189016344190787198178442130210",
        );

        let ic_1: G1 = G1Affine::new_unchecked(ic_1_x, ic_1_y);

        let ic_2_x = parse_biguint_to_fq(
            "18065151204330767741864558320702649470751716898622025547025773925205377458663",
        );
        let ic_2_y = parse_biguint_to_fq(
            "12530120613599435509444558723909129574908256194829780222525439733802640757968",
        );

        let ic_2: G1 = G1Affine::new_unchecked(ic_2_x, ic_2_y);

        let ic_3_x = parse_biguint_to_fq(
            "2515573466743927184129285920552961694034693235978720556942741443996060153714",
        );
        let ic_3_y = parse_biguint_to_fq(
            "10527719347406676325186974791933879637257851126926242922361792698025261451931",
        );

        let ic_3: G1 = G1Affine::new_unchecked(ic_3_x, ic_3_y);

        let ics = vec![ic_0, ic_1, ic_2, ic_3];
        assert!(verify_groth16_proof(
            pi_a, pi_b, pi_c, vk_alpha1, vk_beta2, vk_gamma2, vk_delta2, ics, inputs
        ));
    }
}
