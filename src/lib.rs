//! # 입력된 토시(tossi)가 어떤 것인지 알아내 저장하는 토시 구조체를 구현한 모듈
//!
//! 사용자가 입력한 토시를 변환해서 저장하고,
//! 변환한 값을 토대로 어떤 종류인지 분류한 다음 분류한 결과를 저장한다.
//! 사용법은 아래와 같다.
//!
//! ```rust, ignore

mod filter;
mod hangeul;
mod identifier;
mod number;
mod particle;
mod verifier;

use crate::particle::*;
use identifier::{Tossi, TossiKind};

pub const EUL: (&str, &str, &str) = ("(을)를", "를", "을");
pub const KA: (&str, &str, &str) = ("(이)가", "가", "이");
pub const IDA: (&str, &str, &str) = ("(이)다", "다", "이다");
pub const NEUN: (&str, &str, &str) = ("(은)는", "는", "은");

// hangeul 모듈
// tests/hangeul.rs 에서 test 한다.
pub fn join_phonemes(word: [char; 3]) -> char {
    hangeul::join_phonemes(word)
}

// tests/hangeul.rs 에서 test 한다.
pub fn split_phonemes(word: char) -> [char; 3] {
    hangeul::split_phonemes(word)
}

// filter 모듈
// tests/filter.rs 에서 test 한다.
pub fn find_last_letter(word: &str) -> char {
    filter::find_last_letter(word)
}

// tests/filter.rs 에 test 가 한다.
pub fn guess_final_letter(word: &str) -> char {
    filter::guess_final_letter(word)
}

// number 모듈
// tests/number.rs 에서 test 한다.
pub fn change_num_to_hangeul(word: &str) -> String {
    number::change_num_to_hangeul(word)
}

// tests/number.rs 에서 test 한다.
pub fn change_int_char(num: char) -> char {
    number::change_int_char(num)
}

/// ## 입력된 토시(tossi)가 어떤 것인지 알아내 입력된 값과 반환하는 함수
///
/// 아래와 같은 형식으로 입력된 것 중 두 번째 입력된 토시가 어떤 종류인지 파악합니다.
/// 이 프로젝트에서 구현하고자 하는
/// `postfix()`와 `pick()`를 이 함수를 이용해서 구현하고 있습니다.
///
/// ```rust
/// use tossicat::postfix;
/// postfix("집", "(으)로");
/// postfix("집", "로");
/// postfix("집", "(으)로");
/// ```

fn postfix_raw(word: &str, tossi: &str) -> (String, String) {
    //파라미터에 올바른 규격의 값이 들어왔는지 확인하기
    let temp = Tossi::new(tossi);
    let result = match temp.kind {
        TossiKind::Neun => eul_ka_ida_neun::look_up(word, NEUN).to_owned(),
        TossiKind::Ka => eul_ka_ida_neun::look_up(word, KA).to_owned(),
        TossiKind::Ro => ro::change(word),
        TossiKind::Ida => eul_ka_ida_neun::look_up(word, IDA).to_owned(),
        TossiKind::Eul => eul_ka_ida_neun::look_up(word, EUL).to_owned(),
        TossiKind::None => tossi.to_string(),
    };

    let front = word.to_string();
    (front, result)
}

/// postfix() 함수
pub fn postfix(word: &str, tossi: &str) -> String {
    let temp = postfix_raw(word, tossi);
    temp.0 + &temp.1
}

/// pick() 함수
pub fn pick(word: &str, tossi: &str) -> String {
    let temp = postfix_raw(word, tossi);
    temp.1
}

/// ## 변환하기 전에 입력된 것들이 변환가능한 것인지 검사하는 함수
/// 위에서부터 아래 조건 문을 순서대로 살펴 보겠다.
///
/// 1. 단어는 마지막 글자가 한글이나 숫자이면 된다.
/// 2. 토시는 한글이면 된다.
/// 3. 변환할 수 있는 토시인지 아닌지 파악한다.
/// 4. 단어의 길이가 50자를 넘으면 처리하지 않도록 처리한다.
///
/// 이 4가지를 만족하면 본 작업인 글자에 맞게 토시를 변환하게 된다.
pub fn verifiers<'a>(word: &'a str, tossi: &'a str) -> Result<(), &'a str> {
    verifier::verifiers(word, tossi)
}
