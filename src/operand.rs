use crate::common::*;

pub(crate) struct Operand {
    value: isize,
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "dec:\t{}", self.value)
    }
}

impl FromStr for Operand {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let value = match isize::from_str_radix(&text, 10) {
            Ok(x) => x,
            Err(error) => {
                return Err(Error::NumParse {
                    num_parse: error,
                    bad_num: text.to_owned(),
                })
            }
        };

        Ok(Operand { value })
    }
}
