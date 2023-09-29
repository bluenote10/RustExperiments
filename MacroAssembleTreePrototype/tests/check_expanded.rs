#[test]
pub fn check_expanded() {
    macrotest::expand("tests/expand/*.rs");
}
