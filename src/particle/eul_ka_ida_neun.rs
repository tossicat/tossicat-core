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

use crate::guess_final_letter;

/// ## 종성만 찾아서 이것에 맞게 토시를 찾아 주는 함수
/// 이 함수는 특정 문자열(단어)에서 마지막 글자의 종성만을 뽑아서
/// 이를 토대로 `list`로 들어온 토시들 중 적합한 것을 선택하여 반환하는 함수
pub fn look_up<'a>(word: &'a str, list: (&'a str, &'a str, &'a str)) -> &'a str {
    let filtered = guess_final_letter(word);
    // find_last_letter()은 한글이나 숫자가 없을 경우 ' '을 출력한다.
    // println!("마지막 글자 받침: {}", filtered);
    if filtered == 'N' {
        list.0
    } else if filtered == ' ' {
        list.1
    } else {
        list.2
    }
}

/// 비공개 함수 테스트
/// 위 함수를 lib.rs 에 올려서 공개로 만들지 않고 사용하기 위하여
/// 여기서 아래와 같이 비공개 함수 테스트 형식을 빌어서 테스트를 하겠습니다.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::EUL;
    #[test]
    fn _look_up_in_eul_ka_ida_neun() {
        let temp = "네이버";
        let result = "를";
        assert_eq!(result, look_up(temp, EUL));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "(을)를";
        assert_eq!(result, look_up(temp, EUL));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "을";
        assert_eq!(result, look_up(temp, EUL));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "을";
        assert_eq!(result, look_up(temp, EUL));
    }
}
