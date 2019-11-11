use crate::common::*;

#[derive(Debug)]
pub(crate) enum Error {
    NumParse {
        num_parse: ParseIntError,
        bad_num: String,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::NumParse { num_parse, bad_num } => {
                write!(f, "Invalid operand input: `{}`, {}", bad_num, num_parse)
            }
        }
    }
}
