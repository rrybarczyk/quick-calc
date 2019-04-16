use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "qcal", about = "Quick decimal, hexadecimal, binary, and octal Calcs")]
enum Config {
    #[structopt(name = "add")]
    Add { number: Vec<usize> },
    #[structopt(name = "sub")]
    Sub { number: Vec<usize> },
    #[structopt(name = "mul")]
    Mul { number: Vec<usize> },
    #[structopt(name = "div")]
    Div { number: Vec<usize> },
}

fn calc(result: usize) {
    println!("dec: {}\t\thex: {:x}\t\toct: {:o}\t\tbin: {:b}", result, result, result, result);
}

pub fn run() -> Result<(), ()> {
    match Config::from_args() {
        Config::Add { number } => calc(number.iter().fold(0, |acc, x| acc + x)),
        Config::Sub { number } => calc(number.iter().fold(0, |acc, x| acc - x)),
        Config::Mul { number } => calc(number.iter().fold(1, |acc, x| acc * x)),
        Config::Div { number } => calc( number[0] / number[1]),
    };
    Ok(())
}
