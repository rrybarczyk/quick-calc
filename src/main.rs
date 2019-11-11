mod common;
mod error;
mod operand;
mod opt;

use crate::common::*;

fn main() {
    let opt = Opt::from_args();
    opt.print_calc();
}
