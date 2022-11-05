//! # 기타 문자들을 처리해주는 모듈 -> 숫자는 number 모듈로 이동
//!
//! ## guess_final
//! 종성만 찾아서 도출해주는 함수
//! ```text
//! ex) 류 -> ' '
//! ex) 영 -> 'ㅇ'
//! ex) K(한글이 아닌 경우) -> 'N'
//! ```
//!
//! ## find_last_letter
//! 단어에서 마지막 글자를 찾아주는 함수.
//! 불필요한 요소를 삭제한 후 그 결과에서 마지막 글자를 반환한다.  
//! ```text
//! ex) 넥슨(코리아) -> 넥슨 -> 슨  
//! ex) 비타500 -> 비타오영영 -> 영  
//! ```
//!
//! ## filter_only_significant
//! 단어에서 불필요한 요소(기호 등)들을 제거하는 함수.
//! 기호인 경우 삭제하며, 괄호에 들어간 글자들도 삭제한다.  
//! 숫자인 경우 숫자의 한글발음으로 변경해준다.  
//! ```text
//! ex) 넥슨(코리아) -> [넥,슨]  
//! ex) 비타500 -> [비,타,오,백]  
//! ```

use crate::hangeul::{is_hangeul, split_phonemes};
use crate::number::{change_num_to_hangeul, is_digits};

/// ## 종성만 찾아서 도출해주는 함수
/// 이 함수는 특정 글자의 종성만 도출합니다.
/// #[allow(dead_code)]
pub fn guess_final(word: &str) -> char {
    let filtered = find_last_letter(word);
    // find_last_letter()은 한글이나 숫자가 없을 경우 ' '을 출력한다.
    if filtered == 'N' {
        return 'N';
    } else {
        return split_phonemes(filtered)[2];
    }
}

/// ## 단어에서 마지막 글자를 찾아주는 함수
/// 'N'을 도출한 경우 영어 포함 외국어이다. -> 병기로 연결
pub fn find_last_letter(word: &str) -> char {
    let filtered = filter_only_significant(word);
    if filtered.len() > 0 {
        return filtered[filtered.len() - 1];
    } else {
        return 'N';
    }
}

/// ## 단어에서 불필요한 요소를 제거하는 함수
/// 토시를 선택할 때 불필요한 것들을 아래 리스트를 고려하여 제거한다.
/// 1. '넥슨(코리아)'과 같은 것에 토시를 추가하는 경우 통상적으로 괄호를 제외하고
/// '넥슨'에 맟춰 토스를 선택한다. 따라서 토시를 선택하기 위해서
/// 괄호 안에 있는 한글 글자들은 괄호와 함께 제거하고 그 나머지 것으로 토시 선택을
/// 고려해야 한다.
/// 2. 기본적으로 한글이 아닌 외국어에 대해서는 토시를 추가하는 것을 고려하지 않는다.
/// 따라서 한글 글자가 아닌 글자들은 불필요한 요소로 간주한다.
/// 3. 숫자인 경우에는 숫자를 읽는 함수를 통해 한글로 바꾼 다음, 이를 가지고
/// 토시 선택을 고려한다.
pub fn filter_only_significant(word: &str) -> Vec<char> {
    let word_len = word.chars().count();
    let mut output: Vec<char> = Vec::new();
    let mut numbers = String::new();
    let mut bracket: bool = false;
    for (i, c) in word.chars().enumerate() {
        //괄호 있는지 확인
        if c == '(' {
            bracket = true;
        } else if c == ')' {
            bracket = false;
        }
        //한글이 아니라면 제거
        if bracket {
            continue;
        } else if is_hangeul(c) {
            output.push(c);
        }
        //숫자라면 모아서 변형 후 아웃풋에 추가
        if is_digits(c) {
            numbers.push(c);
        }
        if !is_digits(c) || (i == word_len - 1) {
            let num = change_num_to_hangeul(&numbers);
            let mut arr_num = num.chars().collect();
            output.append(&mut arr_num);
        }
    }
    return output;
}

/// 비 공개 함수 테스트
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _filter_only_significant() {
        let temp = "넥슨(코리아)";
        let result = vec!['넥', '슨'];
        assert_eq!(result, filter_only_significant(temp));

        let temp = "비타500";
        let result = vec!['비', '타', '오', '백'];
        assert_eq!(result, filter_only_significant(temp));
    }
}
