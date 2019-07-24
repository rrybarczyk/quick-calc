use crate::common::*;

#[derive(StructOpt, Debug)]
#[cfg_attr(test, structopt(raw(setting = "AppSettings::ColorNever")))]
#[structopt(name = "qcal", about = "Semi-stacked based quick calculation tool")]
pub(crate) enum Opt {

    #[structopt(name = "format")]
    Format { 
        #[structopt(
            raw(takes_value = "true"),
            raw(multiple = "true"),
            parse(try_from_str="parse_format")
            )]
        inputs: Vec<usize>,
    },

    #[structopt(name = "add")]
    Add { 
        #[structopt(
            raw(takes_value = "true"),
            raw(multiple = "true"),
            parse(try_from_str="parse_format")
            )]
        inputs: Vec<usize>,
    }
}

fn parse_format(s: &str) -> Result<usize, std::num::ParseIntError> {
    s.to_string().parse::<usize>()
}

impl Opt {
    fn print_calc(&self) {
        match &self {
            Opt::Format { inputs } => Opt::print_result(inputs),
            Opt::Add { inputs } => println!("ADD"),

        }
    }

    fn print_result(inputs: &Vec<usize>) {
        inputs
            .iter()
            .map(|x| {
                println!("dec: {}\t\thex: {:x}\t\toct: o{:o}\t\tbin: b{:b}", *x, *x, *x, *x);
                *x
            }).for_each(drop);
    }
}

pub(crate) fn run() {
    Opt::from_args().print_calc();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_format() -> Result<(), structopt::clap::Error> {
        let _opt = Opt::from_iter_safe(vec!["qcal", "format", "4"])?;
        Ok(())
    }

    #[test]
    fn cli_format_multiple() -> Result<(), structopt::clap::Error> {
        let _opt = Opt::from_iter_safe(vec!["qcal", "format", "7", "5"])?;
        Ok(())
    }
}
