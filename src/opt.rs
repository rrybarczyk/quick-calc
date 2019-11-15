use crate::common::*;

#[derive(StructOpt)]
#[structopt(name = "qcal", about = "Semi-stacked based quick calculation tool")]
pub(crate) enum Opt {
    #[structopt(name = "format")]
    Format {
        #[structopt(multiple = true)]
        inputs: Operand,
    },
}

impl Opt {
    pub(crate) fn print_calc(&self) {
        match &self {
            Opt::Format { inputs } => println!("{}", inputs),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    // Test argument parsing for `qcal format 4`
    fn cli_format() -> Result<(), structopt::clap::Error> {
        let dec_num = 4;
        let text = format!("{}", dec_num);
        let _opt = Opt::from_iter_safe(vec!["qcal", "format", &text])?;

        Ok(())
    }
}
