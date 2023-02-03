use tossicat::{modify_sentence, postfix};

#[test]
fn _modify_sentence() {
    let test = "{철수, 은} {영희, 과} {밥, 를} 먹습니다.";
    let result = Ok("철수는 영희와 밥을 먹습니다.".to_string());
    assert_eq!(result, modify_sentence(test));

    let test = "{나, 가} {철수, 과} {사과, 을} 먹었습니다.";
    let result = Ok("내가 철수와 사과를 먹었습니다.".to_string());
    assert_eq!(result, modify_sentence(test));

    let test = "{법원, 가} {철수, 과} {영희, 의} {출국, 를} 막았습니다.";
    let result = Ok("법원이 철수와 영희의 출국을 막았습니다.".to_string());
    assert_eq!(result, modify_sentence(test));

    let test = "{철수, 은} {apple, 를} 먹습니다.";
    let result = Ok("철수는 apple(을)를 먹습니다.".to_string());
    assert_eq!(result, modify_sentence(test));

    let test = "{누구, 이} {나, 을} 막을까?";
    let result = Ok("누가 나를 막을까?".to_string());
    assert_eq!(result, modify_sentence(test));
}

#[test]
fn _postfix() {
    // '으로', '로' 테스트
    let word = "집";
    let tossi = "으로";
    let result = Ok("집으로".to_string());
    assert_eq!(result, postfix(word, tossi));

    let word = "집";
    let tossi = "로";
    let result = Ok("집으로".to_string());
    assert_eq!(result, postfix(word, tossi));

    let word = "집";
    let tossi = "(으)로";
    let result = Ok("집으로".to_string());
    assert_eq!(result, postfix(word, tossi));

    let word = "나무";
    let tossi = "으로";
    let result = Ok("나무로".to_string());
    assert_eq!(result, postfix(word, tossi));

    let word = "나무";
    let tossi = "로";
    let result = Ok("나무로".to_string());
    assert_eq!(result, postfix(word, tossi));

    let word = "나무";
    let tossi = "(으)로";
    let result = Ok("나무로".to_string());
    assert_eq!(result, postfix(word, tossi));

    let word = "칼";
    let tossi = "으로";
    let result = Ok("칼로".to_string());
    assert_eq!(result, postfix(word, tossi));

    let word = "칼";
    let tossi = "로";
    let result = Ok("칼로".to_string());
    assert_eq!(result, postfix(word, tossi));

    let word = "칼";
    let tossi = "(으)로";
    let result = Ok("칼로".to_string());
    assert_eq!(result, postfix(word, tossi));

    let word = "test";
    let tossi = "으로";
    let result = Ok("test(으)로".to_string());
    assert_eq!(result, postfix(word, tossi));

    let word = "test";
    let tossi = "로";
    let result = Ok("test(으)로".to_string());
    assert_eq!(result, postfix(word, tossi));

    let word = "test";
    let tossi = "(으)로";
    let result = Ok("test(으)로".to_string());
    assert_eq!(result, postfix(word, tossi));

    let word = "1000";
    let tossi = "으로";
    let result = Ok("1000으로".to_string());
    assert_eq!(result, postfix(word, tossi));

    let word = "4";
    let tossi = "으로";
    let result = Ok("4로".to_string());
    assert_eq!(result, postfix(word, tossi));

    // 한글과 숫자가 같이 들어있는 경우에는 뒷부분에 숫자가 들어 있는 경우
    // 숫자만 뽑아서 처리합니다.
    // 그 예를 추가합니다.
    let word = "천사1004";
    let tossi = "은";
    let result = Ok("천사1004는".to_string());
    assert_eq!(result, postfix(word, tossi));

    // KA(가) 경우, 즉 "가"와 "이" 같은 경우에는 특정 단어가 오게 되면
    // 토시도 변하지만, 특정 단어 또한 변하게 됩니다.
    // 아래는 이런 경우를 테스트하는 것입니다.
    let word = "누구";
    let tossi = "가";
    let result = Ok("누가".to_string());
    assert_eq!(result, postfix(word, tossi));
    let word = "누구";
    let tossi = "이";
    let result = Ok("누가".to_string());
    assert_eq!(result, postfix(word, tossi));
    let word = "나";
    let tossi = "가";
    let result = Ok("내가".to_string());
    assert_eq!(result, postfix(word, tossi));
    let word = "나";
    let tossi = "이";
    let result = Ok("내가".to_string());
    assert_eq!(result, postfix(word, tossi));
    let word = "저";
    let tossi = "가";
    let result = Ok("제가".to_string());
    assert_eq!(result, postfix(word, tossi));
    let word = "저";
    let tossi = "이";
    let result = Ok("제가".to_string());
    assert_eq!(result, postfix(word, tossi));
    let word = "너";
    let tossi = "가";
    let result = Ok("네가".to_string());
    assert_eq!(result, postfix(word, tossi));
    let word = "너";
    let tossi = "이";
    let result = Ok("네가".to_string());
    assert_eq!(result, postfix(word, tossi));
    // KA(가) 경우에서 일반적인 경우 테스트
    // 이건 받침 유무로
    let word = "철수";
    let tossi = "이";
    let result = Ok("철수가".to_string());
    assert_eq!(result, postfix(word, tossi));
    let word = "법원";
    let tossi = "가";
    let result = Ok("법원이".to_string());
    assert_eq!(result, postfix(word, tossi));
}
