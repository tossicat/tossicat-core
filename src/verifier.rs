//! # 올바른 파라미터로 주어졌는지 확인하는 모듈
//!
//! 이 모듈은 입력된 단어와 토시가 적절한 것들인지 검사하는 모듈이다.
//! 최종 함수는 `verifiers()`이다.

const TOSSI_LIST: [&str; 112] = [
    "(가)이",
    "(과)와",
    "(는)은",
    "(를)을",
    "(와)과",
    "(으)로",
    "(으)로부터",
    "(으)로서",
    "(으)로써",
    "(은)는",
    "(을)를",
    "(이)가",
    "(이)고",
    "(이)나",
    "(이)나마",
    "(이)니",
    "(이)다",
    "(이)든",
    "(이)든가",
    "(이)든지",
    "(이)라도",
    "(이)라야",
    "(이)란",
    "(이)랑",
    "(이)며",
    "(이)야말로",
    "(이)여",
    "가",
    "가(이)",
    "고",
    "과",
    "과(와)",
    "나",
    "나마",
    "는",
    "는(은)",
    "니",
    "다",
    "든",
    "든가",
    "든지",
    "라도",
    "라야",
    "란",
    "랑",
    "로",
    "로부터",
    "로서",
    "로써",
    "를",
    "를(을)",
    "며",
    "야말로",
    "여",
    "와",
    "와(과)",
    "으로",
    "으로부터",
    "으로서",
    "으로써",
    "은",
    "은(는)",
    "을",
    "을(를)",
    "이",
    "이(가)",
    "이고",
    "이나",
    "이나마",
    "이니",
    "이다",
    "이든",
    "이든가",
    "이든지",
    "이라도",
    "이라야",
    "이란",
    "이랑",
    "이며",
    "이야말로",
    "이여",
    "같이",
    "거나",
    "게",
    "까지",
    "께",
    "께서",
    "대로",
    "도",
    "마냥",
    "마다",
    "마저",
    "만",
    "밖에",
    "보다",
    "부터",
    "뿐",
    "에",
    "에게",
    "에게로",
    "에게서",
    "에다가",
    "에서",
    "에서부터",
    "의",
    "이다",
    "조차",
    "처럼",
    "커녕",
    "하고",
    "한테",
    "한테서",
];

use crate::error::ValueErrorType;

/// 변환하기 전에 입력된 것들이 변환가능한 것인지 검사하는 함수
/// 위에서부터 아래 조건 문을 순서대로 살펴 보겠다.
///
/// 1. 단어는 마지막 글자가 한글이나 숫자이면 된다.
/// 2. 토시는 한글이면 된다.
/// 3. 변환할 수 있는 토시인지 아닌지 파악한다.
/// 4. 단어의 길이가 50자를 넘으면 처리하지 않도록 처리한다.
///
/// 이 4가지를 만족하면 본 작업인 글자에 맞게 토시를 변환하게 된다.
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

    // word에 한글이 들어가면 에러가 나야 합니다.
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
