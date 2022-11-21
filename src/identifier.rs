//! # 입력된 토시(tossi)가 어떤 것인지 알아내 저장하는 토시 구조체를 구현한 모듈
//!
//! 사용자가 입력한 토시를 변환해서 저장하고,
//! 변환한 값을 토대로 어떤 종류인지 분류한 다음 분류한 결과를 저장한다.
//! 사용법은 아래와 같다.
//!
//! ```rust, ignore
//! let test_tossi = "으로";
//! let temp = Tossi::new(test_tossi);
//! println!("입력된 토시: {:?}", temp.modified);
//! println!("토시 종류: {:?}", temp.kind);
//! ```
//!
//! 실행 결과는 다음과 같다.
//!
//! ```ignore
//! 입력된 토시: ['으', '로']
//! 토시 종류: Ro
//! ```
//!
//! 여기서 주의할 점은 반환된 값 중 첫 번째 값은, 위에서`temp.modified`은,
//! `filter_only_significant()` 함수를 통해서
//! 변환되어 오기 때문에 이 함수의 처리 방법에 따라 변형되어 반환된다는 점이다.
//! 그리고 반환된 값은 `Vec<char>`으로 되어 있기 때문에 한 글자씩 분리되어 있다.
//!
//! 두 번째 값은 프로그램이 판단한 토시의 종류를 열거자를 가지고 표현하고 있는데
//! 이 '토시 종류(TossiKind)'의 토시 영어명 표시명은
//! 이 프로젝트 안에 있는 다음 파일을 참고한다.
//!
//! - docs/terms.md

use crate::filter::filter_only_significant;
use crate::guess_final_letter;

#[derive(PartialEq)]
pub enum TossiKind {
    Neun,
    Ka,
    Ro,
    Ida,
    Eul,
    Others,
}

pub struct Tossi {
    pub modified: Vec<char>,
    pub kind: TossiKind,
}

impl Tossi {
    pub fn new(raw: &str) -> Self {
        let temp_modified = filter_only_significant(raw);
        // 앞에서 변환 것을 토대로 글자 수에 따라 조사 종류를 찾는다.
        let temp_kind = match temp_modified.len() {
            1 => one_letter(temp_modified[0]),
            2 => two_letters(&temp_modified),
            _ => TossiKind::Others,
        };
        Self {
            modified: temp_modified,
            kind: temp_kind,
        }
    }
}

/// ## 한 글자로 된 토시를 분류하는 함수
/// 한 글자로 된 토시가 들어오면 이를 종류 별로 분류하는 함수
fn one_letter(element: char) -> TossiKind {
    match element {
        '은' | '는' => TossiKind::Neun,
        '이' | '가' => TossiKind::Ka,
        '을' | '를' => TossiKind::Eul,
        '로' => TossiKind::Ro,
        '다' => TossiKind::Ida,
        _ => TossiKind::Others,
    }
}

/// ## 두 글자로 된 토시를 분류하는 함수
/// 두 글자로 된 토시가 들어오면 이를 종류 별로 분류하는 함수
fn two_letters(elements: &[char]) -> TossiKind {
    match (elements[0], elements[1]) {
        ('으', '로') => TossiKind::Ro,
        ('이', '다') => TossiKind::Ida,
        (_, _) => TossiKind::Others,
    }
}

/// ## 종성만 찾아서 이것에 맞게 토시를 찾아 주는 함수
/// 이 함수는 특정 문자열(단어)에서 마지막 글자의 종성만을 뽑아서
/// 이를 토대로 입력된 토시 종류(`TossiKind`)에 리스트(아래 리스트 참고)에서 적합한 것을 선택한다.
/// 그런 다음 위에서 설명한 것을 토대로 그 중 하나를 반환한다.
/// 참고로 이 함수를 들어오는 것은 아래 4개 종류 밖에 없기 때문에 나머지는 공백 한 칸을 반환한다.
///
/// ```rust
/// const EUL: (&str, &str, &str) = ("(을)를", "를", "을");
/// const KA: (&str, &str, &str) = ("(이)가", "가", "이");
/// const IDA: (&str, &str, &str) = ("(이)다", "다", "이다");
/// const NEUN: (&str, &str, &str) = ("(은)는", "는", "은");
/// const Ro: (&str, &str, &str) = ("(으)로", "로", "으로");
/// ```

pub fn look_up(word: &str, kind: TossiKind) -> &str {
    let result = match kind {
        TossiKind::Neun => ("(은)는", "는", "은"),
        TossiKind::Ka => ("(이)가", "가", "이"),
        TossiKind::Ida => ("(이)다", "다", "이다"),
        TossiKind::Eul => ("(을)를", "를", "을"),
        TossiKind::Ro => ("(으)로", "로", "으로"),
        _ => (" ", " ", " "),
    };

    let final_letter = guess_final_letter(word);
    // find_last_letter()은 한글이나 숫자가 없을 경우 ' '을 출력한다.
    // println!("마지막 글자 받침: {}", filtered);

    if final_letter == 'N' {
        result.0
    }
    else if final_letter == ' ' {
        result.1
    }
    else if final_letter == 'ㄹ' && kind == TossiKind::Ro {
        result.1
    }
    else {
        result.2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn _look_up() {
        let temp = "네이버";
        let result = "를";
        assert_eq!(result, look_up(temp, TossiKind::Eul));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "(을)를";
        assert_eq!(result, look_up(temp, TossiKind::Eul));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "을";
        assert_eq!(result, look_up(temp, TossiKind::Eul));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "을";
        assert_eq!(result, look_up(temp, TossiKind::Eul));

        let temp = "네이버";
        let result = "로";
        assert_eq!(result, look_up(temp, TossiKind::Ro));
        // 받침이 있는 경우
        let temp = "법원";
        let result = "으로";
        assert_eq!(result, look_up(temp, TossiKind::Ro));
        // 받침에 `ㄹ`이 있는 경우
        let temp = "구글";
        let result = "로";
        assert_eq!(result, look_up(temp, TossiKind::Ro));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "(으)로";
        assert_eq!(result, look_up(temp, TossiKind::Ro));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "으로";
        assert_eq!(result, look_up(temp, TossiKind::Ro));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "으로";
        assert_eq!(result, look_up(temp, TossiKind::Ro));
    }
}