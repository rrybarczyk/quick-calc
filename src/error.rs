use crate::common::*;

#[derive(Debug, PartialEq)]
pub(crate) enum Error {
    ParseInt {
        parse_int: ParseIntError,
        bad_int: String,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::ParseInt { parse_int, bad_int } => {
                write!(f, "Invalid operand input: `{}`, {}", bad_int, parse_int)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_parse_display() {
        let bad_input = "BAD_INPUT";
        let text = format!("{}", bad_input);
        let err = FmtOperands::from_str(&text).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Invalid operand input: `BAD_INPUT`, invalid digit found in string"
        );
    }
}
