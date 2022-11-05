//! # 입력된 토시(tossi)가 어떤 것인지 알아내 저장하는 토시 구조체를 구현한 모듈
//!
//! 사용자가 입력한 토시를 변환해서 저장하고,
//! 변환한 값을 토대로 어떤 종류인지 분류한 다음 분류한 결과를 저장한다.
//! 사용법은 아래와 같다.
//!
//! ```rust
//! mod identifier;
//! let test_tossi = "으로";
//! let temp = Tossi::new(test_tossi);
//! println!("입력된 토시: {:?}", temp.modified);
//! println!("토시 종류: {:?}", temp.kind);
//! ```
//! ## 조사 종류를 영어로 표기하기
//!
//! 아래 로마자 표기법에 따라 변환했다.
//! 조사를 로마자로 변환했을 때 글자 수가 적은 것을 선택하여 이를 대표 이름으로 선택했다.
//!  
//! https://ko.wiktionary.org/wiki/부록:로마자_표기법/국어
//!
//! - "은", "는": Neun
//! - "이", "가": Ka
//! - "으로", "로": Ro
//! - "이다", "다": Ida
//!

use crate::filter::filter_only_significant;

#[derive(Debug)]
pub enum TossiKind {
    Neun,
    Ka,
    Ro,
    Ida,
    Eul,
    None,
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
            _ => TossiKind::None,
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
    let result = match element {
        '은' | '는' => TossiKind::Neun,
        '이' | '가' => TossiKind::Ka,
        '을' | '를' => TossiKind::Eul,
        '로' => TossiKind::Ro,
        '다' => TossiKind::Ida,
        _ => TossiKind::None,
    };
    result
}

/// ## 두 글자로 된 토시를 분류하는 함수
/// 두 글자로 된 토시가 들어오면 이를 종류 별로 분류하는 함수
fn two_letters(elements: &Vec<char>) -> TossiKind {
    let result = match (elements[0], elements[1]) {
        ('으', '로') => TossiKind::Ro,
        ('이', '다') => TossiKind::Ida,
        (_, _) => TossiKind::None,
    };
    result
}
