use tossicat::bracket::modify_pairs;

#[test]
fn _modify_pairs() {
    let temp: String = "{철수, 은} {영희,   과} {밥,  를} 먹습니다.".to_owned();
    let result = Ok(vec![
        ("철수, 은".to_owned(), "철수".to_owned(), "은".to_owned()),
        ("영희,   과".to_owned(), "영희".to_owned(), "과".to_owned()),
        ("밥,  를".to_owned(), "밥".to_owned(), "를".to_owned()),
    ]);
    assert_eq!(result, modify_pairs(&temp));
}

#[test]
fn _modify_pairs_error_not_balanced() {
    // 괄호 짝이 맞지 않는 경우
    let temp = "{철수, 은".to_owned();
    assert!(modify_pairs(&temp).is_err());
}

#[test]
fn _modify_pairs_error_is_not_brace() {
    // 중괄호가 아닌 괄호를 사용한 경우
    let temp = "[철수, 은]".to_owned();
    assert!(modify_pairs(&temp).is_err());
}

#[test]
fn _modify_pairs_error_nested() {
    // 중첩된 중괄호가 있는 경우
    let temp = "{{철수, 은}}".to_owned();
    assert!(modify_pairs(&temp).is_err());
}

#[test]
fn _modify_pairs_error_no_comma() {
    // 쉼표 없이 입력한 경우
    let temp = "{철수 은}".to_owned();
    assert!(modify_pairs(&temp).is_err());
}

#[test]
fn _modify_pairs_error_empty_tossi() {
    // 토시가 비어 있는 경우
    let temp = "{철수,}".to_owned();
    assert!(modify_pairs(&temp).is_err());
}

#[test]
fn _modify_pairs_error_empty_word() {
    // 단어가 비어 있는 경우
    let temp = "{,은}".to_owned();
    assert!(modify_pairs(&temp).is_err());
}
