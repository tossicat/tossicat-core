//! # '~ 다' 또는 '~ 이다'인지 판단하는 모듈
//!
//! - '로'는 받침 없는 체언이나 ‘ㄹ’ 받침으로 끝나는 체언 뒤에 붙습니다.
//! - '으로'는 ‘ㄹ’을 제외한 받침 있는 체언 뒤에 붙습니다.
//!

use crate::guess_final_letter;

pub fn change(word: &str) -> String {
    let fin = guess_final_letter(word);
    if fin == 'N' {
        return "(이)다".to_string();
    }
    if fin == ' ' {
        "다".to_string()
    } else {
        "이다".to_string()
    }
}
