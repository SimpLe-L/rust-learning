pub type CalcResult<T> = Result<T, CalcError>;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum CalcError {
    #[error("非法字符: {0}")]
    UnexpectedCharacter(char),
    #[error("无效的运算符: {0}")]
    InvalidToken(String),
}
