use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ValueErrorType {
    InvalidTossi,
    LimitLength,
}

impl fmt::Display for ValueErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueErrorType::InvalidTossi => write!(f, "This value is not correct tossi"),
            ValueErrorType::LimitLength => write!(
                f,
                "The length has been exceeded. Set the word length to less than 50"
            ),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ValueError {
    pub error: ValueErrorType,
    pub description: String,
}

impl ValueError {
    pub fn new(error: ValueErrorType) -> Self {
        ValueError {
            description: error.to_string(),
            error,
        }
    }
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

/// SentenceType 의 종류
///
/// - AreNotBalanced : 괄호 짝이 안 맞습니다.
/// - IsNotBrace : "{,}" 과 같은 중괄호가 아닙니다.
/// - NestedParentheses : 중복 중괄호가 있습니다. 여기는 1단계만 처리합니다.
/// - SplitTossiWord : 입력된 것에서 토시와 단어로 분리할 수 없습니다.
/// - TossiIsEmpty : 입력한 문장에 들어 있는 중괄호에 토시가 없습니다.
/// - WordIsEmpty : 입력한 문장에 들어 있는 중괄호에 단어가 없습니다.
#[derive(Debug, PartialEq)]
pub enum ParseErrorType {
    InvalidValue(ValueErrorType),
    AreNotBalanced,
    IsNotBrace,
    NestedParentheses,
    SplitTossiWord,
    TossiIsEmpty,
    WordIsEmpty,
}

impl fmt::Display for ParseErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseErrorType::InvalidValue(_value_error) => write!(f, "{_value_error}."),
            ParseErrorType::AreNotBalanced => write!(f, "The sentence can not be parsed. Please check the sentence has incorrect parentheses."),
            ParseErrorType::IsNotBrace => write!(f, "The sentence can not be parsed. Please set the parentheses as a brace."),
            ParseErrorType::NestedParentheses => write!(f, "The sentence includes Nested Parentheses."),
            ParseErrorType::SplitTossiWord => write!(f, "The sentence can not be parsed. Please separate words and tossi with a comma."),
            ParseErrorType::TossiIsEmpty => write!(f, "The sentence can not be parsed. Please fill the tossi section in the parentheses."),
            ParseErrorType::WordIsEmpty => write!(f, "The sentence can not be parsed. Please fill the word section in the parentheses."),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub error: ParseErrorType,
    pub description: String,
}

impl ParseError {
    pub fn new(error: ParseErrorType) -> Self {
        ParseError {
            description: error.to_string(),
            error,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}
