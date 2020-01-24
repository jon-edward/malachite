use std::u16;

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::unsigneds;

fn split_in_half_helper<T: JoinHalves + PrimitiveUnsigned + SplitInHalf + Rand>() {
    test_properties(unsigneds, |&n: &T| {
        let (upper, lower) = n.split_in_half();
        assert_eq!(T::join_halves(upper, lower), n)
    });
}

#[test]
fn split_in_half_properties() {
    split_in_half_helper::<u16>();
    split_in_half_helper::<u32>();
    split_in_half_helper::<u64>();
}