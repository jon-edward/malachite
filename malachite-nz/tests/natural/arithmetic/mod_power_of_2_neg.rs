use malachite_base::num::arithmetic::traits::{
    ModNeg, ModPowerOf2, ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Neg,
    ModPowerOf2NegAssign, PowerOf2,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::unsigned_pair_gen_var_17;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_unsigned_pair_gen_var_11;
use std::str::FromStr;

#[test]
fn test_mod_power_of_2_neg() {
    let test = |s, pow, out| {
        let u = Natural::from_str(s).unwrap();

        assert!(u.mod_power_of_2_is_reduced(pow));
        let n = u.clone().mod_power_of_2_neg(pow);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
        assert!(n.mod_power_of_2_is_reduced(pow));

        let n = (&u).mod_power_of_2_neg(pow);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = u;
        n.mod_power_of_2_neg_assign(pow);
        assert_eq!(n.to_string(), out);
    };
    test("0", 5, "0");
    test("10", 4, "6");
    test("100", 8, "156");
    test("1", 32, "4294967295");
    test("100", 100, "1267650600228229401496703205276");
    test("1267650600228229401496703205276", 100, "100");
}

#[test]
fn mod_power_of_2_neg_properties() {
    natural_unsigned_pair_gen_var_11().test_properties(|(n, pow)| {
        assert!(n.mod_power_of_2_is_reduced(pow));
        let neg = (&n).mod_power_of_2_neg(pow);
        assert!(neg.is_valid());
        assert!(neg.mod_power_of_2_is_reduced(pow));

        let neg_alt = n.clone().mod_power_of_2_neg(pow);
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        let mut n_alt = n.clone();
        n_alt.mod_power_of_2_neg_assign(pow);
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        assert_eq!(neg, (-&n).mod_power_of_2(pow));
        assert_eq!(neg, (&n).mod_neg(Natural::power_of_2(pow)));
        assert_eq!((&neg).mod_power_of_2_neg(pow), n);
        assert_eq!((&n).mod_power_of_2_add(&neg, pow), 0);
        assert_eq!(
            n == neg,
            n == Natural::ZERO || n == Natural::power_of_2(pow - 1)
        );
    });

    unsigned_pair_gen_var_17::<Limb>().test_properties(|(n, pow)| {
        assert_eq!(
            n.mod_power_of_2_neg(pow),
            Natural::from(n).mod_power_of_2_neg(pow)
        );
    });
}
