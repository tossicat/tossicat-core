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
                "The length has been exceeded. Set the word length to less than 50."
            ),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ValueError {
    error: ValueErrorType,
    description: String,
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

#[derive(Debug, PartialEq)]
pub enum ParseErrorType {
    InvalidParentheses,
    Value(ValueErrorType),
}

impl fmt::Display for ParseErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseErrorType::InvalidParentheses => write!(
                f,
                "The sentence can not be parsed. Please check the sentence has incorrect parentheses."
            ),
            ParseErrorType::Value(ref error) => write!(f,"{error}")
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseError {
    error: ParseErrorType,
    description: String,
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

#[derive(Debug)]
pub enum BracketErrorType {
    AreNotBalanced,
    PairsNums,
    SplitTossiWord,
}
