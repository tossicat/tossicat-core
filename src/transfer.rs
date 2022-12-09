//! ## 입력된 단어에 맞게 입력된 토시를 바꿔 변환하는 모듈
//! `tossi()` 함수가 메인 함수입니다.
//! 이 함수에 `Tossi` 구조체와 단어가 입력하면 그 구조체에 맞는 토시 변형 후보를
//! 선택하고, 그 구조체가 제시하고 있는 변형 방법에 맞는 변형 함수를 선택하게 됩니다.
//! 나머지 함수들을 현형 함수입니다.

const EUL: (&str, &str, &str) = ("(을)를", "를", "을");
const IDA: (&str, &str, &str) = ("(이)다", "다", "이다");
const KA: (&str, &str, &str) = ("(이)가", "가", "이");
const KO: (&str, &str, &str) = ("(이)고", "고", "이고");
const MYEO: (&str, &str, &str) = ("(이)며", "며", "이며");
const NA: (&str, &str, &str) = ("(이)나", "나", "이나");
const NAMA: (&str, &str, &str) = ("(이)나마", "나마", "이나마");
const NEUN: (&str, &str, &str) = ("(은)는", "는", "은");
const RAN: (&str, &str, &str) = ("(이)란", "란", "이란");
const RANG: (&str, &str, &str) = ("(이)랑", "랑", "이랑");
const YAMALRO: (&str, &str, &str) = ("(이)야말로", "야말로", "이야말로");
const RO: (&str, &str, &str) = ("(으)로", "로", "으로");
const ROBUTEO: (&str, &str, &str) = ("(으)로부터", "로부터", "으로부터");
const ROSEO: (&str, &str, &str) = ("(으)로서", "로서", "으로서");
const ROSSEO: (&str, &str, &str) = ("(으)로써", "로써", "으로써");

use crate::guess_final_letter;

use crate::identifier::{Tossi, TossiKind, TransTossiWhen};

/// `Tossi` 구조체와 단어가 입력하면 `Tossi` 구조체의 `kind`을 가지고
/// 토시 변형 후보를 선택하고, `Tossi` 구조체의 `when`을 가지고 있는
/// 변형 방법에 맞는 변형 함수를 선택합니다. 선택한 함수에 토시를 붙일 단어와
/// 토시 변형 후보들을 입력히면 적합한 토시를 반환합니다.
pub fn tossi(word: &str, tossi: Tossi) -> &str {
    let tossi_variants = match tossi.kind {
        TossiKind::Eul => EUL,
        TossiKind::Ida => IDA,
        TossiKind::Ka => KA,
        TossiKind::Ko => KO,
        TossiKind::Myeo => MYEO,
        TossiKind::Na => NA,
        TossiKind::Nama => NAMA,
        TossiKind::Neun => NEUN,
        TossiKind::Ran => RAN,
        TossiKind::Rang => RANG,
        TossiKind::Yamalro => YAMALRO,
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
/// ## 받침 없는 체언이나 ‘ㄹ’ 받침으로 끝나는 체언 뒤에 붙는 경우에 토시가 변환하는 함수
///
/// 이 함수는 현재 아래 목록에 있는 토시를 입력된 특정 문자열(단어)에 따라 변환합니다.
///
/// ```rust
/// const RO: (&str, &str, &str) = ("(으)로", "로", "으로");
/// const ROSEO: (&str, &str, &str) = ("(으)로서", "로서", "으로서");
/// const ROSSEO: (&str, &str, &str) = ("(으)로써", "로써", "으로써");
/// const ROBUTEO: (&str, &str, &str) = ("(으)로부터", "로부터", "으로부터");
/// ```
///
/// 입력된 특정 문자열(단어)의 마지막 글자의 종성만을 뽑아서 이 종성에 맞는
/// 앞의 토시 변화형 중 해당 토시에 적합한 것을 찾아서 반환해주는 역할을 합니다.
/// 각 토시의 상세한 변환 방법은 아래 내용을 참고해 주세요.
///
/// ### RO(로) 경우
/// - '로'는 받침 없는 체언이나 ‘ㄹ’ 받침으로 끝나는 체언 뒤에 붙습니다.
/// - '으로'는 ‘ㄹ’을 제외한 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(으)로'가 출력됩니다.
///
/// ### ROSEO(로서) 경우
/// - '로서'는 받침 없는 체언이나 ‘ㄹ’ 받침으로 끝나는 체언 뒤에 붙습니다.
/// - '으로서'는 ‘ㄹ’을 제외한 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(으)로서'가 출력됩니다.
///
/// ### ROSSEO(로써) 경우
/// - '로써'는 받침 없는 체언이나 ‘ㄹ’ 받침으로 끝나는 체언 뒤에 붙습니다.
/// - '으로써'는 ‘ㄹ’을 제외한 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(으)로써'가 출력됩니다.
///
/// ### ROBUTEO(로부터) 경우
/// - '로부터'는 받침 없는 체언이나 ‘ㄹ’ 받침으로 끝나는 체언 뒤에 붙습니다.
/// - '으로부터'는 ‘ㄹ’을 제외한 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(으)로부터'가 출력됩니다.

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

/// ## 받침 없는 체언 뒤에 붙는 경우에 토시가 변화하는 함수
///
/// 이 함수는 현재 아래 목록에 있는 토시를 입력된 특정 문자열(단어)에 따라 변환합니다.
///
/// ```rust
/// const EUL: (&str, &str, &str) = ("(을)를", "를", "을");
/// const KA: (&str, &str, &str) = ("(이)가", "가", "이");
/// const KO: (&str, &str, &str) = ("(이)고", "고", "이고");
/// const IDA: (&str, &str, &str) = ("(이)다", "다", "이다");
/// const NA: (&str, &str, &str) = ("(이)나", "나", "이나");
/// const NAMA: (&str, &str, &str) = ("(이)나마", "나마", "이나마");
/// const NEUN: (&str, &str, &str) = ("(은)는", "는", "은");
/// const RAN: (&str, &str, &str) = ("(이)란", "란", "이란");
/// const RANG: (&str, &str, &str) = ("(이)랑", "이", "이랑");
/// const MYEO: (&str, &str, &str) = ("(이)며", "며", "이며");
/// const YAMALRO: (&str, &str, &str) = ("(이)야말로", "야말로", "이야말로");
/// ```
///
/// 입력된 특정 문자열(단어)의 마지막 글자의 종성만을 뽑아서 이 종성에 맞는
/// 앞의 토시 변화형 중 해당 토시에 적합한 것을 찾아서 반환해주는 역할을 합니다.
/// 각 토시의 상세한 변환 방법은 아래 내용을 참고해 주세요.
///
/// ### EUL(을) 경우
/// - '를'는 받침 없는 체언 뒤에 붙습니다.
/// - '을'는 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(을)를'이 출력됩니다.
///
/// ### KA(가) 경우
/// - '가'는 받침 없는 체언 뒤에 붙습니다.
/// - '이'는 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(이)가'이 출력됩니다.
///
/// ### IDA(이다) 경우
/// - '다'는 받침 없는 체언 뒤에 붙습니다.
/// - '이다'는 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(이)다'이 출력됩니다.
///
/// ### NEUN(는) 경우
/// - '는'는 받침 없는 체언 뒤에 붙습니다.
/// - '은'는 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(은)는'이 출력됩니다.
///
/// ### NA(나) 경우
/// - '나'는 받침 없는 체언 뒤에 붙습니다.
/// - '이나'는 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(이)나'가 출력됩니다.
///
/// ### RANG(랑) 경우
/// - '랑'는 받침 없는 체언 뒤에 붙습니다.
/// - '이랑'는 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(이)랑'가 출력됩니다.
///
/// ### RAN(란) 경우
/// - '란'는 받침 없는 체언 뒤에 붙습니다.
/// - '이란'는 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(이)란'가 출력됩니다.
///
/// ### NAMA(나마) 경우
/// - '나마'는 받침 없는 체언 뒤에 붙습니다.
/// - '이나마'는 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(이)나마'가 출력됩니다.
///
/// ### YAMALRO(야말로) 경우
/// - '야말로'는 받침 없는 체언 뒤에 붙습니다.
/// - '이야말로'는 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(이)야말로'가 출력됩니다.
///
/// ### KO(고) 경우
/// - '고'는 받침 없는 체언 뒤에 붙습니다.
/// - '이고'는 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(이)고'가 출력됩니다.
///

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

/// 비공개 함수 테스트
/// 위 함수가 이 내부적으로 `pub`로 설정해 사용하지만,
/// `lib.rs`에 올려서 크레이트로 배포할 때 공개로 만들지 않고
/// 사용하기 위하여 여기서 아래와 같이 비공개 함수 테스트 형식을 빌어서 테스트를 하겠습니다.
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn _when_rieul_and_blank() {
        // 밭침이 없는 경우
        let temp = "네이버";
        let result = "로";
        assert_eq!(result, when_rieul_and_blank(temp, RO));
        // 받침이 있는 경우
        let temp = "법원";
        let result = "으로";
        assert_eq!(result, when_rieul_and_blank(temp, RO));
        // 받침에 `ㄹ`이 있는 경우
        let temp = "구글";
        let result = "로";
        assert_eq!(result, when_rieul_and_blank(temp, RO));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "(으)로";
        assert_eq!(result, when_rieul_and_blank(temp, RO));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "으로";
        assert_eq!(result, when_rieul_and_blank(temp, RO));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "으로";
        assert_eq!(result, when_rieul_and_blank(temp, RO));
    }
    #[test]
    fn _when_blank() {
        let temp = "네이버";
        let result = "를";
        assert_eq!(result, when_blank(temp, EUL));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "(을)를";
        assert_eq!(result, when_blank(temp, EUL));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "을";
        assert_eq!(result, when_blank(temp, EUL));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "을";
        assert_eq!(result, when_blank(temp, EUL));
    }
    #[test]
    fn _when_blank_na() {
        // 마지막 받침이 있는 경우
        let temp = "도서관";
        let result = "이나";
        assert_eq!(result, when_blank(temp, NA));
        // 마지막 받침이 없는 경우
        let temp = "학교";
        let result = "나";
        assert_eq!(result, when_blank(temp, NA));
        let temp = "어제";
        let result = "나";
        assert_eq!(result, when_blank(temp, NA));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "(이)나";
        assert_eq!(result, when_blank(temp, NA));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "이나";
        assert_eq!(result, when_blank(temp, NA));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "이나";
        assert_eq!(result, when_blank(temp, NA));
    }
    #[test]
    fn _when_blank_rang() {
        // 마지막 받침이 있는 경우
        let temp = "도서관";
        let result = "이랑";
        assert_eq!(result, when_blank(temp, RANG));
        // 마지막 받침이 없는 경우
        let temp = "학교";
        let result = "랑";
        assert_eq!(result, when_blank(temp, RANG));
        let temp = "어제";
        let result = "랑";
        assert_eq!(result, when_blank(temp, RANG));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "(이)랑";
        assert_eq!(result, when_blank(temp, RANG));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "이랑";
        assert_eq!(result, when_blank(temp, RANG));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "이랑";
        assert_eq!(result, when_blank(temp, RANG));
    }
    #[test]
    fn _when_blank_ran() {
        // 마지막 받침이 있는 경우
        let temp = "사람";
        let result = "이란";
        assert_eq!(result, when_blank(temp, RAN));
        let temp = "책";
        let result = "이란";
        assert_eq!(result, when_blank(temp, RAN));
        // 마지막 받침이 없는 경우
        let temp = "우주";
        let result = "란";
        assert_eq!(result, when_blank(temp, RAN));
        let temp = "어제";
        let result = "란";
        assert_eq!(result, when_blank(temp, RAN));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "(이)란";
        assert_eq!(result, when_blank(temp, RAN));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "이란";
        assert_eq!(result, when_blank(temp, RAN));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "이란";
        assert_eq!(result, when_blank(temp, RAN));
    }
    #[test]
    fn _when_blank_nama() {
        // 마지막 받침이 있는 경우
        let temp = "못하";
        let result = "나마";
        assert_eq!(result, when_blank(temp, NAMA));
        let temp = "책";
        let result = "이나마";
        assert_eq!(result, when_blank(temp, NAMA));
        // 마지막 받침이 없는 경우
        let temp = "몸";
        let result = "이나마";
        assert_eq!(result, when_blank(temp, NAMA));
        let temp = "힘";
        let result = "이나마";
        assert_eq!(result, when_blank(temp, NAMA));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "(이)나마";
        assert_eq!(result, when_blank(temp, NAMA));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "이나마";
        assert_eq!(result, when_blank(temp, NAMA));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "이나마";
        assert_eq!(result, when_blank(temp, NAMA));
    }
    #[test]
    fn _when_blank_myeo() {
        // 마지막 받침이 있는 경우
        let temp = "그림";
        let result = "이며";
        assert_eq!(result, when_blank(temp, MYEO));
        let temp = "조각";
        let result = "이며";
        assert_eq!(result, when_blank(temp, MYEO));
        // 마지막 받침이 없는 경우
        let temp = "학자";
        let result = "며";
        assert_eq!(result, when_blank(temp, MYEO));
        let temp = "정치가";
        let result = "며";
        assert_eq!(result, when_blank(temp, MYEO));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "(이)며";
        assert_eq!(result, when_blank(temp, MYEO));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "이며";
        assert_eq!(result, when_blank(temp, MYEO));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "이며";
        assert_eq!(result, when_blank(temp, MYEO));
    }
    #[test]
    fn _when_blank_yamalro() {
        // 마지막 받침이 있는 경우
        let temp = "삶";
        let result = "이야말로";
        assert_eq!(result, when_blank(temp, YAMALRO));
        let temp = "조각";
        let result = "이야말로";
        assert_eq!(result, when_blank(temp, YAMALRO));
        // 마지막 받침이 없는 경우
        let temp = "한자";
        let result = "야말로";
        assert_eq!(result, when_blank(temp, YAMALRO));
        let temp = "정치가";
        let result = "야말로";
        assert_eq!(result, when_blank(temp, YAMALRO));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "(이)야말로";
        assert_eq!(result, when_blank(temp, YAMALRO));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "이야말로";
        assert_eq!(result, when_blank(temp, YAMALRO));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "이야말로";
        assert_eq!(result, when_blank(temp, YAMALRO));
    }
    #[test]
    fn _when_blank_ko() {
        // 마지막 받침이 있는 경우
        let temp = "산";
        let result = "이고";
        assert_eq!(result, when_blank(temp, KO));
        let temp = "강";
        let result = "이고";
        assert_eq!(result, when_blank(temp, KO));
        // 마지막 받침이 없는 경우
        let temp = "공부";
        let result = "고";
        assert_eq!(result, when_blank(temp, KO));
        let temp = "뭐";
        let result = "고";
        assert_eq!(result, when_blank(temp, KO));
        let temp = "직장에서";
        let result = "고";
        assert_eq!(result, when_blank(temp, KO));
        let temp = "가정에서고";
        let result = "고";
        assert_eq!(result, when_blank(temp, KO));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "(이)고";
        assert_eq!(result, when_blank(temp, KO));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "이고";
        assert_eq!(result, when_blank(temp, KO));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "이고";
        assert_eq!(result, when_blank(temp, KO));
    }
}
