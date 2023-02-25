//! # 올바른 파라미터로 주어졌는지 확인하는 모듈
//!
//! 이 모듈은 입력된 단어와 토시가 적절한 것들인지 검사하는 모듈이다.
//! 최종 함수는 `verifiers()`이다.

use crate::error::ValueErrorType;
use crate::tossi::TOSSI_LIST;

/// ## 변환하기 전에 입력된 것들이 변환가능한 것인지 검사하는 함수
///
/// 원칙적으로 아래 4가지 정도만 검사하면 이 라이브러리가 다 처리할 수 있다.
///
/// 1. 단어는 마지막 글자가 한글이나 숫자이면 된다.
/// 2. 토시는 한글이면 된다.
/// 3. 변환할 수 있는 토시인지 아닌지 파악한다.
/// 4. 단어의 길이가 50자를 넘으면 처리하지 않도록 처리한다.
///
/// 그러나 1번째 경우는 이 TossiCat Core 라이브러리에서는 단어로
/// 한글과 숫자 이외의 것이 들어오면 이 단어와 같이 입력된 토시를 병기해
/// 처리하기 때문에 굳이 검사할 필요가 없다. 그리고 2번째 경우도 이미 들어온 토시를
/// 처리할 수 있는 것인지 없는 것인지를 판단하는 토시 리스트가 당연히 모두 한글이기
/// 때문에 이 경우도 굳이 검사할 필요가 없다. 따라서 나머지 두 경우만 검사하면 된다.
/// 그래서 이 함수는 아래와 같이 처리하고 있다.
///
/// 1. `is_verifier_tossi` : 변환할 수 있는 토시인지
/// 2. `over_limit_word_len` : 단어의 길이가 50자를 넘는지
///
///
/// 이 2가지를 만족하면 본 작업인 글자에 맞게 토시를 변환하게 된다.
/// 이 함수의 사용법은 `tests/lib.rs`에서 `verifiers()`를 테스트 하는
/// `_verifiers()` 부분을 살펴보시면 됩니다.

pub fn verify_value(word: &str, tossi: &str) -> Result<(), ValueErrorType> {
    if !is_verifier_tossi(tossi) {
        Err(ValueErrorType::InvalidTossi)
    } else if over_limit_word_len(word) {
        Err(ValueErrorType::LimitLength)
    } else {
        Ok(())
    }
}

// 올바른 토씨를 입력했는지 확인해주는 함수
fn is_verifier_tossi(tossi: &str) -> bool {
    for check in TOSSI_LIST.iter() {
        if check == &tossi {
            return true;
        }
    }
    false
}

// 파라미터롤 받는 단어를 제한 기준 함수
fn over_limit_word_len(word: &str) -> bool {
    let limitation = 50;
    if word.chars().count() >= limitation {
        return true;
    }
    false
}

#[test]
fn _over_limit_word_len() {
    let temp = "12345";
    assert_eq!(false, over_limit_word_len(temp));

    let temp = "아이디는 50자까지 설정이 가능합니다.";
    assert_eq!(false, over_limit_word_len(temp));

    let temp = "10000000000000000000000000000000000000000000000000000";
    assert_eq!(true, over_limit_word_len(temp));

    let temp = "테트리스1테트리스2테트리스3테트리스4테트리스5테트리스6테트리스7테트리스8테트리스9테트리스10";
    assert_eq!(true, over_limit_word_len(temp));

    let temp = "1테트리스2테트리스3테트리스4테트리스5테트리스6테트리스7테트리스8테트리스9테트리스10테트리스";
    assert_eq!(true, over_limit_word_len(temp));
}

#[test]
fn _is_verifier_tossi() {
    let temp = "까지";
    assert_eq!(true, is_verifier_tossi(temp));

    let temp = "류현지";
    assert_eq!(false, is_verifier_tossi(temp));
}

#[test]
fn _verifier() {
    let word = "코코아";
    let tossi = "까지";
    assert_eq!(Ok(()), verify_value(word, tossi));

    // 영어 단어가 들어오더라도 토시를 병기에 처리하기 때문에 에러가 날 필요가 없다.
    let word = "Cocoa";
    let tossi = "까지";
    assert_eq!(Ok(()), verify_value(word, tossi));

    let word = "코코아";
    let tossi = "먹고싶다";
    assert_eq!(Err(ValueErrorType::InvalidTossi), verify_value(word, tossi));

    let word = "코코아코코아코코아코코아코코아코코아코코아코코아코코아코코아코코아코코아코코아코코아코코아코코아코코아코코아코코아코코아";
    let tossi = "는";
    assert_eq!(Err(ValueErrorType::LimitLength), verify_value(word, tossi));
}
