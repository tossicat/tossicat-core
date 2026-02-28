//! # 입력된 토시(tossi)가 어떤 것인지 알아내 저장하는 토시 구조체를 구현한 모듈
//!
//! 사용자가 입력한 토시를 변환해서 저장하고,
//! 변환한 값을 토대로 어떤 종류인지 분류한 다음 분류한 결과를 저장한다.
//! 사용법은 아래와 같다.
//!
//! ```rust, ignore
//! let test_tossi = "으로";
//! let temp = Tossi::new(test_tossi);
//! println!("토시 종류: {:?}", temp.kind);
//! ```
//!
//! 실행 결과는 다음과 같다.
//!
//! ```ignore
//! 토시 종류: Ro
//! ```
//!
//! '토시 종류(TossiKind)'의 토시 영어명 표시명은
//! 이 프로젝트 안에 있는 다음 파일을 참고한다.
//!
//! - docs/terms.md

use crate::filter::filter_only_significant;

pub enum TossiKind {
    Deun,
    Deunji,
    Deunka,
    Eul,
    Ida,
    Indeul,
    Injeuk,
    Ka,
    Ko,
    Myeo,
    Na,
    Nama,
    Neun,
    Ni,
    Rado,
    Rago,
    Ran,
    Rang,
    Raya,
    Ro,
    Robuteo,
    Roseo,
    Rosseo,
    Wa,
    Yamalro,
    Ya,
    Yeo,
    Illang,
    Others,
}
/// ## 토시 변환 방식을 분류하는 열거형
///
/// 한국어 조사(토시)는 앞 단어의 받침(종성) 유무에 따라 형태가 달라진다.
/// 이 열거형은 그 변환 방식을 다음과 같이 분류한다.
///
/// - `Blank`: 받침 유무에 따라 형태가 달라지는 일반적인 경우 (예: 은/는, 을/를, 과/와)
/// - `RiEulAndBlank`: 받침이 'ㄹ'일 때 특수 처리하는 경우 (예: "서울로", "학교로")
/// - `OnlyKa`: '가/이' 조사의 특수 변환
/// - `LastJamoNieun`: 받침 없는 체언에 'ㄴ' 받침을 추가하는 경우 (예: "철수" → "철순들")
/// - `LastJamoRieul`: 받침 없는 체언에 'ㄹ' 받침을 추가하는 경우 (예: "가구" → "가굴랑")
/// - `Nothing`: 변환하지 않는 경우
pub enum TransTossiWhen {
    Blank,
    RiEulAndBlank,
    OnlyKa,
    LastJamoNieun,
    LastJamoRieul,
    Nothing,
}

pub struct Tossi {
    pub kind: TossiKind,
    pub when: TransTossiWhen,
}

impl Tossi {
    pub fn new(raw: &str) -> Self {
        let temp_modified = filter_only_significant(raw);
        // 앞에서 변환 것을 토대로 글자 수에 따라 조사 종류를 찾는다.
        let temp_kind = match temp_modified.len() {
            1 => one_letter(temp_modified[0]),
            2 => two_letters(&temp_modified),
            3 => three_letters(&temp_modified),
            4 => four_letters(&temp_modified),
            _ => TossiKind::Others,
        };
        // 토시 종류에 따라 변환 방식을 결정한다.
        // 대부분의 조사는 받침 유무에 따라 형태가 달라지는 Blank 방식이고,
        // 아래 나열된 것들만 특수한 변환 방식을 사용한다.
        let temp_trans_tossi_when = match temp_kind {
            // 받침 없는 체언에 'ㄹ' 받침을 추가하는 토시 (예: "가구" → "가굴랑")
            TossiKind::Illang => TransTossiWhen::LastJamoRieul,
            // 받침 없는 체언에 'ㄴ' 받침을 추가하는 토시 (예: "철수" → "철순들")
            TossiKind::Indeul | TossiKind::Injeuk => TransTossiWhen::LastJamoNieun,
            // '가/이' 조사의 특수 변환
            TossiKind::Ka => TransTossiWhen::OnlyKa,
            // 받침이 'ㄹ'일 때 '으'가 붙지 않는 토시 (예: "서울로", "학교로")
            TossiKind::Ro | TossiKind::Robuteo | TossiKind::Roseo | TossiKind::Rosseo => {
                TransTossiWhen::RiEulAndBlank
            }
            // 처리할 수 없는 토시
            TossiKind::Others => TransTossiWhen::Nothing,
            // 그 외 받침 유무에 따라 형태가 달라지는 일반적인 조사
            _ => TransTossiWhen::Blank,
        };
        Self {
            kind: temp_kind,
            when: temp_trans_tossi_when,
        }
    }
}

/// ## 한 글자로 된 토시를 분류하는 함수
/// 한 글자로 된 토시가 들어오면 이를 종류 별로 분류하는 함수
/// 추가한 순서대로 아래에 추가하고 있습니다.
/// 정렬할 필요 없습니다.
fn one_letter(element: char) -> TossiKind {
    match element {
        '은' | '는' => TossiKind::Neun,
        '이' | '가' => TossiKind::Ka,
        '을' | '를' => TossiKind::Eul,
        '와' | '과' => TossiKind::Wa,
        '로' => TossiKind::Ro,
        '다' => TossiKind::Ida,
        '나' => TossiKind::Na,
        '랑' => TossiKind::Rang,
        '란' => TossiKind::Ran,
        '며' => TossiKind::Myeo,
        '고' => TossiKind::Ko,
        '니' => TossiKind::Ni,
        '든' => TossiKind::Deun,
        '여' => TossiKind::Yeo,
        '아' | '야' => TossiKind::Ya,
        _ => TossiKind::Others,
    }
}

/// ## 두 글자로 된 토시를 분류하는 함수
/// 두 글자로 된 토시가 들어오면 이를 종류 별로 분류하는 함수
/// 추가한 순서대로 아래에 추가하고 있습니다.
/// 정렬할 필요 없습니다.
fn two_letters(elements: &[char]) -> TossiKind {
    match (elements[0], elements[1]) {
        ('으', '로') => TossiKind::Ro,
        ('로', '서') => TossiKind::Roseo,
        ('로', '써') => TossiKind::Rosseo,
        ('이', '다') => TossiKind::Ida,
        ('이', '나') => TossiKind::Na,
        ('이', '랑') => TossiKind::Rang,
        ('이', '란') => TossiKind::Ran,
        ('나', '마') => TossiKind::Nama,
        ('이', '며') => TossiKind::Myeo,
        ('이', '고') => TossiKind::Ko,
        ('이', '니') => TossiKind::Ni,
        ('이', '든') => TossiKind::Deun,
        ('든', '지') => TossiKind::Deunji,
        ('든', '가') => TossiKind::Deunka,
        ('이', '여') => TossiKind::Yeo,
        ('라', '야') => TossiKind::Raya,
        ('라', '도') => TossiKind::Rado,
        ('인', '들') => TossiKind::Indeul,
        ('인', '즉') => TossiKind::Injeuk,
        ('라', '고') => TossiKind::Rago,
        ('일', '랑') => TossiKind::Illang,
        (_, _) => TossiKind::Others,
    }
}

/// ## 세 글자로 된 토시를 분류하는 함수
/// 세 글자로 된 토시가 들어오면 이를 종류 별로 분류하는 함수
/// 추가한 순서대로 아래에 추가하고 있습니다.
/// 정렬할 필요 없습니다.
fn three_letters(elements: &[char]) -> TossiKind {
    match (elements[0], elements[1], elements[2]) {
        ('으', '로', '서') => TossiKind::Roseo,
        ('으', '로', '써') => TossiKind::Rosseo,
        ('로', '부', '터') => TossiKind::Robuteo,
        ('이', '나', '마') => TossiKind::Nama,
        ('야', '말', '로') => TossiKind::Yamalro,
        ('이', '든', '지') => TossiKind::Deunji,
        ('이', '든', '가') => TossiKind::Deunka,
        ('이', '라', '야') => TossiKind::Raya,
        ('이', '라', '도') => TossiKind::Rado,
        ('이', '라', '고') => TossiKind::Rago,
        (_, _, _) => TossiKind::Others,
    }
}

/// ## 네 글자로 된 토시를 분류하는 함수
/// 네 글자로 된 토시가 들어오면 이를 종류 별로 분류하는 함수
/// 추가한 순서대로 아래에 추가하고 있습니다.
/// 정렬할 필요 없습니다.
fn four_letters(elements: &[char]) -> TossiKind {
    match (elements[0], elements[1], elements[2], elements[3]) {
        ('으', '로', '부', '터') => TossiKind::Robuteo,
        ('이', '야', '말', '로') => TossiKind::Yamalro,
        (_, _, _, _) => TossiKind::Others,
    }
}
