//! ## 입력된 단어에 맞게 입력된 토시를 바꿔 변환하는 모듈
//! `tossi()` 함수가 메인 함수입니다.
//! 이 함수에 `Tossi` 구조체와 단어가 입력하면 그 구조체에 맞는 토시 변형 후보를
//! 선택하고, 그 구조체가 제시하고 있는 변형 방법에 맞는 변형 함수를 선택하게 됩니다.
//! 나머지 함수들을 현형 함수입니다.

const EUL: (&str, &str, &str) = ("(을)를", "를", "을");
const KA: (&str, &str, &str) = ("(이)가", "가", "이");
const IDA: (&str, &str, &str) = ("(이)다", "다", "이다");
const NEUN: (&str, &str, &str) = ("(은)는", "는", "은");
const RO: (&str, &str, &str) = ("(으)로", "로", "으로");
const ROSEO: (&str, &str, &str) = ("(으)로서", "로서", "으로서");
const ROSSEO: (&str, &str, &str) = ("(으)로써", "로써", "으로써");
const ROBUTEO: (&str, &str, &str) = ("(으)로부터", "로부터", "으로부터");

use crate::guess_final_letter;

use crate::identifier::{Tossi, TossiKind, TransTossiWhen};

/// `Tossi` 구조체와 단어가 입력하면 `Tossi` 구조체의 `kind`을 가지고
/// 토시 변형 후보를 선택하고, `Tossi` 구조체의 `when`을 가지고 있는
/// 변형 방법에 맞는 변형 함수를 선택합니다. 선택한 함수에 토시를 붙일 단어와
/// 토시 변형 후보들을 입력히면 적합한 토시를 반환합니다.
pub fn tossi(word: &str, tossi: Tossi) -> &str {
    let tossi_variants = match tossi.kind {
        TossiKind::Eul => EUL,
        TossiKind::Ka => KA,
        TossiKind::Ida => IDA,
        TossiKind::Neun => NEUN,
        TossiKind::Ro => RO,
        TossiKind::Roseo => ROSEO,
        TossiKind::Rosseo => ROSSEO,
        TossiKind::Robuteo => ROBUTEO,
        TossiKind::Others => (" ", " ", " "),
    };

    let result = match tossi.when {
        TransTossiWhen::Blank => when_blank(word, tossi_variants),
        TransTossiWhen::RiEulAndBlank => when_rieul_and_blank(word, tossi_variants),
        TransTossiWhen::Nothing => " ",
    };
    result
}
fn when_rieul_and_blank<'a>(word: &'a str, tossi_variants: (&'a str, &'a str, &'a str)) -> &'a str {
    let filtered = guess_final_letter(word);
    // find_last_letter()은 한글이나 숫자가 없을 경우 ' '을 출력한다.
    // println!("마지막 글자 받침: {}", filtered);
    if filtered == 'N' {
        tossi_variants.0
    } else if filtered == ' ' || filtered == 'ㄹ' {
        tossi_variants.1
    } else {
        tossi_variants.2
    }
}
fn when_blank<'a>(word: &'a str, tossi_variants: (&'a str, &'a str, &'a str)) -> &'a str {
    let filtered = guess_final_letter(word);
    // find_last_letter()은 한글이나 숫자가 없을 경우 ' '을 출력한다.
    // println!("마지막 글자 받침: {}", filtered);
    if filtered == 'N' {
        tossi_variants.0
    } else if filtered == ' ' {
        tossi_variants.1
    } else {
        tossi_variants.2
    }
}
