//! # '~ 로' 또는 '~ 으로'인지 판단하는 모듈
//!
//! - '로'는 받침 없는 체언이나 ‘ㄹ’ 받침으로 끝나는 체언 뒤에 붙습니다.
//! - '으로'는 ‘ㄹ’을 제외한 받침 있는 체언 뒤에 붙습니다.
//! - 외국어가 앞 단어로 오는 경우 병기 '(으)로'가 출력됩니다.

use crate::guess_final_letter;

pub fn change(word: &str) -> String {
    let fin = guess_final_letter(word);
    if fin == 'N' {
        return "(으)로".to_string();
    }
    if fin == ' ' || fin == 'ㄹ' {
        "로".to_string()
    } else {
        "으로".to_string()
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
        assert_eq!(result, change(temp));
        // 받침이 있는 경우
        let temp = "법원";
        let result = "으로";
        assert_eq!(result, change(temp));
        // 받침에 `ㄹ`이 있는 경우
        let temp = "구글";
        let result = "로";
        assert_eq!(result, change(temp));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "(으)로";
        assert_eq!(result, change(temp));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "으로";
        assert_eq!(result, change(temp));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "으로";
        assert_eq!(result, change(temp));
    }
}
