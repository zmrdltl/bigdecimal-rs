// tests for function bigdecimal::parsing::parse_from_f32

use paste::paste;

use stdlib::f32;

macro_rules! impl_test {
    ($name:ident : $input:literal == $expected:literal) => {
        paste! {
            #[test]
            fn [< case $name >]() {
                let n = $input as f32;
                let d = parse_from_f32(n);
                assert_eq!(d, $expected.parse().unwrap());
            }

            #[test]
            fn [< case_neg $name >]() {
                let n = -($input as f32);
                let d = parse_from_f32(n);
                assert_eq!(d, concat!("-", $expected).parse().unwrap());
            }
        }
    };
}


impl_test!(_0 : 0.0 == "0");
impl_test!(_1 : 1.0 == "1");
impl_test!(_5en1 : 0.5 == "0.5");
impl_test!(_25en2 : 0.25 == "0.25");
impl_test!(_50 : 50. == "50");
impl_test!(_1en3 : 0.001 == "0.001000000047497451305389404296875");
impl_test!(_033203125en8 : 0.033203125 == "0.033203125");

impl_test!(_45En1 : 4.5 == "4.5");
impl_test!(_15625En5 : 0.15625 == "0.15625");
impl_test!(_1192092896En7 : 1.192092896e-7 == "1.1920928955078125E-7");
impl_test!(_1401757440 : 1401757440. == "1401757440");
impl_test!(_215092En1 : 21509.2 == "21509.19921875");
impl_test!(_2289620000 : 2289620000.0 == "2289619968");
impl_test!(_10000000 : 10000000. == "10000000");
impl_test!(_1en05 : 1e-5 == "0.00000999999974737875163555145263671875");
impl_test!(_1en1 : 1e-1 == "0.100000001490116119384765625");
impl_test!(_2en1 : 2e-1 == "0.20000000298023223876953125");
impl_test!(_80000197 : 80000197e0 == "80000200");
impl_test!(_23283064En16 : 2.3283064e-10 == "0.00000000023283064365386962890625");
impl_test!(_14693861798803098En17 : 0.14693861798803098 == "0.146938621997833251953125");
impl_test!(_1e20 : 1e20 == "100000002004087734272");
impl_test!(_1e30 : 1e30 == "1000000015047466219876688855040");
impl_test!(_1e38 : 1e38 == "99999996802856924650656260769173209088");
impl_test!(_317e36 : 317e36 == "317000006395220278118691742155288870912");
impl_test!(_23509889819en48 : 2.3509889819e-38 == "2.35098898190426788090088725919040801362055736959656341832065776397049129686767088287524529732763767242431640625E-38");
impl_test!(_235098744048en49 : 2.35098744048e-38 == "2.350987440475957123602109243087866394712812961308427354153308831195379018097479928428583662025630474090576171875E-38");
impl_test!(_6_99999952316 : 6.99999952316 == "6.999999523162841796875");
impl_test!(_317en40 : 317e-40 == "3.1700000098946435501119816090716154772221806896649747100732700841687651538425285480116144753992557525634765625E-38");
impl_test!(_4294967295 : 4294967295. == "4294967296");
impl_test!(_158456325029e18 : 1.58456325029e+29 == "158456325028528675187087900672");


#[test]
fn case_f32_min() {
    let n = f32::MIN;
    let d = parse_from_f32(n);
    assert_eq!(d, "-340282346638528859811704183484516925440".parse().unwrap());
}

#[test]
fn case_f32_max() {
    let n = f32::MAX;
    let d = parse_from_f32(n);
    assert_eq!(d, "340282346638528859811704183484516925440".parse().unwrap());
}

#[test]
fn case_f32_epsilon() {
    let n = f32::EPSILON;
    let d = parse_from_f32(n);
    assert_eq!(d, "1.1920928955078125E-7".parse().unwrap());
}

#[test]
fn case_f32_pi() {
    let n = f32::consts::PI;
    let d = parse_from_f32(n);
    assert_eq!(d, "3.1415927410125732421875".parse().unwrap());
}

#[test]
fn case_nan() {
    let n = f32::from_bits(0b01111111110000000000000000000000);
    assert!(n.is_nan());

    let d = parse_from_f32(n);
    assert_eq!(d, "510423550381407695195061911147652317184".parse().unwrap());
}

#[test]
fn case_try_from_nan() {
    let n = f32::NAN;
    let d = try_parse_from_f32(n);
    assert!(d.is_err());
}

#[test]
fn case_try_from_infinity() {
    let n = f32::INFINITY;
    let d = try_parse_from_f32(n);
    assert!(d.is_err());
}
