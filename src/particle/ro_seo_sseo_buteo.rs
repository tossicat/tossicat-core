//! # RO(로), ROSEO(로서), ROSSEO(로써), ROBUTEO(로부터) 토시를 변환해 반환하는 모듈
//!
//! 이 모듈은 제목에서 알 수 있듯이
//! RO(로), ROSEO(로서), ROSSEO(로써), ROBUTEO(로부터) 토시에서
//! 입력된 문자열의 종성만을 찾아서 이 종성에 맞는
//! 앞의 토시 변화형 중 적합한 것을 찾아서 반환해주는 역할을 한다.
//! 각 토시의 상세한 변환해주는 것은 아래 내용을 참고한다.
//!
//! ## RO(로) 경우
//! - '로'는 받침 없는 체언이나 ‘ㄹ’ 받침으로 끝나는 체언 뒤에 붙습니다.
//! - '으로'는 ‘ㄹ’을 제외한 받침 있는 체언 뒤에 붙습니다.
//! - 외국어가 앞 단어로 오는 경우 병기 '(으)로'가 출력됩니다.
//!
//! ## ROSEO(로서) 경우
//! - '로서'는 받침 없는 체언이나 ‘ㄹ’ 받침으로 끝나는 체언 뒤에 붙습니다.
//! - '으로서'는 ‘ㄹ’을 제외한 받침 있는 체언 뒤에 붙습니다.
//! - 외국어가 앞 단어로 오는 경우 병기 '(으)로서'가 출력됩니다.
//!
//! ## ROSSEO(로써) 경우
//! - '로써'는 받침 없는 체언이나 ‘ㄹ’ 받침으로 끝나는 체언 뒤에 붙습니다.
//! - '으로써'는 ‘ㄹ’을 제외한 받침 있는 체언 뒤에 붙습니다.
//! - 외국어가 앞 단어로 오는 경우 병기 '(으)로써'가 출력됩니다.
//!
//! ## ROBUTEO(로부터) 경우
//! - '로부터'는 받침 없는 체언이나 ‘ㄹ’ 받침으로 끝나는 체언 뒤에 붙습니다.
//! - '으로부터'는 ‘ㄹ’을 제외한 받침 있는 체언 뒤에 붙습니다.
//! - 외국어가 앞 단어로 오는 경우 병기 '(으)로부터'가 출력됩니다.

use crate::{guess_final_letter, identifier::TossiKind};

/// ## 종성만 찾아서 이것에 맞게 토시를 찾아 주는 함수
/// 이 함수는 특정 문자열(단어)에서 마지막 글자의 종성만을 뽑아서
/// 이를 토대로 입력된 토시 종류(`TossiKind`)에 리스트(아래 리스트 참고)에서 적합한 것을 선택한다.
/// 그런 다음 위에서 설명한 것을 토대로 그 중 하나를 반환한다.
/// 참고로 이 함수를 들어오는 것은 아래 4개 종류 밖에 없기 때문에 나머지는 공백 한 칸을 반환한다.
///
/// ```rust
/// const RO: (&str, &str, &str) = ("(으)로", "로", "으로");
/// const ROSEO: (&str, &str, &str) = ("(으)로서", "로서", "으로서");
/// const ROSSEO: (&str, &str, &str) = ("(으)로써", "로써", "으로써");
/// const ROBUTEO: (&str, &str, &str) = ("(으)로부터", "로부터", "으로부터");
/// ```

pub fn look_up(word: &str, kind: TossiKind) -> &str {
    let result = match kind {
        TossiKind::Ro => ("(으)로", "로", "으로"),
        TossiKind::Roseo => ("(으)로서", "로서", "으로서"),
        TossiKind::Rosseo => ("(으)로써", "로써", "으로써"),
        TossiKind::Robuteo => ("(으)로부터", "로부터", "으로부터"),
        _ => (" ", " ", " "),
    };

    let filtered = guess_final_letter(word);
    // find_last_letter()은 한글이나 숫자가 없을 경우 ' '을 출력한다.
    // println!("마지막 글자 받침: {}", filtered);
    if filtered == 'N' {
        result.0
    } else if filtered == ' ' || filtered == 'ㄹ' {
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
    fn _ro_change() {
        // 밭침이 없는 경우
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
