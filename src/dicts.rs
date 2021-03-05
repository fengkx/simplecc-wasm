//! A set of built-in OpenCC dictionaries.
//!
//! Turn on `builtin_dicts` feature to enable them.
use lazy_static::lazy_static;
use simplecc::Dict;

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

lazy_static! {
    /// Simplified Chinese to Traditional Chinese
    pub static ref S2T: Dict = {
        builtin_dicts!("STCharacters", "STPhrases")
    };

    /// Traditional Chinese to Simplified Chinese
    pub static ref T2S: Dict = {
        builtin_dicts!("TSCharacters", "TSPhrases")
    };

    /// Simplified Chinese to Traditional Chinese (Taiwan Standard)
    pub static ref S2TW: Dict = {
        S2T.clone().chain(builtin_dicts!("TWVariants"))
    };

    /// Simplified Chinese to Traditional Chinese (Hong Kong Standard)
    pub static ref S2HK: Dict = {
        S2T.clone().chain(builtin_dicts!("HKVariants"))
    };

    /// Simplified Chinese to Traditional Chinese (Taiwan Standard) with
    /// Taiwanese idiom
    pub static ref S2TWP: Dict = {
        S2T.clone().chain(builtin_dicts!("TWVariants",
            "TWPhrasesIT", "TWPhrasesName", "TWPhrasesOther"))
    };

    pub static ref HK2S: Dict = {
        T2S.clone().chain(builtin_dicts!("HKVariantsRevPhrases"))
    };



}

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
