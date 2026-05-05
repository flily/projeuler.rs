pub mod p0001;
pub mod p0002;
pub mod p0003;
pub mod p0004;
pub mod p0014;
pub mod p0022;
pub mod p0092;

use crate::common::Problem;

pub fn all_problems() -> Vec<&'static Problem> {
    vec![
        &p0001::INFO,
        &p0002::INFO,
        &p0003::INFO,
        &p0004::INFO,
        &p0014::INFO,
        &p0022::INFO,
        &p0092::INFO,
    ]
}
