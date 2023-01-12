//! # 괄호에 들어 있는 단어와 토시를 다루는 모듈
//!
//! 이 모듈은 다양한 괄호('{' | '[' | '(' )에 `,`로 분리되어 들어 있는
//! 단어와 문자열을 뽑아내는 역할을 합니다.
//! 이 일을 하는 이유는 단어 하나, 토시 하나로 된 것만 분석하는 것이 아니라,
//! 특정 문장에 이런 쌍이 많이 들어 있는 문장이 입력될 때 이를 이 라이브러리 안에서
//! 한 번에 처리할 수 있게 하고 싶기 때문입니다.
//!
//! ## 에러 처리 기준
//!
//! 이 라이브러리에서는 "{철수,와} {밥,를} 먹습니다." 과 같은 것을 분석하려고 하기 때문에
//! 괄호 안의 괄호가 들어 있는 즉 중첩된 괄호도 분석할 필요가 없습니다.
//! 따라서 만약 괄호 안의 괄호가 들어 있는 문자열이 들어오면 처리하지 않고,
//! false를 반환하게 됩니다.

use crate::error::SentenceType;

/// ## 입력된 문장 안의 여러 개의 단어와 토시 쌍을 뽑아내서 적절한 토시로 변경하는 함수
///
/// 이 `bracket` 모듈에서 최종 함수입니다. 이 함수 아래에 있는 다음 함수들을 사용하고 있습니다.
///
/// - `are_balanced()`
/// - `find_pairs_nums()`
/// - `split_tossi_word()`
///
/// 각각의 함수에 대한 자세한 내용은 해당 함수의 설명을 참고하세요. 대락적인 소개를 하면
/// 다음과 같습니다. 참고로 이 함수들은 입력된 문장이 변환하기에 올바른 것인지 아닌지도
/// 평가합니다. 부적합한 것이라면 3 함수 모두 `false` 값을 반환합니다.
/// 우선 `are_balanced()` 함수로 괄호 쌍이 맞는지 확인하면서 괼호 쌍을 분석해서
/// 어떤 위치에 있는 괄호가 어떤 괄호와 쌍인지 반환합니다. 이렇게 반환한 값을 토대로  
/// `find_pairs_nums()` 함수는 중접되지 않는 중괄호를 순서대로 괄호가 열리는 위치와
/// 닫히는 위치를 반환합니다. 이때 중괄호가 아니면, 중접된 괄호가 있으면 정지합니다.
/// 마지막으로 `split_tossi_word()`은 중괄호 안에 있는 내용을 순서대로 뽑아내서
/// 단어와 이 단어에 붙일 토시를 분리하고, `("철수, 은", "철수", "은")`과 같이
/// 첫번째 값은 원본, 두 번째 값은 단어, 그리고 세 번째 값은 토시로 반환합니다.

pub fn modify_pairs(string: &str) -> Result<Vec<(String, String, String)>, SentenceType> {
    let mut temp_result: Vec<(String, String, String)> = vec![];
    let content = are_balanced(string);
    // println!("are_balanced: {:?}: ", content);
    if !content.0 {
        Err(SentenceType::AreNotBalanced)
    } else {
        let content = find_pairs_nums(content.1);
        if !content.0 {
            Err(SentenceType::NestedParentheses)
        } else if !content.1 {
            Err(SentenceType::IsNotBrace)
        } else {
            // println!("find_pairs_nums: {:?}, {:?}", content.1, content.1.len());
            // println!("find_pairs_nums: {:?}", content);
            for item in 0..content.2.len() {
                let temp = split_tossi_word(string, content.2[item].open, content.2[item].close);
                // println!("temp: {:?}", temp);
                // `split_tossi_word()`가 실패하면 `temp.0` 가 `false`가 됩니다.
                if !temp.0 {
                    return Err(SentenceType::SplitTossiWord);
                // 단어가 비어 있다면 Err 처리
                } else if temp.2 .0.is_empty() {
                    return Err(SentenceType::WordIsEmpty);
                // 토시가 비어 있다면 Err 처리
                } else if temp.2 .1.is_empty() {
                    return Err(SentenceType::TossiIsEmpty);
                }
                temp_result.push((temp.1, temp.2 .0, temp.2 .1));
                // if !temp.0 {
                //     return Err(SentenceType::SplitTossiWord);
                // }
            }
            Ok(temp_result)
        }
    }
}

/// ## 중 괄호로 쌓여 있는 문자를 뽑아내서 쉽표로 두 개로 분리해 반환하는 함수
///
/// `find_pairs_nums()` 함수로 처리되어 문제가 없는, 즉 `true`과 반환된
/// `[BracketPair { open: 0, close: 6 }]`과 같은 요소로 구성된`Vec`를 처리하는
/// 함수입니다. 이것들 가지고 입력된 `string`에서 중괄호, 즉 `{, }`로 쌓여 있는 문자열을
/// 뽑아낸 다음 그 문장이 쉼표, 즉 `,`로 하나로 되어 있는지 확인합니다. 만약 2개 이상이면
/// `false`를 반환하면서 분석을 멈추게 됩니다.
/// 만약 1개이면 앞뒤로 문자열을 쪼갠 다음, 이 2개의 문자열의 앞 뒤에 있는 공백을 제거한 다음
/// 이 두 쌍의 문자열과 원본 문자열, 그리고 분석을 성공했기 때문에 `true`를 반환합니다.

fn split_tossi_word(
    string: &str,
    start_num: usize,
    end_num: usize,
) -> (bool, String, (String, String)) {
    let temp = string.chars().collect::<Vec<_>>();
    let temp_splited = temp[start_num + 1..end_num]
        .iter()
        .cloned()
        .collect::<String>();
    let temp: Vec<&str> = temp_splited.split(',').collect();
    let mut results: (String, String) = (" ".to_owned(), " ".to_owned());
    if temp.len() != 2 {
        return (false, temp_splited, results);
    } else {
        let mut i = 0;
        for item in temp {
            let temp = item.trim().replace(' ', "");
            if i == 0 {
                results.0 = temp;
                i += 1;
            } else {
                results.1 = temp;
                i -= 1;
            }
        }
    }
    (true, temp_splited, results)
}

// 테스트 코드 작성을 위해 `PartialEq` 키워드 추가
#[derive(Debug, PartialEq)]
struct BracketPair {
    open: usize,
    close: usize,
}

/// ## 괄호 짝이 올바른 문장에서 올바른 짝의 열린 괄호와 닫힌 괄호의 숫자를 반환하는 함수   
///
/// 이 함수를 사용하기 전에 알아야 할 점을 살펴봅시다. 이 함수를 사용하려면, 사용할 문장이
/// `are_balanced()` 함수를 통해서 처리된 것만을 사용할 수 있습니다.
/// `are_balanced()` 함수는 입력된 문장에 들어 있는 괄호의 짝이 올바른 짝으로 구성되어 있다면,
/// `true`를, 구성되어 있지 않다면 `false`를 반환합니다.  
/// 그러나 현재 설명하고 있는 `find_pairs_nums()` 함수는 `true`을 반환한 것의 자료,
/// 즉 짝이 올바른 것들로만 `Vec`가 들어 온다고 가정할 것입니다.
/// 따라서 이 함수를 쓰기 전에 `are_balanced()` 함수를 통해  `false`를 받는 문장은
/// 예러 처리해야 합니다.
///
/// 이 `find_pairs_nums()` 함수는 `[(0, 1, '{'), (6, 1, '}')]`과 같은 `Vec`를
/// 받습니다. 각 요소에 대한 설명은 `are_balanced()`를 참고하세요. 이 `Vec` 안에 들어 있는
/// 값은 단지 해당 괄호 쌍이 어디부터 어디인지 알려주지 않습니다. 이 함수가 그것을 알려주는 역할을
/// 합니다. 예를 들어 앞의 `Vec`는 `BracketPair { open: 0, close: 6 }`과 같이
/// 처리합니다. 이는 첫번째 괄호 짝이 분석할 문자열의 0번째부터 시작해 6번째에 끝난다는 것을
/// 알려주는 것입니다. 이렇게 이 함수는 여러개의 올바른 괄호를 시작 번호와 끝나는 번호를
/// 반환해주는 역할을 합니다.
///
/// 그렇게 하면서 2가지는 제한합니다.
///
/// 1. 우선 괄호는 중괄호, `{,}`만을 다룹니다. 그래서 다른 형식의 괄호가 발견된다면,
/// `true, false`를 반환하고 실행을 끝냅니다.
///
/// 2. 중첩된 괄호를 처리하지 않습니다. 이 라이브러리 특성상 처리하고자 하는 것은 `{철수, 은}`과 같은
/// 것입니다. 따라서 중첩된 괄호를 분석할 필요가 없습니다. 왜냐하면 중괄호에 들어 있는 2개의 요소를 가지고
/// 분석하는 것이 이 라이브러리의 목표이기 때문입니다. 따라서 중첩된 괄호가 발견된다면,
/// `(false, true,`를 반환하고 실행을 끝냅니다.

fn find_pairs_nums(temp_vec: Vec<(usize, i32, char)>) -> (bool, bool, Vec<BracketPair>) {
    let mut brackets: Vec<BracketPair> = vec![];
    let mut temp_open = 0;
    for item in temp_vec {
        // 아래 if 는 중괄호, "{}"만 있는지 확인하기 위한 것입니다.
        if let '{' | '}' = item.2 {
            // 아래 `if`문은 깊이가 1단계 인지 아닌지 확인하는 것입니다.
            // 코드를 진행하다가 1단계가 이상이 있으면 바로 정지합니다.
            if item.1 != 1 {
                // println!("in No 1 deep");
                return (false, true, brackets);
            // 앞에서 1단계
            } else if item.2 == '{' {
                temp_open = item.0;
            } else {
                brackets.push(BracketPair {
                    open: temp_open,
                    close: item.0,
                });
            }
        } else {
            // 즉 중괄호 아닌 괄호가 있는 경우에 여기로 옵니다.
            // println!("in No WFF!");
            return (true, false, brackets);
        }
    }
    (true, true, brackets)
}

enum Bracket {
    Open(char),
    Close(char),
}

impl Bracket {
    pub fn new(c: char) -> Option<Bracket> {
        match c {
            '{' | '[' | '(' => Some(Bracket::Open(c)),
            '}' => Some(Bracket::Close('{')),
            ']' => Some(Bracket::Close('[')),
            ')' => Some(Bracket::Close('(')),
            _ => None,
        }
    }
}

/// ## 입력된 괄호들의 짝이 올바른지 검사하고 숫자인지 아닌지 확인하는 함수
///
/// 입력된 문장 안에 있는 괄호들의 짝이 모두 올바르면,
/// 첫 번째 반환 값은 `true`, 아니면 `false`이 됩니다.
/// 따라서 이 값이 `false`이면, 입력된 문장을 처리할 필요가 없습니다.
/// 곧장 에러 처리하면 됩니다.
/// 두 번째 반환값은 괄호의 정보를 저장한 튜플로 구성된 `Vec`입니다.
/// 주의할 점은 첫 번째 반환 값인 `bool` 값이 `false`이더라도
/// `Vec` 값의 원소는 빈 껍데기를 반환하는 것이 아니라
/// 짝이 올바른지 파악할 때까지 분석한 것이 들어있게 됩니다.
/// 왜냐하면 입력된 문장 전체를 분석하기 전에는 이 문장에 들어있는 괄호들이 올바른 짝을
/// 구성하고 있는지 결정할 수 없기 때문입니다.
///
/// 아래 `Vec`의 구성 요소인 튜플의 2번째 값은 괄호 안에 괄호가 들어 있는 경우를
/// 분석한 것입니다. 수학에서처럼 안에 들어 있는 괄호를 해결해야 밖에 있는 괄호를
/// 처리할 수 있기 때문에 안에 들어 있는 괄호가 우선 순위가 높게 되는 것입니다.
///
/// `Vec`의 구성 요소인 튜플의 각 원소 값은 다음과 같습니다.
/// - 첫 번째: 해당 괄호의 위치 숫자
/// - 두 번째: 해당 괄호의 우선 순위?
/// - 세 번째: 해당 괄호를 `char`로 저장

fn are_balanced(string: &str) -> (bool, Vec<(usize, i32, char)>) {
    let mut brackets: Vec<Bracket> = vec![];
    let mut brackets_list: Vec<(usize, i32, char)> = vec![];
    let mut i = 1;

    for (j, c) in string.chars().enumerate() {
        match Bracket::new(c) {
            Some(Bracket::Open(char_bracket)) => {
                brackets.push(Bracket::Open(char_bracket));
                brackets_list.push((j, i, c));
                i += 1;
            }
            Some(Bracket::Close(char_close_bracket)) => {
                i -= 1;
                brackets_list.push((j, i, c));
                match brackets.pop() {
                    Some(Bracket::Open(char_open_bracket)) => {
                        if char_close_bracket != char_open_bracket {
                            return (false, brackets_list);
                        }
                    }
                    _ => return (false, brackets_list),
                }
            }
            _ => (),
        };
    }
    (brackets.is_empty(), brackets_list)
}

/// 비 공개 함수 테스트
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _are_balanced() {
        // 짝이 맞는 경우
        let temp = "(((185 + 223.85) * 15) - 543)/2";
        let v = vec![
            (0, 1, '('),
            (1, 2, '('),
            (2, 3, '('),
            (15, 3, ')'),
            (21, 2, ')'),
            (28, 1, ')'),
        ];
        let result = (true, v);
        assert_eq!(result, are_balanced(temp));
        // 괄호 짝이 맞지 않는 경우 입니다.
        // 당연히 반환값 중 첫 번째 값이 `false`가 되지만,
        // `false`를 판정할때까지 분석한 내용이 들어 `Vec`는 반환합니다.
        // 자세한 내용은 앞의 `are_balanced()` 설명을 참고하세요.
        let temp = "{[]";
        let v = vec![(0, 1, '{'), (1, 2, '['), (2, 2, ']')];
        let result = (false, v);
        assert_eq!(result, are_balanced(temp));
    }

    #[test]
    fn _find_pairs_nums() {
        // `find_pairs_nums()` 설명에서도 언급한 것처럼 이 함수는 짝이 올바른 것만
        // 들어온다고 가정하고 있습니다.
        // 자세한 내용은 앞의 `find_pairs_nums()` 설명을 참고하세요.
        // 아래 테스트 코드는 짝이 맞는 경우 입니다. 참고로 아래 `temp` 는
        // 다음 코드에서 나온 것입니다.
        // `let test = "{철수, 은} {영희,   과} {밥,  를} 먹습니다.";
        // 괄호 깊이가 1단계이고 모두 같은 중괄호, `{,} 이기 때문에 적절하게
        // 적절하게 처리할 수 있습니다.
        let temp: Vec<(usize, i32, char)> = vec![
            (0, 1, '{'),
            (6, 1, '}'),
            (8, 1, '{'),
            (16, 1, '}'),
            (18, 1, '{'),
            (24, 1, '}'),
        ];
        let result = (
            true,
            true,
            vec![
                BracketPair { open: 0, close: 6 },
                BracketPair { open: 8, close: 16 },
                BracketPair {
                    open: 18,
                    close: 24,
                },
            ],
        );
        assert_eq!(result, find_pairs_nums(temp));
        // 아래 테스트 코드는 당연히 중괄호가 아닌 대괄호가 있기 때문에 첫 번째 값이 `false`가 되지만,
        // `false`를 판정할때까지 분석한 내용이 들어 있는 `Vec`는 반환합니다.
        // 그래서 첫 번째 중 괄호만 분석해서 반환합니다.
        let temp: Vec<(usize, i32, char)> =
            vec![(0, 1, '{'), (6, 1, '}'), (8, 1, '['), (16, 1, ']')];
        let result = (true, false, vec![BracketPair { open: 0, close: 6 }]);
        assert_eq!(result, find_pairs_nums(temp));
        // 괄호 짝이 맞지 않는 경우 입니다.
        // 원래는 이 함수에 도달할 수 없지만, 테스트 파일에 넣어 봤습니다.
        // 당연히 반환값 중 첫 번ㅉ 값은 `false`이 나옵니다.
        // `false`를 판정할때까지 분석한 내용이 들어 있는 `Vec`는 반환해야 하지만,
        // 하나 밖에 없는 괄호의 짝이 없기 때문에 두 번째 값은 빈 `Vec`만 반환하게 됩니다.
        let temp = vec![(0, 1, '{'), (1, 2, '['), (2, 2, ']')];
        let result = (false, false, vec![]);
        assert_eq!(result, find_pairs_nums(temp));
    }
}
