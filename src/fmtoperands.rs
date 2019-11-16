use crate::common::*;

#[derive(Debug, PartialEq)]
pub(crate) struct FmtOperands {
    input: Vec<isize>,
}

impl FromStr for FmtOperands {
    type Err = Error;
    /// Receives cli args as `&str` and returns Self
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        // `&str` to `Vec<String>`
        // let args: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
        let args: Vec<&str> = text.split_whitespace().collect();

        // `Vec<String>` to `Vec<isize>`
        let mut input = Vec::new();
        for arg in args {
            let sarg = FmtOperands::strip_prefix(arg);
            // input.push(isize::from_str_radix(&sarg.0, sarg.1).unwrap());
            match isize::from_str_radix(&sarg.0, sarg.1) {
                Ok(x) => input.push(x),
                Err(error) => {
                    return Err(Error::ParseInt {
                        parse_int: error,
                        bad_int: arg.to_string(),
                    })
                }
            };
        }
        Ok(FmtOperands { input })
    }
}

impl FmtOperands {
    pub fn print_result(&self) -> Result<(), Error> {
        self.input
            .iter()
            .map(|x| {
                println!(
                    "dec: {}\t\thex: {:x}\t\toct: o{:o}\t\tbin: b{:b}",
                    *x, *x, *x, *x
                );
                *x
            })
            .for_each(drop);

        Ok(())
    }

    /// Remove the `x`, `o`, `b`, or `*x` prefix if it exists, and returns a
    /// tuple of the number as a string and its base.
    fn strip_prefix(s: &str) -> (&str, u32) {
        let len = s.len();
        let bytes = s.as_bytes();

        if bytes[0] == b'x' || bytes[0] == b'X' {
            return (&s[1..len], 16);
        } else if bytes[0] == b'o' || bytes[0] == b'O' {
            return (&s[1..len], 8);
        } else if bytes[0] == b'b' || bytes[0] == b'B' {
            return (&s[1..len], 2);
        } else if len > 1 && (bytes[1] == b'x' || bytes[1] == b'X') {
            return (&s[2..len], 16);
        }
        (&s[..], 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operands_from_str() -> Result<(), Error> {
        let mut vec = Vec::new();
        vec.push(4);
        vec.push(5);
        let want = FmtOperands { input: vec };

        let text = format!("4 5");
        let have = FmtOperands::from_str(&text)?;

        assert_eq!(have, want);

        Ok(())
    }

    #[test]
    fn print_executes() -> Result<(), Error> {
        let mut vec = Vec::new();
        vec.push(4);
        vec.push(5);
        let oper = FmtOperands { input: vec };

        assert_eq!(Ok(()), oper.print_result());

        Ok(())
    }
}
