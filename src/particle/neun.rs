//! # '~ 는' 또는 '~은'인지 판단하는 모듈
//!
//! - '는'는 받침 없는 체언 뒤에 붙습니다.
//! - '은'는 받침 있는 체언 뒤에 붙습니다.
//! - 외국어가 앞 단어로 오는 경우 병기 '(은)는'이 출력됩니다.
//!

use crate::guess_final;

pub fn change(word: &str) -> String {
    let fin = guess_final(word);
    if fin == 'N' {
        return "(은)는".to_string();
    }
    if fin == ' ' {
        "는".to_string()
    } else {
        "은".to_string()
    }
}
