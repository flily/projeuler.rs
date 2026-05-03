pub mod p0001;
pub mod p0014;

use crate::common::Problem;

pub fn all_problems() -> Vec<&'static Problem> {
    vec![
        &p0001::INFO,
        &p0014::INFO,
    ]
}
