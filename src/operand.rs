use crate::common::*;

pub(crate) struct Operand {
    _value: isize,
}

impl FromStr for Operand {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let _value = match isize::from_str_radix(&text, 10) {
            Ok(x) => x,
            Err(error) => {
                return Err(Error::NumParse {
                    num_parse: error,
                    bad_num: text.to_owned(),
                })
            }
        };

        Ok(Operand { _value })
    }
}
