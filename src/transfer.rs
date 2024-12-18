//! ## 입력된 단어에 맞게 입력된 토시를 바꿔 변환하는 모듈
//! `tossi()` 함수가 메인 함수입니다.
//!  이 함수에 `Tossi` 구조체와 단어가 입력하면 그 구조체에 맞는 토시 변형 후보를
//!  선택하고, 그 구조체가 제시하고 있는 변형 방법에 맞는 변형 함수를 선택하게 됩니다.
//!  나머지 함수들을 현형 함수입니다.
//!
//! ### 상수 목록에 대햐여
//!
//! 아래 목록에서 개별 목록 안에 있는 토시들은 같은 의미를 가진 토시라고 봐도 무방합니다.
//!  물론 이렇게 쉽게 단정할 수는 없습니다. 그러나 이 프로젝트의 목적이 변환이기 때문에
//!  각 항목이 의미를 가진 토시들라고 보는 것이 변환하는데 더 편리힙니다.
//!  원칙적으로 0번째에 있는 토시는 외국어인 경우에 사용하는 토시입니다.
//!  그러나 예외가 있습니다. 다음 목록을 참고하세요.
//!
//! - INDEUL, INJEUK 경우에는 외국어의 문자에 `ㄴ`을 추가할 수 없기 때문에
//! 이 토시 종류를 명확하게 하기 위해서 0번째에 `ㄴ`을 넣었습니다.
//! 이와 관련된 내용은 `when_last_jamo_nieun()` 함수 설명 부분을
//! 참고하세요.
//!
const DEUN: (&str, &str, &str) = ("(이)든", "든", "이든");
const DEUNJI: (&str, &str, &str) = ("(이)든지", "든지", "이든지");
const DEUNKA: (&str, &str, &str) = ("(이)든가", "든가", "이든가");
const EUL: (&str, &str, &str) = ("(을)를", "를", "을");
const IDA: (&str, &str, &str) = ("(이)다", "다", "이다");
const INDEUL: (&str, &str, &str) = ("ㄴ", "들", "인들");
const INJEUK: (&str, &str, &str) = ("ㄴ", "즉", "인즉");
const ILLANG: (&str, &str, &str) = ("ㄹ", "랑", "일랑");
const KA: (&str, &str, &str) = ("(이)가", "가", "이");
const KO: (&str, &str, &str) = ("(이)고", "고", "이고");
const MYEO: (&str, &str, &str) = ("(이)며", "며", "이며");
const NA: (&str, &str, &str) = ("(이)나", "나", "이나");
const NAMA: (&str, &str, &str) = ("(이)나마", "나마", "이나마");
const NEUN: (&str, &str, &str) = ("(은)는", "는", "은");
const NI: (&str, &str, &str) = ("(이)니", "니", "이니");
const RADO: (&str, &str, &str) = ("(이)라도", "라도", "이라도");
const RAGO: (&str, &str, &str) = ("(이)라고", "라고", "이라고");
const RAN: (&str, &str, &str) = ("(이)란", "란", "이란");
const RANG: (&str, &str, &str) = ("(이)랑", "랑", "이랑");
const RAYA: (&str, &str, &str) = ("(이)라야", "라야", "이라야");
const RO: (&str, &str, &str) = ("(으)로", "로", "으로");
const ROBUTEO: (&str, &str, &str) = ("(으)로부터", "로부터", "으로부터");
const ROSEO: (&str, &str, &str) = ("(으)로서", "로서", "으로서");
const ROSSEO: (&str, &str, &str) = ("(으)로써", "로써", "으로써");
const WA: (&str, &str, &str) = ("(와)과", "와", "과");
const YAMALRO: (&str, &str, &str) = ("(이)야말로", "야말로", "이야말로");
const YEO: (&str, &str, &str) = ("(이)여", "여", "이여");

use crate::guess_final_letter;
use crate::modify_finall_jamo;

use crate::identifier::{Tossi, TossiKind, TransTossiWhen};

/// ## 입력된 토시를 어떻게 변혈할지 방법을 설정하고 변형 목록을 제공하는 함수
///
/// `identifier` 모듈을 통헤서 입력된 토시가 어떤 토시인지 절정할 수 있습니다.
///  `Tossi` 구조체와 단어가 입력하면 `Tossi` 구조체의 `kind`을 가지고
///  토시 변형 후보를 선택하고, `Tossi` 구조체의 `when`을 가지고 있는
///  변형 방법에 맞는 변형 함수를 선택합니다. 선택한 함수에 토시를 붙일 단어와
///  토시 변형 후보들을 입력히면 적합한 토시를 반환합니다.
///
///  참고로 아래 목록 순서는 알파벳 순으로 바꿀 필요가 없습니다.
///  왜냐하면 추가할 때 같은 처리 형식 묶음으로 엮어서 처리하고 있기 때문에
///  현재 순서가 의미가 있기 때문입니다.
pub fn tossi(word: &str, tossi: Tossi) -> String {
    let tossi_variants = match tossi.kind {
        TossiKind::Deun => DEUN,
        TossiKind::Deunka => DEUNKA,
        TossiKind::Deunji => DEUNJI,
        TossiKind::Eul => EUL,
        TossiKind::Ida => IDA,
        TossiKind::Indeul => INDEUL,
        TossiKind::Injeuk => INJEUK,
        TossiKind::Ka => KA,
        TossiKind::Ko => KO,
        TossiKind::Myeo => MYEO,
        TossiKind::Na => NA,
        TossiKind::Nama => NAMA,
        TossiKind::Neun => NEUN,
        TossiKind::Ni => NI,
        TossiKind::Rado => RADO,
        TossiKind::Rago => RAGO,
        TossiKind::Ran => RAN,
        TossiKind::Rang => RANG,
        TossiKind::Raya => RAYA,
        TossiKind::Yamalro => YAMALRO,
        TossiKind::Yeo => YEO,
        TossiKind::Wa => WA,
        TossiKind::Ro => RO,
        TossiKind::Roseo => ROSEO,
        TossiKind::Rosseo => ROSSEO,
        TossiKind::Robuteo => ROBUTEO,
        TossiKind::Illang => ILLANG,
        TossiKind::Others => (" ", " ", " "),
    };

    match tossi.when {
        TransTossiWhen::Blank => when_blank(word, tossi_variants).to_string(),
        TransTossiWhen::RiEulAndBlank => when_rieul_and_blank(word, tossi_variants).to_string(),
        TransTossiWhen::OnlyKa => only_ka(word, tossi_variants),
        TransTossiWhen::LastJamoNieun => when_last_jamo_nieun(word, tossi_variants),
        TransTossiWhen::LastJamoRieul => when_last_jamo_rieul(word, tossi_variants),
        TransTossiWhen::Nothing => " ".to_string(),
    }
}

/// ## 변형된 단어와 토씨를 분리하는 함수
///
/// `split_word_tossi` 함수는 변형된 단어와 토씨를 원래 단어와 토씨로 분리하여 반환합니다.
/// `word`와 `Tossi'를 입력하면, 각 토씨와 일치하는지 확인한 뒤,
/// 원래의 단어와 토씨로 나누어 반환합니다.
/// ```
pub fn split_word_tossi(word: &str, tossi: Tossi) -> (String, String) {
    let tossi_variants = match tossi.kind {
        TossiKind::Deun => DEUN,
        TossiKind::Deunka => DEUNKA,
        TossiKind::Deunji => DEUNJI,
        TossiKind::Eul => EUL,
        TossiKind::Ida => IDA,
        TossiKind::Indeul => INDEUL,
        TossiKind::Injeuk => INJEUK,
        TossiKind::Ka => KA,
        TossiKind::Ko => KO,
        TossiKind::Myeo => MYEO,
        TossiKind::Na => NA,
        TossiKind::Nama => NAMA,
        TossiKind::Neun => NEUN,
        TossiKind::Ni => NI,
        TossiKind::Rado => RADO,
        TossiKind::Rago => RAGO,
        TossiKind::Ran => RAN,
        TossiKind::Rang => RANG,
        TossiKind::Raya => RAYA,
        TossiKind::Yamalro => YAMALRO,
        TossiKind::Yeo => YEO,
        TossiKind::Wa => WA,
        TossiKind::Ro => RO,
        TossiKind::Roseo => ROSEO,
        TossiKind::Rosseo => ROSSEO,
        TossiKind::Robuteo => ROBUTEO,
        TossiKind::Illang => ILLANG,
        TossiKind::Others => (" ", " ", " "),
    };

    let mut variants = [tossi_variants.0, tossi_variants.1, tossi_variants.2];
    variants.sort_by_key(|b| std::cmp::Reverse(b.len()));

    for tossi in variants {
        if word.ends_with(tossi) {
            let word_length = word.len() - tossi.len();
            let word = &word[..word_length];
            return (word.to_string(), tossi.to_string());
        }
    }
    // 해당하는 토씨가 없을 경우, 원래 문자열을 반환합니다.
    (word.to_string(), "".to_string())
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

/// ## 체언 뒤에 붙는 "이 / 가" 토시를 변환하는 함수
///
/// 이 함수는 현재 아래 목록에 있는 토시를 입력된 특정 문자열(단어)에 따라 변환합니다.
///
/// ```rust
/// const KA: (&str, &str, &str) = ("(이)가", "가", "이");
/// ```
///
/// - '가'는 받침 없는 체언 뒤에 붙습니다.
/// - '이'는 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(이)가'가 출력됩니다.
///
/// 그러나 ‘가’가 붙어야 하는 "나", "누구", "저"와 같은 것이 오면 위와 같이 토시는 변환하지만,
/// 같이 입력된 단어도 함께 변합니다.
///
/// - 누구 + 가(이) : 누가
/// - 나 + 가(이): 내가
/// - 저 + 가(이): 제가
/// - 너 + 가(이): 네가

fn only_ka<'a>(word: &'a str, tossi_variants: (&'a str, &'a str, &'a str)) -> String {
    let filtered = guess_final_letter(word);
    // find_last_letter()은 한글이나 숫자가 없을 경우 ' '을 출력한다.
    // println!("마지막 글자 받침: {}", filtered);
    if filtered == 'N' {
        word.to_string() + tossi_variants.0
    } else if filtered == ' ' {
        if word == "누구" {
            "누가".to_string()
        } else if word == "나" {
            "내가".to_string()
        } else if word == "저" {
            "제가".to_string()
        } else if word == "너" {
            "네가".to_string()
        } else {
            word.to_string() + tossi_variants.1
        }
    } else {
        word.to_string() + tossi_variants.2
    }
}

/// ## 받침 없는 체언 뒤에 붙는 경우에 그 체언에 "ㄴ"을 추가하는 토시 변환
///
/// 이 함수는 받침 있는 체언 뒤에는 그대로 토시가 붙지만,
///  받침 없는 체언 뒤에 붙는 경우에는 그 체언에 "ㄴ"을 추가하고 해당 토시는
///  그대로 붙는 토시들을 변환하는 함수입니다.
///
/// 현재 아래 목록에 있는 토시를 입력된 특정 문자열(단어)에 따라 변환합니다.
///
/// ### NINDEUL(인들) 경우
///
/// INDEUL 경우에는 외국어의 문자에 `ㄴ`을 추가할 수 없기 때문에
/// 이 토시 종류를 명확하게 하기 위해서 0번째에 `ㄴ`을 넣었습니다.
/// 왜냐하면 이 토시 모음은 "ㄴ들"과 "인들"을 위한 것이지
/// "들"을 위한 것이 아니기 때문입니다. 물론 0번째가 아니라 1번쩨 칸에
/// "ㄴ들"을 넣으면 더 명확할 수 있지만, 그렇게 된다면 이 "ㄴ들"에서 `str`을 분리해서
/// "들" 뽑아야 하는데 rust에서는 불필요한 코드를 많이 작성하게 됩니다.
/// 그러나 아래와 같이 하면 깔끔하게 됩니다.
///
/// ```rust
/// const INDEUL: (&str, &str, &str) = ("ㄴ", "들", "인들");
/// ```
///
/// - '인들'은 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 '인들'이 붙습니다.
///
/// 그러나 받침 없는 체언 뒤에서는 체언의 마지막 글자에 'ㄴ'를 붙이고 토시 '들'을 붙입니다.
///
/// 예를 들면 다음과 같습니다.
///
/// - "나" + "인들" = "난들"
///
/// ### INJEUK(인즉) 경우
///
/// INJEUK 경우에는 외국어의 문자에 `ㄴ`을 추가할 수 없기 때문에
/// 이 토시 종류를 명확하게 하기 위해서 0번째에 `ㄴ`을 넣었습니다.
/// 왜냐하면 이 토시 모음은 "ㄴ즉"과 "인즉"을 위한 것이지
/// "들"을 위한 것이 아니기 때문입니다. 물론 0번째가 아니라 1번쩨 칸에
/// "ㄴ들"을 넣으면 더 명확할 수 있지만, 그렇게 된다면 이 "ㄴ즉"에서 `str`을 분리해서
/// "즉" 뽑아야 하는데 rust에서는 불필요한 코드를 많이 작성하게 됩니다.
/// 그러나 아래와 같이 하면 깔끔하게 됩니다.
///
/// ```rust
/// const INJEUK: (&str, &str, &str) = ("ㄴ", "즉", "인즉");
/// ```
///
/// - '인들'은 받침 있는 체언 뒤에 붙어, ‘으로 말하면’의 뜻으로 쓰이는 보조사.
/// - 외국어가 앞 단어로 오는 경우 '인즉'이 붙습니다.
///
/// 그러나 받침 없는 체언 뒤에서는 체언의 마지막 글자에 'ㄴ'를 붙이고 토시 '즉'을 붙입니다.
/// 뜻은 ‘로 말하면’의 뜻을 나타내는 보조사.
///
/// 예를 들면 다음과 같습니다.
///
/// 얘기∼ 그게 옳다. - 얘긴즉 그게 옳다.
///

fn when_last_jamo_nieun<'a>(word: &'a str, tossi_variants: (&'a str, &'a str, &'a str)) -> String {
    let filtered = guess_final_letter(word);
    // find_last_letter()은 한글이나 숫자가 없을 경우 ' '을 출력한다.
    // println!("마지막 글자 받침: {}", filtered);
    if filtered == 'N' {
        word.to_string() + tossi_variants.2
    } else if filtered == ' ' {
        let mut temp_word: String = word.to_string();
        let last_char = temp_word.pop();
        // 여기 조건문이 필요 없을 것 같은데 Some으로 들어오기 때문에
        // 현재 현재 아는 바로는 이렇게 처리할 수 밖에 없다.
        if let Some(temp) = last_char {
            let temp_last_char = modify_finall_jamo(temp, 'ㄴ');
            temp_word.push(temp_last_char);
            return temp_word + tossi_variants.1;
        } else {
            word.to_string() + tossi_variants.1
        }
    } else {
        word.to_string() + tossi_variants.2
    }
}

/// ## 받침 없는 체언 뒤에 붙는 경우에 그 체언에 "ㄹ"을 추가하는 토시 변환
///
/// 이 함수는 받침 있는 체언 뒤에는 그대로 토시가 붙지만,
///  받침 없는 체언 뒤에 붙는 경우에는 그 체언에 "ㄹ"을 추가하고 해당 토시는
///  그대로 붙는 토시들을 변환하는 함수입니다.
///
/// 현재 아래 목록에 있는 토시를 입력된 특정 문자열(단어)에 따라 변환합니다.
///
/// ### ILLANG(일랑) 경우
///
/// ILLANG 경우에는 외국어의 문자에 `ㄹ`을 추가할 수 없기 때문에
/// 이 토시 종류를 명확하게 하기 위해서 0번째에 `ㄹ`을 넣었습니다.
/// 왜냐하면 이 토시 모음은 "ㄹ랑"과 "인랑"을 위한 것이지
/// "랑"을 위한 것이 아니기 때문입니다. 물론 0번째가 아니라 1번쩨 칸에
/// "ㄹ랑"을 넣으면 더 명확할 수 있지만, 그렇게 된다면 이 "ㄹ랑"에서 `str`을 분리해서
/// "랑" 뽑아야 하는데 rust에서는 불필요한 코드를 많이 작성하게 됩니다.
/// 그러나 아래와 같이 하면 깔끔하게 됩니다.
///
/// ```rust
/// const ILLANG: (&str, &str, &str) = ("ㄹ", "랑", "일랑");
/// ```
///
/// - '인랑'은 받침 있는 체언 뒤에 붙어, 어떤 대상을 특별히 지적하는 뜻을 나타내는 보조사.
/// 는. 자네∼ 부디 착하게 살게 | 이사 갈 때 낡은 가구∼ 버립시다 | 위험하니까 길가에서∼ 놀지 말아라 | 여기서∼ 잠깐 기다리고 계세요. ▷ 일랑.
/// - 외국어가 앞 단어로 오는 경우 '인랑'이 붙습니다.
///
/// 그러나 받침 없는 체언 뒤에서는 체언의 마지막 글자에 'ㄹ'를 붙이고 토시 '랑'을 붙입니다.
///
/// 예를 들면 다음과 같습니다.
///
/// 이사 갈 때 낡은 가굴랑 버립시다.
/// 자네일랑 부디 착하게 살게
/// 술일랑 제발 그만 마시세요.
/// 그 여자에 대한 미련일랑 버려라.

fn when_last_jamo_rieul<'a>(word: &'a str, tossi_variants: (&'a str, &'a str, &'a str)) -> String {
    let filtered = guess_final_letter(word);
    // find_last_letter()은 한글이나 숫자가 없을 경우 ' '을 출력한다.
    // println!("마지막 글자 받침: {}", filtered);
    if filtered == 'N' {
        word.to_string() + tossi_variants.2
    } else if filtered == ' ' {
        let mut temp_word: String = word.to_string();
        let last_char = temp_word.pop();
        // 여기 조건문이 필요 없을 것 같은데 Some으로 들어오기 때문에
        // 현재 현재 아는 바로는 이렇게 처리할 수 밖에 없다.
        if let Some(temp) = last_char {
            let temp_last_char = modify_finall_jamo(temp, 'ㄹ');
            temp_word.push(temp_last_char);
            return temp_word + tossi_variants.1;
        } else {
            word.to_string() + tossi_variants.1
        }
    } else {
        word.to_string() + tossi_variants.2
    }
}
/// ## 받침 없는 체언 뒤에 붙는 경우에 토시가 변화하는 함수
///
/// 이 함수는 현재 아래 목록에 있는 토시를 입력된 특정 문자열(단어)에 따라 변환합니다.
///
/// ```rust
/// const EUL: (&str, &str, &str) = ("(을)를", "를", "을");
/// const KO: (&str, &str, &str) = ("(이)고", "고", "이고");
/// const IDA: (&str, &str, &str) = ("(이)다", "다", "이다");
/// const NA: (&str, &str, &str) = ("(이)나", "나", "이나");
/// const NAMA: (&str, &str, &str) = ("(이)나마", "나마", "이나마");
/// const NEUN: (&str, &str, &str) = ("(은)는", "는", "은");
/// const NI: (&str, &str, &str) = ("(이)니", "니", "이니");
/// const RAN: (&str, &str, &str) = ("(이)란", "란", "이란");
/// const RANG: (&str, &str, &str) = ("(이)랑", "이", "이랑");
/// const MYEO: (&str, &str, &str) = ("(이)며", "며", "이며");
/// const YAMALRO: (&str, &str, &str) = ("(이)야말로", "야말로", "이야말로");
/// const DEUN: (&str, &str, &str) = ("(이)든", "든", "이든");
/// const DEUNJI: (&str, &str, &str) = ("(이)든지", "든지", "이든지");
/// const DEUNKA: (&str, &str, &str) = ("(이)든가", "든가", "이든가");
/// const YEO: (&str, &str, &str) = ("(이)여", "여", "이여");
/// const RAYA: (&str, &str, &str) = ("(이)라야", "라야", "이라야");
/// const WA: (&str, &str, &str) = ("(와)과", "와", "과");
/// const RADO: (&str, &str, &str) = ("(이)라도", "라도", "이라도");
/// ```
///
/// 입력된 특정 문자열(단어)의 마지막 글자의 종성만을 뽑아서 이 종성에 맞는
/// 앞의 토시 변화형 중 해당 토시에 적합한 것을 찾아서 반환해주는 역할을 합니다.
/// 각 토시의 상세한 변환 방법은 아래 내용을 참고해 주세요.
///
/// ### EUL(을) 경우
/// - '를'은 받침 없는 체언 뒤에 붙습니다.
/// - '을'은 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(을)를'이 출력됩니다.
///
/// ### IDA(이다) 경우
/// - '다'는 받침 없는 체언 뒤에 붙습니다.
/// - '이다'는 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(이)다'가 출력됩니다.
///
/// ### NEUN(는) 경우
/// - '는'은 받침 없는 체언 뒤에 붙습니다.
/// - '은'은 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(은)는'이 출력됩니다.
///
/// ### NA(나) 경우
/// - '나'는 받침 없는 체언 뒤에 붙습니다.
/// - '이나'는 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(이)나'가 출력됩니다.
///
/// ### RANG(랑) 경우
/// - '랑'은 받침 없는 체언 뒤에 붙습니다.
/// - '이랑'은 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(이)랑'이 출력됩니다.
///
/// ### RAN(란) 경우
/// - '란'은 받침 없는 체언 뒤에 붙습니다.
/// - '이란'은 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(이)란'이 출력됩니다.
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
/// ### Ni(니) 경우
/// - '니'는 받침 없는 체언 뒤에 붙습니다.
/// - '이니'는 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(이)니'가 출력됩니다.
///
/// ### Deun(든) 경우
/// - '든'은 받침 없는 체언 뒤에 붙습니다.
/// - '이든'은 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(이)든'이 출력됩니다.
///
/// ### Deunji(든지) 경우
/// - '든지'는 받침 없는 체언 뒤에 붙습니다.
/// - '이든지'는 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(이)든지'가 출력됩니다.
///
/// ### Deunka(든가) 경우
/// - '든가'는 받침 없는 체언 뒤에 붙습니다.
/// - '이든가'는 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(이)든가'가 출력됩니다.
///
/// ### YEO(여) 경우
/// - '여'는 받침 없는 체언 뒤에 붙습니다.
/// - '이여'는 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(이)여'가 출력됩니다.
///
/// ### RAYA(라야) 경우
/// - '라야'는 받침 없는 체언 뒤에 붙습니다.
/// - '이라야'는 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(이)라야'가 출력됩니다.
///
/// ### WA(와) 경우
/// - '와'는 받침 없는 체언 뒤에 붙습니다.
/// - '과'는 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(와)과'가 출력됩니다.
///
/// ### RADO(라도) 경우
/// - '라도'는 받침 없는 체언 뒤에 붙습니다.
/// - '이라도'는 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(이)라도'가 출력됩니다.
///
/// ### RAGO(라고) 경우
/// - '라고'는 받침 없는 체언 뒤에 붙습니다.
/// - '이라고'는 받침 있는 체언 뒤에 붙습니다.
/// - 외국어가 앞 단어로 오는 경우 병기 '(이)라고'가 출력됩니다.

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
    fn _when_blank_rago() {
        // 마지막 받침이 있는 경우
        let temp = "고향";
        let result = "이라고";
        assert_eq!(result, when_blank(temp, RAGO));
        // 마지막 받침이 없는 경우
        let temp = "자네";
        let result = "라고";
        assert_eq!(result, when_blank(temp, RAGO));
        let temp = "아이";
        let result = "라고";
        assert_eq!(result, when_blank(temp, RAGO));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "(이)라고";
        assert_eq!(result, when_blank(temp, RAGO));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "이라고";
        assert_eq!(result, when_blank(temp, RAGO));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "이라고";
        assert_eq!(result, when_blank(temp, RAGO));
    }

    #[test]
    fn _when_only_ka() {
        // 밭침이 없는 경우
        let temp = "철수";
        let result = "철수가";
        assert_eq!(result, only_ka(temp, KA));
        // 받침이 있는 경우
        let temp = "법원";
        let result = "법원이";
        assert_eq!(result, only_ka(temp, KA));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "google(이)가";
        assert_eq!(result, only_ka(temp, KA));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "넥슨(코리아)이";
        assert_eq!(result, only_ka(temp, KA));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "비타500이";
        assert_eq!(result, only_ka(temp, KA));
        // - 가 / 이 변환의 특수한 경우를 테스트 합니다.
        // 누구, 나, 저, 너
        let temp = "누구";
        let result = "누가";
        assert_eq!(result, only_ka(temp, KA));
        let temp = "나";
        let result = "내가";
        assert_eq!(result, only_ka(temp, KA));
        let temp = "저";
        let result = "제가";
        assert_eq!(result, only_ka(temp, KA));
        let temp = "너";
        let result = "네가";
        assert_eq!(result, only_ka(temp, KA));
    }

    #[test]

    fn _when_when_last_jamo_rieul() {
        // 밭침이 없는 경우
        let temp = "자네";
        let result = "자넬랑";
        assert_eq!(result, when_last_jamo_rieul(temp, ILLANG));
        let temp = "여기서";
        let result = "여기설랑";
        assert_eq!(result, when_last_jamo_rieul(temp, ILLANG));
        let temp = "나";
        let result = "날랑";
        assert_eq!(result, when_last_jamo_rieul(temp, ILLANG));
        // 받침이 있는 경우
        let temp = "술";
        let result = "술일랑";
        assert_eq!(result, when_last_jamo_rieul(temp, ILLANG));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "google일랑";
        assert_eq!(result, when_last_jamo_rieul(temp, ILLANG));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "naver";
        let result = "naver일랑";
        assert_eq!(result, when_last_jamo_rieul(temp, ILLANG));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "넥슨(코리아)일랑";
        assert_eq!(result, when_last_jamo_rieul(temp, ILLANG));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "비타500일랑";
        assert_eq!(result, when_last_jamo_rieul(temp, ILLANG));
    }

    #[test]
    fn _when_when_last_jamo_nieun() {
        // 밭침이 없는 경우
        let temp = "철수";
        let result = "철순들";
        assert_eq!(result, when_last_jamo_nieun(temp, INDEUL));
        let temp = "아버지";
        let result = "아버진들";
        assert_eq!(result, when_last_jamo_nieun(temp, INDEUL));
        let temp = "나";
        let result = "난들";
        assert_eq!(result, when_last_jamo_nieun(temp, INDEUL));
        // 받침이 있는 경우
        let temp = "법원";
        let result = "법원인들";
        assert_eq!(result, when_last_jamo_nieun(temp, INDEUL));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "google인들";
        assert_eq!(result, when_last_jamo_nieun(temp, INDEUL));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "naver";
        let result = "naver인들";
        assert_eq!(result, when_last_jamo_nieun(temp, INDEUL));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "넥슨(코리아)인들";
        assert_eq!(result, when_last_jamo_nieun(temp, INDEUL));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "비타500인들";
        assert_eq!(result, when_last_jamo_nieun(temp, INDEUL));
        // ㄴ즉, 인즉 경우
        // 밭침이 없는 경우
        let temp = "철수";
        let result = "철순즉";
        assert_eq!(result, when_last_jamo_nieun(temp, INJEUK));
        let temp = "아버지";
        let result = "아버진즉";
        assert_eq!(result, when_last_jamo_nieun(temp, INJEUK));
        let temp = "나";
        let result = "난즉";
        assert_eq!(result, when_last_jamo_nieun(temp, INJEUK));
        // 받침이 있는 경우
        let temp = "법원";
        let result = "법원인즉";
        assert_eq!(result, when_last_jamo_nieun(temp, INJEUK));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "google인즉";
        assert_eq!(result, when_last_jamo_nieun(temp, INJEUK));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "naver";
        let result = "naver인즉";
        assert_eq!(result, when_last_jamo_nieun(temp, INJEUK));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "넥슨(코리아)인즉";
        assert_eq!(result, when_last_jamo_nieun(temp, INJEUK));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "비타500인즉";
        assert_eq!(result, when_last_jamo_nieun(temp, INJEUK));
    }

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
    #[test]
    fn _when_blank_ni() {
        // 마지막 받침이 있는 경우
        let temp = "떡";
        let result = "이니";
        assert_eq!(result, when_blank(temp, NI));
        let temp = "과일";
        let result = "이니";
        assert_eq!(result, when_blank(temp, NI));
        // 마지막 받침이 없는 경우
        let temp = "옥수수";
        let result = "니";
        assert_eq!(result, when_blank(temp, NI));
        let temp = "조";
        let result = "니";
        assert_eq!(result, when_blank(temp, NI));
        let temp = "고추";
        let result = "니";
        assert_eq!(result, when_blank(temp, NI));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "(이)니";
        assert_eq!(result, when_blank(temp, NI));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "이니";
        assert_eq!(result, when_blank(temp, NI));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "이니";
        assert_eq!(result, when_blank(temp, NI));
    }
    #[test]
    fn _when_blank_deun() {
        // 마지막 받침이 있는 경우
        let temp = "짐승";
        let result = "이든";
        assert_eq!(result, when_blank(temp, DEUN));
        let temp = "사람";
        let result = "이든";
        assert_eq!(result, when_blank(temp, DEUN));
        // 마지막 받침이 없는 경우
        let temp = "사과";
        let result = "든";
        assert_eq!(result, when_blank(temp, DEUN));
        let temp = "배";
        let result = "든";
        assert_eq!(result, when_blank(temp, DEUN));
        let temp = "고추";
        let result = "든";
        assert_eq!(result, when_blank(temp, DEUN));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "(이)든";
        assert_eq!(result, when_blank(temp, DEUN));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "이든";
        assert_eq!(result, when_blank(temp, DEUN));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "이든";
        assert_eq!(result, when_blank(temp, DEUN));
    }
    #[test]
    fn _when_blank_deunji() {
        // 마지막 받침이 있는 경우
        let temp = "일";
        let result = "이든지";
        assert_eq!(result, when_blank(temp, DEUNJI));
        let temp = "짐승";
        let result = "이든지";
        assert_eq!(result, when_blank(temp, DEUNJI));
        // 마지막 받침이 없는 경우
        let temp = "비행기";
        let result = "든지";
        assert_eq!(result, when_blank(temp, DEUNJI));
        let temp = "기차";
        let result = "든지";
        assert_eq!(result, when_blank(temp, DEUNJI));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "(이)든지";
        assert_eq!(result, when_blank(temp, DEUNJI));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "이든지";
        assert_eq!(result, when_blank(temp, DEUNJI));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "이든지";
        assert_eq!(result, when_blank(temp, DEUNJI));
    }
    #[test]
    fn _when_blank_deunka() {
        // 마지막 받침이 있는 경우
        let temp = "재작년";
        let result = "이든가";
        assert_eq!(result, when_blank(temp, DEUNKA));
        let temp = "작년";
        let result = "이든가";
        assert_eq!(result, when_blank(temp, DEUNKA));
        // 마지막 받침이 없는 경우
        let temp = "운전이라";
        let result = "든가";
        assert_eq!(result, when_blank(temp, DEUNKA));
        let temp = "뭐";
        let result = "든가";
        assert_eq!(result, when_blank(temp, DEUNKA));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "(이)든가";
        assert_eq!(result, when_blank(temp, DEUNKA));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "이든가";
        assert_eq!(result, when_blank(temp, DEUNKA));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "이든가";
        assert_eq!(result, when_blank(temp, DEUNKA));
    }
    #[test]
    fn _when_blank_yeo() {
        // 마지막 받침이 있는 경우
        let temp = "슬픔";
        let result = "이여";
        assert_eq!(result, when_blank(temp, YEO));
        let temp = "기쁨";
        let result = "이여";
        assert_eq!(result, when_blank(temp, YEO));
        // 마지막 받침이 없는 경우
        let temp = "겨레";
        let result = "여";
        assert_eq!(result, when_blank(temp, YEO));
        let temp = "나라";
        let result = "여";
        assert_eq!(result, when_blank(temp, YEO));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "(이)여";
        assert_eq!(result, when_blank(temp, YEO));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "이여";
        assert_eq!(result, when_blank(temp, YEO));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "이여";
        assert_eq!(result, when_blank(temp, YEO));
    }
    #[test]
    fn _when_blank_raya() {
        // 마지막 받침이 있는 경우
        let temp = "사람";
        let result = "이라야";
        assert_eq!(result, when_blank(temp, RAYA));
        let temp = "기쁨";
        let result = "이라야";
        assert_eq!(result, when_blank(temp, RAYA));
        // 마지막 받침이 없는 경우
        let temp = "저녁에";
        let result = "라야";
        assert_eq!(result, when_blank(temp, RAYA));
        let temp = "호랑이";
        let result = "라야";
        assert_eq!(result, when_blank(temp, RAYA));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "(이)라야";
        assert_eq!(result, when_blank(temp, RAYA));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "이라야";
        assert_eq!(result, when_blank(temp, RAYA));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "이라야";
        assert_eq!(result, when_blank(temp, RAYA));
    }
    #[test]
    fn _when_blank_wa() {
        // 마지막 받침이 있는 경우
        let temp = "얼음";
        let result = "과";
        assert_eq!(result, when_blank(temp, WA));
        let temp = "친구들";
        let result = "과";
        assert_eq!(result, when_blank(temp, WA));
        // 마지막 받침이 없는 경우
        let temp = "소";
        let result = "와";
        assert_eq!(result, when_blank(temp, WA));
        let temp = "친구";
        let result = "와";
        assert_eq!(result, when_blank(temp, WA));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "(와)과";
        assert_eq!(result, when_blank(temp, WA));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "과";
        assert_eq!(result, when_blank(temp, WA));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "과";
        assert_eq!(result, when_blank(temp, WA));
    }
    #[test]
    fn _when_blank_rado() {
        // 마지막 받침이 있는 경우
        let temp = "얼음";
        let result = "이라도";
        assert_eq!(result, when_blank(temp, RADO));
        let temp = "친구들";
        let result = "이라도";
        assert_eq!(result, when_blank(temp, RADO));
        // 마지막 받침이 없는 경우
        let temp = "소";
        let result = "라도";
        assert_eq!(result, when_blank(temp, RADO));
        let temp = "친구";
        let result = "라도";
        assert_eq!(result, when_blank(temp, RADO));
        // 마지막 글자가 영어가 나오는 경우
        let temp = "google";
        let result = "(이)라도";
        assert_eq!(result, when_blank(temp, RADO));
        // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
        let temp = "넥슨(코리아)";
        let result = "이라도";
        assert_eq!(result, when_blank(temp, RADO));
        // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
        let temp = "비타500";
        let result = "이라도";
        assert_eq!(result, when_blank(temp, RADO));
    }
}
