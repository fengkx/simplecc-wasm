//! A set of built-in OpenCC dictionaries.
//!
//! Turn on `builtin_dicts` feature to enable them.
use simplecc::Dict;
use std::sync::LazyLock;

macro_rules! builtin_dicts {
    ( $x:expr ) => {
        {
            let lines = include_str!(
                concat!("../OpenCC/data/dictionary/", $x, ".txt")).lines();
            Dict::load_lines(lines)
        }
    };
    ( $x:expr $(, $y:expr )+ ) => {
        {
            let lines = include_str!(
                concat!("../OpenCC/data/dictionary/", $x, ".txt")).lines();
            let mut lines = Box::new(lines) as Box<dyn Iterator<Item=&'static str>>;
            $(
                let text = include_str!(
                    concat!("../OpenCC/data/dictionary/", $y, ".txt"));
                lines = Box::new(lines.chain(text.lines()));
            )+
            Dict::load_lines(lines)
        }
    };
}

/// Simplified Chinese to Traditional Chinese
pub static S2T: LazyLock<Dict> = LazyLock::new(|| builtin_dicts!("STCharacters", "STPhrases"));

/// Traditional Chinese to Simplified Chinese
pub static T2S: LazyLock<Dict> = LazyLock::new(|| builtin_dicts!("TSCharacters", "TSPhrases"));

/// Simplified Chinese to Traditional Chinese (Taiwan Standard)
pub static S2TW: LazyLock<Dict> = LazyLock::new(|| S2T.clone().chain(builtin_dicts!("TWVariants")));

/// Simplified Chinese to Traditional Chinese (Hong Kong Standard)
pub static S2HK: LazyLock<Dict> = LazyLock::new(|| S2T.clone().chain(builtin_dicts!("HKVariants")));

/// Simplified Chinese to Traditional Chinese (Taiwan Standard) with
pub static S2TWP: LazyLock<Dict> = LazyLock::new(|| {
    S2T.clone().chain(builtin_dicts!(
        "TWVariants",
        "TWPhrasesIT",
        "TWPhrasesName",
        "TWPhrasesOther"
    ))
});

pub static HK2S: LazyLock<Dict> =
    LazyLock::new(|| T2S.clone().chain(builtin_dicts!("HKVariantsRevPhrases")));

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ( $d:path, $x:expr ) => {
            let input = include_str!(concat!("../OpenCC/test/testcases/", $x, ".in"));
            let ans = include_str!(concat!("../OpenCC/test/testcases/", $x, ".ans"));
            assert_eq!(ans, $d.replace_all(input));
        };
    }

    #[test]
    fn test_builtin_opencc() {
        test!(S2T, "s2t");
        test!(T2S, "t2s");
    }

    #[test]
    fn test_builtin_opencc_chain() {
        test!(S2TW, "s2tw");
        test!(S2HK, "s2hk");
        test!(S2TWP, "s2twp");
    }
}
