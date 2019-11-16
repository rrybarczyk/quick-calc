use crate::common::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "qcal", about = "Semi-stacked based quick calculation tool")]
pub(crate) enum Opt {
    #[structopt(name = "format")]
    Format {
        #[structopt(takes_value = true, multiple = true)]
        operands: FmtOperands,
    },
}

impl Opt {
    /// Execute all operations and print formatted result
    pub fn run_operations(&self) {
        match &self {
            Opt::Format { operands } => {
                &operands.print_result();

                dbg!("ashldfjas: {:?}", &operands);
                println!("ashldfjas: {:?}", &operands);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// CLI accepts a single input
    fn cli_format_single() -> Result<(), structopt::clap::Error> {
        let _opt = Opt::from_iter_safe(vec!["qcal", "format", "4"])?;
        let _opt = Opt::from_iter_safe(vec!["qcal", "format", "x4"])?;
        let _opt = Opt::from_iter_safe(vec!["qcal", "format", "o4"])?;
        let _opt = Opt::from_iter_safe(vec!["qcal", "format", "b0100"])?;
        Ok(())
    }

    #[test]
    /// CLI accepts multiple decimal inputs
    fn cli_format_multiple() -> Result<(), structopt::clap::Error> {
        let _opt = Opt::from_iter_safe(vec!["qcal", "format", "4 5"])?;
        let _opt = Opt::from_iter_safe(vec!["qcal", "format", "x4 5"])?;
        let _opt = Opt::from_iter_safe(vec!["qcal", "format", "o4 5"])?;
        let _opt = Opt::from_iter_safe(vec!["qcal", "format", "b0100 b0101"])?;
        Ok(())
    }
}
