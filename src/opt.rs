use crate::common::*;

#[derive(StructOpt)]
pub(crate) struct Opt {
    #[structopt(name = "FORMAT", long = "format")]
    _format: Operand,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    // Test argument parsing for `qcal format 4`
    fn cli_format() -> Result<(), structopt::clap::Error> {
        let dec_num = 4;
        let text = format!("{}", dec_num);
        let _opt = Opt::from_iter_safe(vec!["qcal", "--format", &text])?;

        Ok(())
    }
}
