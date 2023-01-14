use tossicat::{modify_sentence, postfix};

#[test]
fn _modify_sentence() {
    let test = "{철수, 은} {영희, 과} {밥, 를} 먹습니다.";
    let result = Ok("철수는 영희와 밥을 먹습니다.".to_string());
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
}
