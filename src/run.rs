use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "qcal", about = "Quick decimal, hexadecimal, binary, and octal calcs.")]
enum Config {
    #[structopt(name = "add", about="Adds numbers")]
    Add { numbers: Vec<isize> },
    #[structopt(name = "sub", about="Subtracts numbers")]
    Sub { numbers: Vec<isize> },
    #[structopt(name = "mul", about="Multiplies numbers")]
    Mul { numbers: Vec<isize> },
    #[structopt(name = "div", about="Divides numbers")]
    Div { numbers: Vec<isize> },
}

fn print_result(result: isize) {
    println!("dec: {}\t\thex: {:x}\t\toct: {:o}\t\tbin: {:b}", result, result, result, result);
}

fn calculate<F>(numbers: &mut Vec<isize>, func: F) 
    where F: std::ops::FnMut(isize, &isize) -> isize {
    let vec2 = numbers.split_off(1);
    print_result(vec2.iter().fold(numbers[0], func));
}


pub fn run() -> Result<(), ()> {
    match Config::from_args() {
        Config::Add { mut numbers } => calculate(&mut numbers, |acc, x| acc + x),
        Config::Sub { mut numbers } => calculate(&mut numbers, |acc, x| acc - x),
        Config::Mul { mut numbers } => calculate(&mut numbers, |acc, x| acc * x),
        Config::Div { mut numbers } => calculate(&mut numbers, |acc, x| acc / x),
    };
    Ok(())
}


