//! # 올바른 파라미터로 주어졌는지 확인하는 모듈
//! 이 모듈은 입력된 단어와 토시가 적절한 것들인지 검사하는 모듈이다.
//! 현재 이 프로젝트에서 취급하는 토시(tossi)는 다음과 같다.
//!
//! - 은/는
//! - 이/가
//! - 로/으로
//! - 다/이다
//!
//! 최종 함수는 `verifiers()`이다.

use crate::hangeul::is_hangeul;

const TOSSI_LIST: [&str; 35] = [
    "은", "는", "이", "가", "이다", "다", "으로", "로", "의", "도", "께", "에", "만", "뿐", "보다",
    "같이", "밖에", "부터", "까지", "마냥", "처럼", "마저", "조차", "마냥", "커녕", "을", "를",
    "(으)로", "은(는)", "(는)은", "(이)다", "(을)를", "(를)을", "(이)가", "(가)이",
];

// pub fn verifiers(word: &str, tossi: &str) {
//     match verifier_tossi(tossi) {
//         Ok(_) => (),
//         Err(e) => panic!("{}", e),
//     }
//     match limit_word_len(word) {
//         Ok(_) => (),
//         Err(e) => panic!("{}", e),
//     }
// }

/// ### 한글인지 또는 숫자인지 파악하는 함수
/// 마지막 글자가 한글 또는 숫자인지 아닌지 파악한다.
/// 반환은 `Tuple` 형으로 하는데
/// - 첫 번째 값은 한글인지 아닌지를
/// - 두 번째 깂은 숫자인지 아닌지를
/// 반환한다.
fn is_hangeul_or_number(word: String) -> (bool, bool) {
    let char_vec: Vec<char> = word.chars().collect();
    let last_char = char_vec[char_vec.len() - 1];
    // println!("마지막 글자는: {:?}", last_char);
    return (
        is_hangeul(last_char),
        ('0' <= last_char && last_char <= '9'),
    );
}

/// 변환하기 전에 입력된 것들이 변환가능한 것인지 검사하는 함수
/// 위에서부터 아래 조건 문을 순서대로 살펴 보겠다.
///
/// 1. 단어는 마지막 글자가 한글이나 숫자이면 된다.
/// 2. 토시는 한글이면 된다.
/// 3. 변환할 수 있는 토시인지 아닌지 파악한다.
/// 4. 단어의 길이가 50자를 넘으면 처리하지 않도록 처리한다.
///
/// 이 4가지를 만족하면 본 작업인 글자에 맞게 토시를 변환하게 된다.
pub fn verifiers<'a>(word: &'a str, tossi: &'a str) -> Result<(), &'a str> {
    if is_hangeul_or_number(word.to_string()) == (false, false) {
        return Err("입력하신 단어가 한글도 아니고 숫자도 아닙니다.");
    } else if is_hangeul_or_number(tossi.to_string()).0 == false {
        return Err("입력하신 토시가 한글이 아닙니다.");
    } else if verifier_tossi(tossi) != Ok(()) {
        return verifier_tossi(tossi);
    } else if limit_word_len(word) != Ok(()) {
        return limit_word_len(word);
    } else {
        return Ok(());
    }
}

// 올바른 토씨를 입력했는지 확인해주는 함수
fn verifier_tossi(tossi: &str) -> Result<(), &str> {
    let mut status = 0;
    for check in TOSSI_LIST.iter() {
        if check == &tossi {
            status = 1;
            break;
        }
    }
    if status == 1 {
        return Ok(());
    } else {
        return Err("This value is not correct tossi.");
    }
}

// 파라미터롤 받는 단어를 제한 기준 함수
fn limit_word_len(word: &str) -> Result<(), &str> {
    let limitation = 50;
    if word.chars().count() <= limitation {
        return Ok(());
    } else {
        return Err("The length has been exceeded. Set the word length to less than 50.");
    }
}

#[test]
fn _limit_word_len() {
    let temp = "12345";
    assert_eq!(Ok(()), limit_word_len(temp));

    let temp = "아이디는 50자까지 설정이 가능합니다.";
    assert_eq!(Ok(()), limit_word_len(temp));

    let temp = "10000000000000000000000000000000000000000000000000000";
    assert_eq!(
        Err("The length has been exceeded. Set the word length to less than 50."),
        limit_word_len(temp)
    );

    let temp = "테트리스1테트리스2테트리스3테트리스4테트리스5테트리스6테트리스7테트리스8테트리스9테트리스10";
    assert_eq!(
        Err("The length has been exceeded. Set the word length to less than 50."),
        limit_word_len(temp)
    );

    let temp = "1테트리스2테트리스3테트리스4테트리스5테트리스6테트리스7테트리스8테트리스9테트리스10테트리스";
    assert_eq!(
        Err("The length has been exceeded. Set the word length to less than 50."),
        limit_word_len(temp)
    );
}

#[test]
fn _verifier_tossi() {
    let temp = "까지";
    assert_eq!(Ok(()), verifier_tossi(temp));

    let temp = "류현지";
    assert_eq!(
        Err("This value is not correct tossi."),
        verifier_tossi(temp)
    );
}

#[test]
// is_hangeul_or_number() 함수는 다음과 같이 반환한다.
// 한글이고 숫자이다: 현재 이것은 불가능하다.
// 한글이지만 숫자는 아니다: (true, false)
// 한글은 아니고 숫자이다: (false, true)
// 한글은 아니고 숫자도 아니다: (false, false)
fn _is_hangeul_or_number_마지막_글자가_숫자인지_테스트() {
    let temp = "테트리스1테트리스2테트리스3테트리스4테트리스5테트리스6테트리스7테트리스8테트리스9테트리스10";
    assert_eq!((false, true), is_hangeul_or_number(temp.to_string()));
    let temp = "테트리스9";
    assert_eq!((false, true), is_hangeul_or_number(temp.to_string()));
    let temp = "테트리스a";
    assert_eq!((false, false), is_hangeul_or_number(temp.to_string()));
    let temp = "테트리스!";
    assert_eq!((false, false), is_hangeul_or_number(temp.to_string()));
    let temp = "tetris";
    assert_eq!((false, false), is_hangeul_or_number(temp.to_string()));
}

#[test]
fn _is_hangeul_or_number_마지막_글자가_한글인가() {
    let temp = "테트리스1테트리스2테트리스3테트리스4테트리스5테트리스6테트리스7테트리스8테트리스9테트리스10테트리스";
    assert_eq!((true, false), is_hangeul_or_number(temp.to_string()));
}
