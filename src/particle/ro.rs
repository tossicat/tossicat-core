//! # '~ 로' 또는 '~ 으로'인지 판단하는 모듈
//!
//! - '로'는 받침 없는 체언이나 ‘ㄹ’ 받침으로 끝나는 체언 뒤에 붙습니다.
//! - '으로'는 ‘ㄹ’을 제외한 받침 있는 체언 뒤에 붙습니다.
//! - 외국어가 앞 단어로 오는 경우 병기 '(으)로'가 출력됩니다.
//!
//! https://github.com/LOPES-HUFS/tossi_for_rust/wiki/'~-로---~-으로'에-대하여
//!

use crate::guess_final;

pub fn change(word: &str) -> String {
    let fin = guess_final(word);
    if fin == 'N' {
        return "(으)로".to_string();
    }
    if fin == ' ' || fin == 'ㄹ' {
        "로".to_string()
    } else {
        "으로".to_string()
    }
}
