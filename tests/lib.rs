use tossicat::{postfix, value_verifier};

#[test]
fn _postfix() {
    // '으로', '로' 테스트
    let word = "집";
    let tossi = "으로";
    let result = "집으로";
    assert_eq!(result, postfix(word, tossi));

    let word = "집";
    let tossi = "로";
    let result = "집으로";
    assert_eq!(result, postfix(word, tossi));

    let word = "집";
    let tossi = "(으)로";
    let result = "집으로";
    assert_eq!(result, postfix(word, tossi));

    let word = "나무";
    let tossi = "으로";
    let result = "나무로";
    assert_eq!(result, postfix(word, tossi));

    let word = "나무";
    let tossi = "로";
    let result = "나무로";
    assert_eq!(result, postfix(word, tossi));

    let word = "나무";
    let tossi = "(으)로";
    let result = "나무로";
    assert_eq!(result, postfix(word, tossi));

    let word = "칼";
    let tossi = "으로";
    let result = "칼로";
    assert_eq!(result, postfix(word, tossi));

    let word = "칼";
    let tossi = "로";
    let result = "칼로";
    assert_eq!(result, postfix(word, tossi));

    let word = "칼";
    let tossi = "(으)로";
    let result = "칼로";
    assert_eq!(result, postfix(word, tossi));

    let word = "test";
    let tossi = "으로";
    let result = "test(으)로";
    assert_eq!(result, postfix(word, tossi));

    let word = "test";
    let tossi = "로";
    let result = "test(으)로";
    assert_eq!(result, postfix(word, tossi));

    let word = "test";
    let tossi = "(으)로";
    let result = "test(으)로";
    assert_eq!(result, postfix(word, tossi));

    let word = "1000";
    let tossi = "으로";
    let result = "1000으로";
    assert_eq!(result, postfix(word, tossi));

    let word = "4";
    let tossi = "으로";
    let result = "4로";
    assert_eq!(result, postfix(word, tossi));

    // 한글과 숫자가 같이 들어있는 경우에는 뒷부분에 숫자가 들어 있는 경우
    // 숫자만 뽑아서 처리합니다.
    // 그 예를 추가합니다.
    let word = "천사1004";
    let tossi = "은";
    let result = "천사1004는";
    assert_eq!(result, postfix(word, tossi));
}

#[test]
fn _verifier() {
    // 둘 다 적절하다
    let temp = value_verifier("하하하", "은");
    assert_eq!("Success", temp);
    // 둘 다 적절하다
    let temp =value_verifier("하하하", "는");
    assert_eq!("Success", temp);
    // 단어는 적절하지만, 토시가 적절하지 않다.
    let temp = value_verifier("하하하", "은은");
    assert_eq!("This value is not correct tossi", temp);
    // 단어도 마지막이 한글이고 토시도 적절하지만, 단어 길이가 50 글자 이상이다. 그래서 에러 처리
    let temp_word = "1테트리스2테트리스3테트리스4테트리스5테트리스6테트리스7테트리스8테트리스9테트리스10테트리스";
    let temp = value_verifier(temp_word, "은");
    assert_eq!(
        "The length has been exceeded. Set the word length to less than 50.",
        temp
    );
    // 단어도 마지막이 숫자이고 토시도 적절하지만, 단어 길이가 50 글자 이상이다. 그래서 에러 처리
    let temp_word = "테트리스1테트리스2테트리스3테트리스4테트리스5테트리스6테트리스7테트리스8테트리스9테트리스10";
    let temp = value_verifier(temp_word, "은");
    assert_eq!(
        "The length has been exceeded. Set the word length to less than 50.",
        temp
    );
}
