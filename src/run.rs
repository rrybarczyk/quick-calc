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

fn swap_endianness(hex_str: &mut Vec<String>) {
    println!("{}", swap(hex_str));
}

fn swap(le_vec: &mut Vec<String>) -> String {
    let mut s = String::new();
    let mut tmp_c = 0 as char;
    for le in le_vec {
        let mut be_slice: Vec<char> = Vec::new();
        for (idx, c) in le.chars().rev().enumerate() {
            if idx % 2 != 0 {
                &be_slice.push(c);
                &be_slice.push(tmp_c);
            } else {
                tmp_c = c;
            }
        }
        s = be_slice.iter().collect();
    }
    s
}

pub fn run() -> Result<(), ()> {
    let mut args = std::env::args().collect::<Vec<String>>();
    args.remove(0);
    match args.remove(0).as_str() {
        "endian"   => swap_endianness(&mut args),
        "add"   => calculate(&mut args, |acc, x| acc + x),
        "sub"   => calculate(&mut args, |acc, x| acc - x),
        "mul"   => calculate(&mut args, |acc, x| acc * x),
        "div"   => calculate(&mut args, |acc, x| acc / x),
         _      => panic!("help me!"),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::run::*;
    #[test]
    fn test_swap_endianness() {
        let hex_0a = "000000002a22cfee1f2c846adbd12b3e183d4f97683f85dad08a79780a84bd55";
        let hex_0b = "55bd840a78798ad0da853f68974f3d183e2bd1db6a842c1feecf222a00000000";
        assert_eq!(swap(&mut vec![String::from(hex_0a)]), hex_0b);
        assert_eq!(swap(&mut vec![String::from(hex_0b)]), hex_0a);

        let hex_1a = "abcdef";
        let hex_1b = "efcdab";
        assert_eq!(swap(&mut vec![String::from(hex_1a)]), hex_1b);
        assert_eq!(swap(&mut vec![String::from(hex_1b)]), hex_1a);
    }
}
