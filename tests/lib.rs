use tossicat::{modify_sentence, postfix, transform};

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

    let test = "{아버지, 인들} {자식들, 에게} {밥, 를} 안 줄까?";
    let result = Ok("아버진들 자식들에게 밥을 안 줄까?".to_string());
    assert_eq!(result, modify_sentence(test));

    let test = "전쟁 통에 {공부, 이라고} 어디 제대로 했나. {자네, 라고} 별수 있겠나.";
    let result = Ok("전쟁 통에 공부라고 어디 제대로 했나. 자네라고 별수 있겠나.".to_string());
    assert_eq!(result, modify_sentence(test));

    // '만큼' 검사
    let test = "너는 {학생인, 만큼} 공부에만 전념하여라.";
    let result = Ok("너는 학생인만큼 공부에만 전념하여라.".to_string());
    assert_eq!(result, modify_sentence(test));
}

#[test]
fn _postfix() {
    // 'ㄴ즉', '인즉' 테스트
    let word = "물건";
    let tossi = "인즉";
    let result = Ok("물건인즉".to_string());
    assert_eq!(result, postfix(word, tossi));
    let word = "얘기";
    let tossi = "인즉";
    let result = Ok("얘긴즉".to_string());
    assert_eq!(result, postfix(word, tossi));

    // 'ㄴ들', '인들' 테스트
    let word = "아버지";
    let tossi = "인들";
    let result = Ok("아버진들".to_string());
    assert_eq!(result, postfix(word, tossi));

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

#[test]
fn _transform() {
    // 'ㄴ즉', '인즉' 테스트
    let word = "물건";
    let tossi = "인즉";
    let result = Ok(("물건".to_string(), "인즉".to_string()));
    assert_eq!(result, transform(word, tossi));
    let word = "얘기";
    let tossi = "인즉";
    let result = Ok(("얘긴".to_string(), "즉".to_string()));
    assert_eq!(result, transform(word, tossi));

    // 'ㄴ들', '인들' 테스트
    let word = "아버지";
    let tossi = "인들";
    let result = Ok(("아버진".to_string(), "들".to_string()));
    assert_eq!(result, transform(word, tossi));

    // '으로', '로' 테스트
    let word = "집";
    let tossi = "으로";
    let result = Ok(("집".to_string(), "으로".to_string()));
    assert_eq!(result, transform(word, tossi));

    let word = "집";
    let tossi = "로";
    let result = Ok(("집".to_string(), "으로".to_string()));
    assert_eq!(result, transform(word, tossi));

    let word = "집";
    let tossi = "(으)로";
    let result = Ok(("집".to_string(), "으로".to_string()));
    assert_eq!(result, transform(word, tossi));

    let word = "나무";
    let tossi = "으로";
    let result = Ok(("나무".to_string(), "로".to_string()));
    assert_eq!(result, transform(word, tossi));

    let word = "나무";
    let tossi = "로";
    let result = Ok(("나무".to_string(), "로".to_string()));
    assert_eq!(result, transform(word, tossi));

    let word = "나무";
    let tossi = "(으)로";
    let result = Ok(("나무".to_string(), "로".to_string()));
    assert_eq!(result, transform(word, tossi));

    let word = "칼";
    let tossi = "으로";
    let result = Ok(("칼".to_string(), "로".to_string()));
    assert_eq!(result, transform(word, tossi));

    let word = "칼";
    let tossi = "로";
    let result = Ok(("칼".to_string(), "로".to_string()));
    assert_eq!(result, transform(word, tossi));

    let word = "칼";
    let tossi = "(으)로";
    let result = Ok(("칼".to_string(), "로".to_string()));
    assert_eq!(result, transform(word, tossi));

    let word = "test";
    let tossi = "으로";
    let result = Ok(("test".to_string(), "(으)로".to_string()));
    assert_eq!(result, transform(word, tossi));

    let word = "test";
    let tossi = "로";
    let result = Ok(("test".to_string(), "(으)로".to_string()));
    assert_eq!(result, transform(word, tossi));

    let word = "test";
    let tossi = "(으)로";
    let result = Ok(("test".to_string(), "(으)로".to_string()));
    assert_eq!(result, transform(word, tossi));

    let word = "1000";
    let tossi = "으로";
    let result = Ok(("1000".to_string(), "으로".to_string()));
    assert_eq!(result, transform(word, tossi));

    let word = "4";
    let tossi = "으로";
    let result = Ok(("4".to_string(), "로".to_string()));
    assert_eq!(result, transform(word, tossi));

    // 한글과 숫자가 같이 들어있는 경우에는 뒷부분에 숫자가 들어 있는 경우
    // 숫자만 뽑아서 처리합니다.
    // 그 예를 추가합니다.
    let word = "천사1004";
    let tossi = "은";
    let result = Ok(("천사1004".to_string(), "는".to_string()));
    assert_eq!(result, transform(word, tossi));

    // KA(가) 경우, 즉 "가"와 "이" 같은 경우에는 특정 단어가 오게 되면
    // 토시도 변하지만, 특정 단어 또한 변하게 됩니다.
    // 아래는 이런 경우를 테스트하는 것입니다.
    let word = "누구";
    let tossi = "가";
    let result = Ok(("누".to_string(), "가".to_string()));
    assert_eq!(result, transform(word, tossi));
    let word = "누구";
    let tossi = "이";
    let result = Ok(("누".to_string(), "가".to_string()));
    assert_eq!(result, transform(word, tossi));
    let word = "나";
    let tossi = "가";
    let result = Ok(("내".to_string(), "가".to_string()));
    assert_eq!(result, transform(word, tossi));
    let word = "나";
    let tossi = "이";
    let result = Ok(("내".to_string(), "가".to_string()));
    assert_eq!(result, transform(word, tossi));
    let word = "저";
    let tossi = "가";
    let result = Ok(("제".to_string(), "가".to_string()));
    assert_eq!(result, transform(word, tossi));
    let word = "저";
    let tossi = "이";
    let result = Ok(("제".to_string(), "가".to_string()));
    assert_eq!(result, transform(word, tossi));
    let word = "너";
    let tossi = "가";
    let result = Ok(("네".to_string(), "가".to_string()));
    assert_eq!(result, transform(word, tossi));
    let word = "너";
    let tossi = "이";
    let result = Ok(("네".to_string(), "가".to_string()));
    assert_eq!(result, transform(word, tossi));
    // KA(가) 경우에서 일반적인 경우 테스트
    // 이건 받침 유무로
    let word = "철수";
    let tossi = "이";
    let result = Ok(("철수".to_string(), "가".to_string()));
    assert_eq!(result, transform(word, tossi));
    let word = "법원";
    let tossi = "가";
    let result = Ok(("법원".to_string(), "이".to_string()));
    assert_eq!(result, transform(word, tossi));
}

#[test]
fn _postfix_new_tossi() {
    // 지/이지
    assert_eq!(Ok("사과지".to_string()), postfix("사과", "지"));
    assert_eq!(Ok("사람이지".to_string()), postfix("사람", "지"));
    assert_eq!(Ok("apple(이)지".to_string()), postfix("apple", "지"));

    // 지만/이지만
    assert_eq!(Ok("사과지만".to_string()), postfix("사과", "지만"));
    assert_eq!(Ok("사람이지만".to_string()), postfix("사람", "지만"));
    assert_eq!(Ok("apple(이)지만".to_string()), postfix("apple", "지만"));

    // 니까/이니까
    assert_eq!(Ok("사과니까".to_string()), postfix("사과", "니까"));
    assert_eq!(Ok("사람이니까".to_string()), postfix("사람", "니까"));
    assert_eq!(Ok("apple(이)니까".to_string()), postfix("apple", "니까"));

    // 건/이건
    assert_eq!(Ok("사과건".to_string()), postfix("사과", "건"));
    assert_eq!(Ok("사람이건".to_string()), postfix("사람", "건"));
    assert_eq!(Ok("apple(이)건".to_string()), postfix("apple", "건"));

    // 거든/이거든
    assert_eq!(Ok("사과거든".to_string()), postfix("사과", "거든"));
    assert_eq!(Ok("사람이거든".to_string()), postfix("사람", "거든"));
    assert_eq!(Ok("apple(이)거든".to_string()), postfix("apple", "거든"));

    // 거나/이거나
    assert_eq!(Ok("사과거나".to_string()), postfix("사과", "거나"));
    assert_eq!(Ok("사람이거나".to_string()), postfix("사람", "거나"));
    assert_eq!(Ok("apple(이)거나".to_string()), postfix("apple", "거나"));

    // 냐/이냐
    assert_eq!(Ok("사과냐".to_string()), postfix("사과", "냐"));
    assert_eq!(Ok("사람이냐".to_string()), postfix("사람", "냐"));
    assert_eq!(Ok("apple(이)냐".to_string()), postfix("apple", "냐"));
}

#[test]
fn _postfix_error_invalid_tossi() {
    // 지원하지 않는 조사를 입력한 경우
    let result = postfix("사과", "먹고싶다");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "This value is not correct tossi"
    );
}

#[test]
fn _postfix_error_limit_length() {
    // 50자 이상의 단어를 입력한 경우
    let long_word = "가".repeat(50);
    let result = postfix(&long_word, "을");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "The length has been exceeded. Set the word length to less than 50"
    );
}

#[test]
fn _transform_error_invalid_tossi() {
    let result = transform("사과", "먹고싶다");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "This value is not correct tossi"
    );
}

#[test]
fn _transform_error_limit_length() {
    let long_word = "가".repeat(50);
    let result = transform(&long_word, "을");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "The length has been exceeded. Set the word length to less than 50"
    );
}

#[test]
fn _modify_sentence_error_not_balanced() {
    // 괄호 짝이 맞지 않는 경우
    let result = modify_sentence("{철수, 은");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("incorrect parentheses"));
}

#[test]
fn _modify_sentence_error_is_not_brace() {
    // 중괄호가 아닌 괄호를 사용한 경우
    let result = modify_sentence("[철수, 은]");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("brace"));
}

#[test]
fn _modify_sentence_error_nested_parentheses() {
    // 중첩된 중괄호가 있는 경우
    let result = modify_sentence("{{철수, 은}}");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Nested Parentheses"));
}

#[test]
fn _modify_sentence_error_split_tossi_word() {
    // 쉼표 없이 입력한 경우
    let result = modify_sentence("{철수 은}");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("separate words and tossi with a comma"));
}

#[test]
fn _modify_sentence_error_tossi_is_empty() {
    // 토시가 비어 있는 경우
    let result = modify_sentence("{철수,}");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("fill the tossi section"));
}

#[test]
fn _modify_sentence_error_word_is_empty() {
    // 단어가 비어 있는 경우
    let result = modify_sentence("{,은}");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("fill the word section"));
}

#[test]
fn _modify_sentence_error_invalid_tossi() {
    // 괄호 안에 잘못된 조사가 있는 경우
    let result = modify_sentence("{철수, 먹고싶다}");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("not correct tossi"));
}

#[test]
fn _modify_sentence_error_limit_length() {
    // 괄호 안에 50자 이상의 단어가 있는 경우
    let long_word = "가".repeat(50);
    let sentence = format!("{{{}, 을}}", long_word);
    let result = modify_sentence(&sentence);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("length has been exceeded"));
}
