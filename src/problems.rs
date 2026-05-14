pub mod p0001;
pub mod p0002;
pub mod p0003;
pub mod p0004;
pub mod p0005;
pub mod p0006;
pub mod p0007;
pub mod p0008;
pub mod p0009;
pub mod p0010;
pub mod p0011;
pub mod p0012;
pub mod p0013;
pub mod p0014;
pub mod p0015;
pub mod p0016;
pub mod p0017;
pub mod p0018;
pub mod p0019;
pub mod p0020;
pub mod p0021;
pub mod p0022;
pub mod p0023;
pub mod p0024;
pub mod p0025;
pub mod p0026;
pub mod p0065;
pub mod p0067;
pub mod p0074;
pub mod p0092;

use crate::common::Problem;

pub fn all_problems() -> Vec<&'static Problem> {
    vec![
        &p0001::INFO,
        &p0002::INFO,
        &p0003::INFO,
        &p0004::INFO,
        &p0005::INFO,
        &p0006::INFO,
        &p0007::INFO,
        &p0008::INFO,
        &p0009::INFO,
        &p0010::INFO,
        &p0011::INFO,
        &p0012::INFO,
        &p0013::INFO,
        &p0014::INFO,
        &p0015::INFO,
        &p0016::INFO,
        &p0017::INFO,
        &p0018::INFO,
        &p0019::INFO,
        &p0020::INFO,
        &p0021::INFO,
        &p0022::INFO,
        &p0023::INFO,
        &p0024::INFO,
        &p0025::INFO,
        &p0026::INFO,
        &p0065::INFO,
        &p0067::INFO,
        &p0074::INFO,
        &p0092::INFO,
    ]
}
