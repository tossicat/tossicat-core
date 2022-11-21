//! # EUL(을), KA(가), IDA(이다), NEUN(는) 토시를 변환해 반환하는 모듈
//!
//! 이 모듈은 제목에서 알 수 있듯이
//! EUL(을), KA(가), IDA(이다), NEUN(는) 토시에서
//! 입력된 문자열의 종성만을 찾아서 이 종성에 맞는
//! 앞의 토시 변화형 중 적합한 것을 찾아서 반환해주는 역할을 한다.
//! 각 토시의 상세한 변환해주는 것은 아래 내용을 참고한다.
//!
//! ## EUL(을) 경우
//! - '를'는 받침 없는 체언 뒤에 붙습니다.
//! - '을'는 받침 있는 체언 뒤에 붙습니다.
//! - 외국어가 앞 단어로 오는 경우 병기 '(을)를'이 출력됩니다.
//!
//! ## KA(가) 경우
//! - '가'는 받침 없는 체언 뒤에 붙습니다.
//! - '이'는 받침 있는 체언 뒤에 붙습니다.
//! - 외국어가 앞 단어로 오는 경우 병기 '(이)가'이 출력됩니다.
//!
//! ## IDA(이다) 경우
//! - '다'는 받침 없는 체언 뒤에 붙습니다.
//! - '이다'는 받침 있는 체언 뒤에 붙습니다.
//! - 외국어가 앞 단어로 오는 경우 병기 '(이)다'이 출력됩니다.
//!
//! ## NEUN(는) 경우
//! - '는'는 받침 없는 체언 뒤에 붙습니다.
//! - '은'는 받침 있는 체언 뒤에 붙습니다.
//! - 외국어가 앞 단어로 오는 경우 병기 '(은)는'이 출력됩니다.

use crate::{guess_final_letter, identifier::TossiKind};

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
/// ```

pub fn look_up(word: &str, kind: TossiKind) -> &str {
    let result = match kind {
        TossiKind::Neun => ("(은)는", "는", "은"),
        TossiKind::Ka => ("(이)가", "가", "이"),
        TossiKind::Ida => ("(이)다", "다", "이다"),
        TossiKind::Eul => ("(을)를", "를", "을"),
        _ => (" ", " ", " "),
    };

    let filtered = guess_final_letter(word);
    // find_last_letter()은 한글이나 숫자가 없을 경우 ' '을 출력한다.
    // println!("마지막 글자 받침: {}", filtered);
    if filtered == 'N' {
        result.0
    } else if filtered == ' ' {
        result.1
    } else {
        result.2
    }
}

/// 비공개 함수 테스트
/// 위 함수가 이 내부적으로 `pub`로 설정해 사용하지만,
/// `lib.rs`에 올려서 크레이트로 배포할 때 공개로 만들지 않고
/// 사용하기 위하여 여기서 아래와 같이 비공개 함수 테스트 형식을 빌어서 테스트를 하겠습니다.
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn _look_up_in_eul_ka_ida_neun() {
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
    }
}
