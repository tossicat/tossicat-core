//! # TossiCat Core
//!
//! ## 토시를 같이 입력된 단어에 맞춰 변환해 주는 모듈
//!
//! 이 모듈은 임의의 단어와 그 단어에 붙일 조사(즉 토시)를 입력하면,
//! 입력한 조사를 같이 입력한 단어에 자연스러운 형태로 바꿔서
//! 적절하게 변환하는 라이브러리입니다.
//!
//! 변환할 토시가 여러개 들어 있는 문장을 아래와 같은 형식으로 입력하면,
//!
//! `"{철수, 은} {영희,   과} {밥,  를} 먹습니다."`
//!
//! 다음과 같이 변경해 줍니다.
//!
//!  `"철수는 영희처럼 밥을 먹습니다."`
//!
//! 이것은 `modify_sentence()` 함수가 맡습니다. 아래와 같이 사용하시면 됩니다.
//!
//! ```
//! use tossicat::modify_sentence;
//!
//! let test = "{철수, 은} {영희, 과} {밥, 를} 먹습니다.";
//! let result = Ok("철수는 영희와 밥을 먹습니다.".to_string());
//! assert_eq!(result, modify_sentence(test));
//! ```
//!
//! 앞에서처럼 문장을 다루는 것이 아니라 단순하게 특정 단어에 특정 토시를 붙일 때
//! 어떻게 변환해야 하는 것인지를 알고 싶은 경우도 있습니다. 이런 경우에 사용할 함수는
//! `postfix()`입니다.
//!
//! ```
//! use tossicat::postfix;
//!
//! let result = Ok("사과를".to_string());
//! assert_eq!(result, postfix("사과", "을"));
//! ```
//!
//! 현재 단어의 마지막 글자가 한글과 숫자라면 아무런 문제 없이 같이 입력된 토시를 적절하게
//! 변경해줄 수 있습니다. 그런데 입력된 단어 전체가 영어와 같은 외국어 단어로 되어 있거나
//! 마지막 글자가 외국어라면, 에러를 발생하지 않고
//! 아래와 같이 토시를 병기해 처리합니다. 따라서 외국어를 사용하는데도 문제가 발생하지
//! 않습니다.
//!
//! ```
//! use tossicat::postfix;
//!
//! let test = postfix("apple", "을");
//! let result = Ok("apple(을)를".to_string());
//! assert_eq!(test, result);
//!
//! use tossicat::modify_sentence;
//!
//! let test = "{철수, 은} {apple, 를} 먹습니다.";
//! let result = Ok("철수는 apple(을)를 먹습니다.".to_string());
//! assert_eq!(result, modify_sentence(test));
//! ```
//!
//! 이 두 개의 함수가 이 라이브러리의 가장 중요한 기능입니다.
//! 이 라이브러리에서 구현하고 있는 중요한 함수는 다음과 같습니다.
//!
//! - `modify_sentence()`: 입력된 문장에 포함된 1개 이상의 토시를 같이 입력된 단어에 맞게 전환해
//! 입력된 문장을 바꿔 반환하는 함수
//! - `postfix()`: 입력된 토시를 같이 입력된 단어에 맞게 변환해, 입력된 단어와 합쳐 반환하는 함수
//!
//! 위의 기능을 구현하기 위해서 작성한 몇개의 함수도 같이 공개합니다.
//! 자세한 내용은 각 함수 설명을 참고하세요.

pub mod bracket;
mod error;
mod filter;
mod hangeul;
mod identifier;
mod number;
mod tossi;
mod transfer;
mod verifier;

use error::{ParseError, ValueError};
use identifier::{Tossi, TossiKind};

// bracket 모듈에 있습니다.
// tests/bracket.rs 에서 test 합니다.

/// ## 변경할 토시가 여러 개 들어 있는 문장을 적절한 토시로 변경해 문장을 반환하는 함수  
///
/// 아래 보기 같이 변결할 단어와 토시가 중괄호로 여러 개를 감싸고 있는 문장에서
/// 토시를 같이 입력된 단어에 맞게 적절한 토시로 일괄적으로 변경해서 아래 변환 결과처럼
/// 중괄호, `{,}`를 제거해 완전한 문장으로 바꿔 변환해 줍니다.
///
/// 보기: `"{철수, 은} {영희, 처럼} {밥,  를} 먹습니다.";`
///
/// 변환 결과: `"철수는 영희처럼 밥을 먹습니다.";`
///
/// 구체적인 사용 방법은 다음과 같습니다.
///
/// ```rust
/// use tossicat::modify_sentence;
///
/// let test = "{철수, 은} {영희, 처럼} {밥,  를} 먹습니다.";
/// let result = Ok("철수는 영희처럼 밥을 먹습니다.".to_string());
/// assert_eq!(result, modify_sentence(test));
/// ```

pub fn modify_sentence(string: &str) -> Result<String, ParseError> {
    // let mut original_copy = string;
    let mut sentence = String::from(string);
    // let temp = bracket::modify_pairs(string);
    let temp = match bracket::modify_pairs(string) {
        Ok(temp) => temp,
        Err(e) => return Err(ParseError::new(e)),
    };
    let mut temp_tossi_num: Vec<bool> = vec![];
    for item in temp {
        let result = postfix(&item.1, &item.2);
        temp_tossi_num.push(true);
        let original = "{".to_string() + &item.0 + "}";
        match result {
            Ok(n) => sentence = sentence.replace(&original, &n),
            Err(e) => {
                return Err(ParseError::new(error::ParseErrorType::InvalidValue(
                    e.error,
                )))
            }
        }
    }
    Ok(sentence)
}

// hangeul 모듈에 있습니다.
// tests/hangeul.rs 에서 test 합니다.

/// ## 초, 중, 종성을 한글 한 글자로 합쳐주는 함수
///
/// 이 함수는 기본적으로 입력된 것이 종성까지 가지고 있는다고 가정하고 작성하였습니다.
/// 사용하기 위해서는 종성이 없는 경우에도 다음과 같이 종성 자리에 ` `를 넣어야 합니다.
/// 만약 종성이 없이 글자를 만들려고 한다면  `['ㅈ','ㅏ',' ']`처럼
/// 공백으로 종성을 넣어야 합니다. 만악 `['ㅈ','ㅏ','']` 처럼
/// 한다면 에러가 발생합니다.
///
/// ```rust
/// use tossicat::join_phonemes;
/// assert_eq!('글', join_phonemes(['ㄱ','ㅡ','ㄹ']));
/// assert_eq!('자', join_phonemes(['ㅈ','ㅏ',' ']));
/// ```

pub fn join_phonemes(word: [char; 3]) -> char {
    hangeul::join_phonemes(word)
}

// hangeul 모듈에 있습니다.
// tests/hangeul.rs 에서 test 합니다.

/// ## 입력된 한 글자에서 그 글자의 종성을 바꿔주는 함수
/// 이 함수는 입력된 한글 한 글자에서 입력된 값으로 종성을 바꿔 반환한다.
/// 이때 입력된 한 글자가 한글이 아닌 경우와
/// 바꾸기 위해 입력한 자모가 한글 종성 자모에 쓰일 수 없는 것이면
/// 입력된 글자 그대로를 반환한다.
/// ```rust
///    let temp = '정';
///    assert_eq!('점', tossicat::modify_finall_jamo(temp, 'ㅁ'));
///    let temp = '감';
///    assert_eq!('강', tossicat::modify_finall_jamo(temp, 'ㅇ'));
/// ```

pub fn modify_finall_jamo(letter: char, jamo: char) -> char {
    hangeul::modify_finall_jamo(letter, jamo)
}

// hangeul 모듈에 있습니다.
// tests/hangeul.rs 에서 test 합니다.

/// ## 한글 한 글자를 초, 중, 종성으로 분리해 주는 함수
///
/// 이 함수는 기본적으로 입력된 것이 종성이 없는 경우에도
/// 종성을 스페이스, 즉 `' '`으로 반환합니다.
/// 예를 들어 만약 종성이 없는 경우에는 ' '으로 치환됩니다.
/// 아래 2번째 예를 참고하세요.
///
/// ```rust
/// use tossicat::split_phonemes;
/// assert_eq!(['ㄱ','ㅡ','ㄹ'], split_phonemes('글'));
/// assert_eq!(['ㅈ','ㅏ',' '], split_phonemes('자'));
/// ```

pub fn split_phonemes(word: char) -> [char; 3] {
    hangeul::split_phonemes(word)
}

// filter 모듈
// tests/filter.rs 에서 test 합니다.

/// ## 입력된 문자열에서 마지막 글자를 찾아 주는함수
///
/// 만약 입력된 문자가 한글이 아니면 `N`을 반환합니다.
/// 아래 2번째 예를 참고하세요.
/// 숫자는 한글로 변환해서 마지막 글자를 뽑아냅니다.
/// 아래 3번째 예를 참고하세요.
/// 마지막 문자만을 뽑아내는 것이기 때문에 마지막 글자만 한글이나 숫자이면 됩니다.
///
/// ```rust
/// use tossicat::find_last_letter;
/// assert_eq!('자', find_last_letter("글자"));
/// assert_eq!('N', find_last_letter("apple"));
/// assert_eq!('삼', find_last_letter("123"));
/// ```

pub fn find_last_letter(word: &str) -> char {
    filter::find_last_letter(word)
}

// filter 모듈
// tests/filter.rs 에 test 합니다.

/// ## 종성만 찾아서 도출해주는 함수
///
/// 이 함수는 특정 문자열에서 마지막 글자의 종성만 도출합니다.
/// 만약 마지막 글자가 종성이 없는 경우에는 스페이스, 즉 `' '`을 반환합니다.
/// 예를 들어 만약 종성이 없는 경우에는 ' '으로 치환됩니다.
/// 아래 2번째 예를 참고하세요.
/// 만약 입력된 문자가 한글이 아니면 `N`을 반환합니다.
/// 아래 4번째 예를 참고하세요.
///
/// ```rust
/// use tossicat::guess_final_letter;
/// assert_eq!('ㄹ', guess_final_letter("글"));
/// assert_eq!(' ', guess_final_letter("글자"));
/// assert_eq!('ㅇ', guess_final_letter("몸빵"));
/// assert_eq!('N', guess_final_letter("apple"));
/// ```

pub fn guess_final_letter(word: &str) -> char {
    filter::guess_final_letter(word)
}

// number 모듈
// tests/number.rs 에서 test 한다.

/// ## 숫자를 한글 표기법으로 바꿔주는 함수
///
/// 입력된 숫자를 단위를 포함해서 한글로 읽는 한글 표기법으로 바꿔줍니다.
///
/// ```rust
/// use tossicat::change_num_to_hangeul;
/// assert_eq!("천사", change_num_to_hangeul("1004"));
/// assert_eq!("만이천삼백사십오", change_num_to_hangeul("12345"));
/// assert_eq!("십억", change_num_to_hangeul("1000000000"));
/// ```

pub fn change_num_to_hangeul(word: &str) -> String {
    number::change_num_to_hangeul(word)
}

/// ## 입력된 토시를 같이 입력된 단어에 맞게 변환해, 입력된 단어와 합쳐 반환하는 함수
///
/// 아래와 같은 형식으로 입력된 것 중 두 번째로 입력된 토시를
/// 첫 번째로 입력된 단어에 맞쳐 적절하게 변형해 입력된 단어와 합쳐서
/// 반환하는 함수
///
/// ```rust
/// use tossicat::postfix;
/// postfix("집", "으로");
/// postfix("집", "로");
/// postfix("집", "(으)로");
/// ```
pub fn postfix(word: &str, tossi: &str) -> Result<String, ValueError> {
    match verifier::verify_value(word, tossi) {
        Err(e) => Err(ValueError::new(e)),
        Ok(()) => {
            let temp = Tossi::new(tossi);
            match temp.kind {
                TossiKind::Others => Ok(word.to_string() + tossi),
                // 아래에서 더하는 것이랑 안 더하는 것 차이는 만약 입력된 단어가 변하는
                // 경우에는 단어 자체로 변해서 반환하기 때문에 또 단어를 더하면 중복이
                // 발생하게 됩니다. 그래서 더하면 안 됩니다.
                TossiKind::Ka => Ok(transfer::tossi(word, temp)),
                TossiKind::Indeul => Ok(transfer::tossi(word, temp)),
                TossiKind::Injeuk => Ok(transfer::tossi(word, temp)),
                TossiKind::Illang => Ok(transfer::tossi(word, temp)),
                _ => Ok(word.to_string() + &transfer::tossi(word, temp)),
            }
        }
    }
}
