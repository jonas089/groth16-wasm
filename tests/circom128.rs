use bn::AffineG1;
use casper_groth16::bn128::{
    alt_bn128_pairing, compute_vk, fq_from_u256, fq_to_u256, negate_g1_affine,
};
use casper_types::U256;

#[test]
fn circom_multiplier_2() {
    // AX
    let pi_a_x = U256::from_str_radix(
        "4619434547164325081923648243067958995814461722276790408259976269673531268875",
        10,
    )
    .unwrap();
    // AY
    let pi_a_y = U256::from_str_radix(
        "17285941344797724749074955491828477791926771489034344863858176130130219822865",
        10,
    )
    .unwrap();
    // CX
    let pi_c_x = U256::from_str_radix(
        "2224906812514985819002007785400739200833587017118171662746436788881490639334",
        10,
    )
    .unwrap();
    // CY
    let pi_c_y = U256::from_str_radix(
        "17575872684026867761893584228054463905548398624577391451682634656255301190545",
        10,
    )
    .unwrap();
    // BX
    let pi_b_x = U256::from_str_radix(
        "7493377171278660922342026159516494202893397635160892892797904546053101726860",
        10,
    )
    .unwrap();
    let pi_b_x2 = U256::from_str_radix(
        "12257015281543965245685445974249405875916234863299766453693211602557670657219",
        10,
    )
    .unwrap();
    // BY
    let pi_b_y = U256::from_str_radix(
        "12131353492675488324271920506889811484612170039713745676687476036748951969131",
        10,
    )
    .unwrap();
    let pi_b_y2 = U256::from_str_radix(
        "5187697901168563347516107227846365175711629678791848343161631452197878544126",
        10,
    )
    .unwrap();
    // ALPHAX
    let vk_alpha1_x = U256::from_str_radix(
        "10246350822467771900076635245792972119666566556250807950902733806864247380952",
        10,
    )
    .unwrap();
    // ALPHAY
    let vk_alpha1_y = U256::from_str_radix(
        "608411288378915329930935766447369940767930506471659681097230521603283651905",
        10,
    )
    .unwrap();
    // BETAX
    let vk_beta2_x = U256::from_str_radix(
        "6131344741220743386799335429820992680362925873963442544072984714378368926041",
        10,
    )
    .unwrap();
    let vk_beta2_x2 = U256::from_str_radix(
        "15789153394103558986310497145299360386833033851225792260568730098540011835894",
        10,
    )
    .unwrap();
    // BETAY
    let vk_beta2_y = U256::from_str_radix(
        "20294744769931145130063498330622344384466672603336352492159120958989063471433",
        10,
    )
    .unwrap();
    let vk_beta2_y2 = U256::from_str_radix(
        "3758612818443493808972214480762460937559058096828360946639526592835030859803",
        10,
    )
    .unwrap();
    // GAMMAX
    let vk_gamma2_x = U256::from_str_radix(
        "10857046999023057135944570762232829481370756359578518086990519993285655852781",
        10,
    )
    .unwrap();
    let vk_gamma2_x2 = U256::from_str_radix(
        "11559732032986387107991004021392285783925812861821192530917403151452391805634",
        10,
    )
    .unwrap();
    // GAMMAY
    let vk_gamma2_y = U256::from_str_radix(
        "8495653923123431417604973247489272438418190587263600148770280649306958101930",
        10,
    )
    .unwrap();
    let vk_gamma2_y2 = U256::from_str_radix(
        "4082367875863433681332203403145435568316851327593401208105741076214120093531",
        10,
    )
    .unwrap();
    // DELTAX
    let vk_delta_2_x = U256::from_str_radix(
        "2331685158934782270621884102594249521613050557963549726699028399736205391535",
        10,
    )
    .unwrap();
    let vk_delta_2_x2 = U256::from_str_radix(
        "19932904864070474666569306255777842591060844877329635027414969502137306204189",
        10,
    )
    .unwrap();
    // DELTA_Y
    let vk_delta_2_y = U256::from_str_radix(
        "18328176957461925860223052153948913273697229957014116201548221893444067392668",
        10,
    )
    .unwrap();
    let vk_delta_2_y2 = U256::from_str_radix(
        "4892040004975702242175034718975862230235444061193165072087100231911981786509",
        10,
    )
    .unwrap();

    let ic_0_x = U256::from_str_radix(
        "21631942485326744232766849971585115612456593023934275850499378648736190910977",
        10,
    )
    .unwrap();
    let ic_0_y = U256::from_str_radix(
        "10990468352600828980319524627816836646396500759270877213016615483259184677726",
        10,
    )
    .unwrap();

    let ic_1_x = U256::from_str_radix(
        "21229468961321243348662110358869948527418599923035918852855987234632719885365",
        10,
    )
    .unwrap();
    let ic_1_y = U256::from_str_radix(
        "14718418867019175107712538434554605791301866350066611533272126162199859274702",
        10,
    )
    .unwrap();

    let a_neg = negate_g1_affine(pi_a_x, pi_a_y);
    let ics = vec![
        AffineG1::new(fq_from_u256(ic_0_x), fq_from_u256(ic_0_y)).unwrap(),
        AffineG1::new(fq_from_u256(ic_1_x), fq_from_u256(ic_1_y)).unwrap(),
    ];
    let inputs = vec![U256::from(33)];
    /*
        vec![negate_g1_affine(pi_a), vk_alpha1, vk_x, pi_c],
        vec![pi_b, vk_beta2, vk_gamma2, vk_delta2],
    */
    // ax, ay, bay, bax, bby, bbx
    let vk = compute_vk(ics, inputs);
    let result = alt_bn128_pairing(vec![
        (a_neg.0, a_neg.1, pi_b_y, pi_b_x, pi_b_y2, pi_b_x2),
        (
            vk_alpha1_x,
            vk_alpha1_y,
            vk_beta2_x,
            vk_beta2_y,
            vk_beta2_x2,
            vk_beta2_y2,
        ),
        (
            vk.0,
            vk.1,
            vk_gamma2_x,
            vk_gamma2_y,
            vk_gamma2_x2,
            vk_gamma2_y2,
        ),
        (
            pi_c_x,
            pi_c_y,
            vk_delta_2_x,
            vk_delta_2_y,
            vk_delta_2_x2,
            vk_delta_2_y2,
        ),
    ]);
    println!("Result: {}", &result);
}
