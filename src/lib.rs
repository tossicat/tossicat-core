//! # TossiCat Core
//!
//! ## 토시를 같이 입력된 단어에 맞춰 변환해 주는 모듈
//!
//! 이 모듈은 임의의 단어와 그 단어에 붙일 조사(즉 토시)를 입력하면,
//! 입력한 조사를 같이 입력한 단어에 자연스러운 형태로 바꿔서
//! 적절하게 변환하는 라이브러리입니다.
//!
//! 이 라이브러리에서 구현하고 있는 함수는 다음과 같습니다.
//!
//! - `postfix()`: 입력된 토시를 같이 입력된 단어에 맞게 변환해, 입력된 단어와 합쳐 반환하는 함수
//! - `pick()` : 입력된 토시를 같이 입력된 단어에 맞게 변환해, 변환된 토시만 반환하는 함수
//!
//! 위의 기능을 구현하기 위해서 작성한 몇개의 함수도 같이 공개합니다.
//! 자세한 내용은 각 함수 설명을 참고하세요.

pub mod bracket;
mod error;
mod filter;
mod hangeul;
mod identifier;
mod number;
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
/// 보기: `"{철수, 은} {영희,   과} {밥,  를} 먹습니다.";`
/// 
/// 변환 결과: `"철수는 영희와 밥을 먹습니다.";`
/// 
/// 구체적인 사용 방법은 다음과 같습니다.
/// 
/// ```rust
/// use tossicat::modify_sentence;
///     
/// let test = "{철수, 은} {영희,   과} {밥,  를} 먹습니다.";
/// let result = "철수는 영희와 밥을 먹습니다.";
/// assert_eq!(result, modify_sentence(test).1);
/// ```


pub fn modify_sentence(string: &str) -> (bool, String) {
    // let mut original_copy = string;
    let mut sentence = String::from(string);
    // let temp = bracket::modify_pairs(string);
    let temp = match bracket::modify_pairs(string) {
        Ok(temp) => temp,
        Err(e) => panic!("{:?} 오류 발생!", e),
    };
    let mut temp_tossi_num: Vec<bool> = vec![];
    for item in temp {
        let temp = postfix(&item.1, &item.2);
        // 만약 분석할 결과가 모두 빈 것이라면 빈
        if &temp.replace(' ', "").len().to_string() == "0" {
            temp_tossi_num.push(false);
            return (false, sentence);
        } else {
            temp_tossi_num.push(true);
            let original = "{".to_string() + &item.0 + "}";
            sentence = sentence.replace(&original, &temp);
        }
    }
    (true, sentence)
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

/// ## 입력된 토시를 같이 입력된 단어에 맞게 변환해, 입력된 단어와 변환한 토시를 같이 반환하는 실질적인 함수
///
/// 아래 `postfix()`, `pick()`, 두 함수 안에서 실질적인 역할을 하는 함수입니다.
/// 아래와 같은 형식으로 입력된 것 중 `tossi` 인수로 입력된 것을 어떤 토시인지 파악해서
/// 그 종류를 열거자로 변환한 다음, 토시를 붙이고자 하는 `word` 인수로 받은 단어와 함께
/// 해당 토시를 변환하는 함수로 보내서, 그 단어에 적절한 적절한 토시를 받아서
/// 해당 단어와 변환된 토시를 각각 반환합니다.
fn postfix_raw(word: &str, tossi: &str) -> (String, String) {
    //파라미터에 올바른 규격의 값이 들어왔는지 확인하기
    let temp = Tossi::new(tossi);
    let result = match temp.kind {
        TossiKind::Others => tossi.to_string(),
        _ => transfer::tossi(word, temp).to_string(),
    };

    let front = word.to_string();
    (front, result)
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
pub fn postfix(word: &str, tossi: &str) -> String {
    let temp = postfix_raw(word, tossi);
    temp.0 + &temp.1
}

/// ## 입력된 토시를 같이 입력된 단어에 맞게 변환해, 변환된 토시만 반환하는 함수
///
/// `postfix()`와는 다르게 이 함수는 변환된 토시만 반환합니다.
///
/// ```rust
/// use tossicat::pick;
/// pick("집", "으로");
/// pick("집", "로");
/// pick("집", "(으)로");
/// ```

pub fn pick(word: &str, tossi: &str) -> String {
    let temp = postfix_raw(word, tossi);
    temp.1
}

/// ## 변환하기 전에 입력된 값들이 변환가능한 것인지 검사하는 함수
/// 입력된 토씨가 올바른지, 단어의 길이가 50자가 넘는지 확인합니다.
///
/// 1. 단어는 마지막 글자가 한글이나 숫자이면 된다.
/// 2. 토시는 한글이면 된다.
/// 3. 변환할 수 있는 토시인지 아닌지 파악한다.
/// 4. 단어의 길이가 50자를 넘으면 처리하지 않도록 처리한다.
///
/// 이 4가지를 만족하면 Success라고 반환하며, 문제가 있다면 에러 원인을 반환합니다.
pub fn value_verifier<'a>(word: &'a str, tossi: &'a str) -> String {
    match verifier::verify_value(word, tossi) {
        Err(error::ValueErrorType::InvalidTossi) => {
            format!("{}", ValueError::new(error::ValueErrorType::InvalidTossi))
        }
        Err(error::ValueErrorType::LimitLength) => {
            format!("{}", ValueError::new(error::ValueErrorType::LimitLength))
        }
        Ok(()) => "Success".to_string(),
    }
}
