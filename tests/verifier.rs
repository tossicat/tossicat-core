use library::verifiers;

#[test]
#[should_panic(expected = "This value is not correct tossi.")]
fn _verifiers_tossi() {
    verifiers("하하하", "이상한 크기");
}

#[test]
#[should_panic(expected = "The length has been exceeded. Set the word length to less than 50.")]
fn _verifiers_word() {
    verifiers(
        "10000000000000000000000000000000000000000000000000000",
        "은",
    );
}
