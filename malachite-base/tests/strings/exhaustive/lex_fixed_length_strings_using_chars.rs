use std::iter::{empty, once};

use malachite_base::strings::exhaustive::lex_fixed_length_strings_using_chars;

fn lex_fixed_length_strings_using_chars_helper<I: Iterator<Item = char>>(
    len: u64,
    cs: I,
    out: &[&str],
) {
    let css = lex_fixed_length_strings_using_chars(len, cs)
        .take(20)
        .collect::<Vec<_>>();
    assert_eq!(
        css.iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .as_slice(),
        out
    );
}

#[test]
fn test_lex_fixed_length_strings_using_chars() {
    lex_fixed_length_strings_using_chars_helper(0, empty(), &[""]);
    lex_fixed_length_strings_using_chars_helper(1, empty(), &[]);
    lex_fixed_length_strings_using_chars_helper(0, once('q'), &[""]);
    lex_fixed_length_strings_using_chars_helper(1, once('q'), &["q"]);
    lex_fixed_length_strings_using_chars_helper(2, once('q'), &["qq"]);
    lex_fixed_length_strings_using_chars_helper(3, once('q'), &["qqq"]);
    lex_fixed_length_strings_using_chars_helper(10, once('q'), &["qqqqqqqqqq"]);
    lex_fixed_length_strings_using_chars_helper(0, ['x', 'y'].iter().cloned(), &[""]);
    lex_fixed_length_strings_using_chars_helper(1, ['x', 'y'].iter().cloned(), &["x", "y"]);
    lex_fixed_length_strings_using_chars_helper(
        2,
        ['x', 'y'].iter().cloned(),
        &["xx", "xy", "yx", "yy"],
    );
    lex_fixed_length_strings_using_chars_helper(
        3,
        ['x', 'y'].iter().cloned(),
        &["xxx", "xxy", "xyx", "xyy", "yxx", "yxy", "yyx", "yyy"],
    );
    lex_fixed_length_strings_using_chars_helper(
        10,
        ['x', 'y'].iter().cloned(),
        &[
            "xxxxxxxxxx",
            "xxxxxxxxxy",
            "xxxxxxxxyx",
            "xxxxxxxxyy",
            "xxxxxxxyxx",
            "xxxxxxxyxy",
            "xxxxxxxyyx",
            "xxxxxxxyyy",
            "xxxxxxyxxx",
            "xxxxxxyxxy",
            "xxxxxxyxyx",
            "xxxxxxyxyy",
            "xxxxxxyyxx",
            "xxxxxxyyxy",
            "xxxxxxyyyx",
            "xxxxxxyyyy",
            "xxxxxyxxxx",
            "xxxxxyxxxy",
            "xxxxxyxxyx",
            "xxxxxyxxyy",
        ],
    );
    lex_fixed_length_strings_using_chars_helper(0, ['c', 'a', 't'].iter().cloned(), &[""]);
    lex_fixed_length_strings_using_chars_helper(
        1,
        ['c', 'a', 't'].iter().cloned(),
        &["c", "a", "t"],
    );
    lex_fixed_length_strings_using_chars_helper(
        2,
        ['c', 'a', 't'].iter().cloned(),
        &["cc", "ca", "ct", "ac", "aa", "at", "tc", "ta", "tt"],
    );
    lex_fixed_length_strings_using_chars_helper(
        3,
        ['c', 'a', 't'].iter().cloned(),
        &[
            "ccc", "cca", "cct", "cac", "caa", "cat", "ctc", "cta", "ctt", "acc", "aca", "act",
            "aac", "aaa", "aat", "atc", "ata", "att", "tcc", "tca",
        ],
    );
    lex_fixed_length_strings_using_chars_helper(
        10,
        ['c', 'a', 't'].iter().cloned(),
        &[
            "cccccccccc",
            "ccccccccca",
            "ccccccccct",
            "ccccccccac",
            "ccccccccaa",
            "ccccccccat",
            "cccccccctc",
            "ccccccccta",
            "cccccccctt",
            "cccccccacc",
            "cccccccaca",
            "cccccccact",
            "cccccccaac",
            "cccccccaaa",
            "cccccccaat",
            "cccccccatc",
            "cccccccata",
            "cccccccatt",
            "ccccccctcc",
            "ccccccctca",
        ],
    );
}
