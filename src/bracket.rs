//! # 괄호를 다루는 모듈
//!
//! 이 모듈은 다양한 괄호('{' | '[' | '(' )를 가 들어 있는 문자열에서
//! 입력된 문자열에 들어 있는 괄호들이 올바른 짝이 있는지를 파악하고
//! 이렇게 파악된 결과에 따라 올바른 괄호 쌍의 위치를 알려주는 역할을 한다.
//!
//! 이 라이브러리에서는 "{철수,와} {밥,를} 먹습니다." 과 같은 것을 분석하려고 하기 때문에
//! 괄호 안의 괄호가 들어 있는 즉 중첩된 괄호도 분석할 필요가 없습니다. 
//! 따라서 만약 괄호 안의 괄호가 들어 있는 문자열이 들어오면 처리하지 않고,
//! false를 반환하게 됩니다.
//! 

pub fn find_pairs(string: &str){
    let temp = are_balanced(string);

    if temp.0 {
        let mut open_brackets: Vec<usize> = vec![];
        let mut close_brackets: Vec<usize> = vec![];
        for item in temp.1 {
            // 아래 `match`는 중괄호, "{}"만 있는지 확인하기 위한 것입니다.
            match item.2 {
                '{' | '}' => {
                    if item.1 != 1 {
                        return println!("in No 1 deep");
                    } else if item.2 == '{' {
                        open_brackets.push(item.0);
                    } else {
                        close_brackets.push(item.0);
                    }
                }
                _ => return println!("in No WFF!"),
            }
        }
        println!("{:?}", open_brackets);
        println!("{:?}", close_brackets);
    } else {
        println!("No WFF!");
    }
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

/// ## 입력된 괄호가 짝이 올바른지 검사하고 숫자인지 아닌지 확인하는 함수
/// 
/// 첫 번째 값은 우선 짝이 맞지 않으면, `true`, 아니면 `false`를 반환합니다.
/// 두 번째 값은 괄호의 정보를 저장한 튜플로 구성된 `Vec`입니다.
/// 만약 첫 번째 반환 값인 `bool` 값이 `false`이 될 것이며,
/// `Vec` 값의 원소는 빈 껍데기를 반환하는 것이 아니라
/// 짝이 올바른지 파악할 때까지 분석한 것이 들어있게 됩니다.
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
}
