fn print_result(result: isize) {
    println!("dec: {}\t\thex: {:x}\t\toct: {:o}\t\tbin: {:b}", result, result, result, result);
}

fn calculate<F>(args: &mut Vec<String>, func: F)
    where F: std::ops::FnMut(isize, &isize) -> isize {
        let mut var_buf: Vec<isize> = Vec::new();
        for mut num in args {
            var_buf.push(parse_numbers(&mut num));
        }
        let vec2 = var_buf.split_off(1);
        print_result(vec2.iter().fold(var_buf[0], func));
}

fn parse_numbers(number: &mut String) -> isize {
    if &number.len() > &1 {
        if &number[0..1] == "b" {
            &number.remove(0);
            return isize::from_str_radix(&number, 2).unwrap();
        } else if &number[0..1] == "o" {
            &number.remove(0);
            return isize::from_str_radix(&number, 8).unwrap();
        } else if &number[0..2] == "0x" {
            &number.remove(0);
            &number.remove(0);
            return isize::from_str_radix(&number, 16).unwrap();
        } else if &number[0..2] == "0b" {
            &number.remove(0);
            &number.remove(0);
            return isize::from_str_radix(&number, 2).unwrap();
        } else {
            return isize::from_str_radix(&number, 10).unwrap();
        }
    } else {
        return isize::from_str_radix(&number, 10).unwrap();
    }
}

pub fn run() -> Result<(), ()> {
    let mut args = std::env::args().collect::<Vec<String>>();
    args.remove(0);
    match args.remove(0).as_str() {
        "add"  => calculate(&mut args, |acc, x| acc + x),
        "sub"  => calculate(&mut args, |acc, x| acc - x),
        "mul"  => calculate(&mut args, |acc, x| acc * x),
        "div"  => calculate(&mut args, |acc, x| acc / x),
         _ => panic!("help me!"),
    };
    Ok(())
}


