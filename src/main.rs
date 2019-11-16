mod common;
mod error;
mod fmtoperands;
mod opt;

use crate::common::*;

fn main() {
    let opt = Opt::from_args();
    opt.run_operations();
}
