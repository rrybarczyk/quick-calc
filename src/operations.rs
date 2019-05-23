/// Print the cli args formatted as dec, hex, oct, and bin.
pub fn format_args(args: &mut Vec<String>) {
    for arg in args {
        let sarg = strip_prefix(arg);
        print_calc(isize::from_str_radix(&sarg.0, sarg.1).unwrap());
    }
}


/// Calculate the specified operation and print.
pub fn calculate_operation<F>(args: &mut Vec<String>, func: F)
    where F: std::ops::FnMut(isize, &isize) -> isize {

        let mut var_buf: Vec<isize> = Vec::new();
        for arg in args {
            let sarg = strip_prefix(arg);
            var_buf.push(isize::from_str_radix(&sarg.0, sarg.1).unwrap());
        }
        let vec2 = var_buf.split_off(1);
        print_calc(vec2.iter().fold(var_buf[0], func));
}

/// Swap the endianness of a hex string and print.
pub fn swap_endianness(args: &mut Vec<String>) {
    
    for arg in args {
        println!("0x{}", swap(&arg));
    }
}

/// Calculate the number of bytes or characters of a hex string and print.
pub fn calculate_length(args: &mut Vec<String>, is_bytes: bool) {
    for arg in args {
        if is_bytes {
            let arg_count = ((get_char_length(arg) as f64)/2.0).ceil() as usize;
            print_len(arg.to_string(), arg_count, is_bytes);
        } else {
            print_len(arg.to_string(), get_char_length(arg), is_bytes);
        }
    }
}

/// Removes the `x`, `o`, `b`, or `*x` prefix if it exists, and returns a
/// tuple of the number as a string and its base.
fn strip_prefix(s: &str) -> (&str, u32) {
    let len = s.len();
    let bytes = s.as_bytes();

    if bytes[0] == b'x'  || bytes[0] == b'X' {
        return (&s[1..len], 16);
    } else if bytes[0] == b'o' || bytes[0] == b'O' {
        return (&s[1..len], 8);
    } else if bytes[0] == b'b' || bytes[0] == b'B' {
        return (&s[1..len], 2);
    } else if len > 1 && (bytes[1] == b'x' || bytes[1] == b'X') {
        return (&s[2..len], 16);
    }
    (&s[..], 10)
}

/// Swap the endianness of a hex string.
fn swap(arg: &str) -> String {
    let mut tmp_c = 0 as char;
    let mut be_slice: Vec<char> = Vec::new();

    let sarg = strip_prefix(arg);

    for (idx, c) in sarg.0.chars().rev().enumerate() {
        if idx % 2 != 0 {
            be_slice.push(c);
            be_slice.push(tmp_c);
        } else {
            tmp_c = c;
        }
    }
    be_slice.iter().collect()
}

/// Calculate the number of characters in a hex string.
fn get_char_length(arg: &str) -> usize {
    strip_prefix(&arg).0.to_string().len()
}


/// Print the result formatted as dec, hex, oct, and bin.
fn print_calc(result: isize) {
    println!("dec: {}\t\thex: 0x{:x}\t\toct: o{:o}\t\tbin: b{:b}", result, result, result, result);
}

/// Print the number of bytes or characters of a hex string.
fn print_len(arg: String, len: usize, is_bytes: bool) {
    let len_type = if is_bytes { "byte" } else { "char" };

    if len == 1 || len == 0 {
        println!("{}:\t 1 {}", arg, len_type);
    } else {
        println!("{}:\t {} {}s", arg, len, len_type);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_char_length() {
        dbg!("needs better tests");
        let mut args = create_deadbeef_vector(true);
        calculate_length(&mut args, false);
        calculate_length(&mut args, true);
        assert!(true);
    }

    #[test]
    fn test_swap() {
        let hex0a = "000000002a22cfee1f2c846adbd12b3e183d4f97683f85dad08a79780a84bd55";
        let hex0b = "55bd840a78798ad0da853f68974f3d183e2bd1db6a842c1feecf222a00000000";
        assert_eq!(swap(&hex0a), hex0b);
        assert_eq!(swap(&hex0b), hex0a);

        let hex1a = "abcdef";
        let hex1b = "efcdab";
        assert_eq!(swap(&hex1a), hex1b);
        assert_eq!(swap(&hex1b), hex1a);
    }

    #[test]
    fn test_get_char_length() {
        let arg = String::from("0xdeadbeef");
        assert_eq!(get_char_length(&arg), 8);
    }

    #[test]
    fn test_strip_prefix() {
        let arg = String::from("0xdeadbeef");
        assert_eq!(strip_prefix(&arg).0, "deadbeef");
    }

    fn create_deadbeef_vector(prefix: bool) -> Vec<String> {
        let mut db_dec: String;
        let mut db_hex: String;
        let mut db_oct: String;
        let mut db_bin: String;

        if prefix {
            db_dec = String::from("3735928559");
            db_hex = String::from("0xdeadbeef");
            db_oct = String::from("o33653337357");
            db_bin = String::from("b11011110101011011011111011101111");
        } else {
            db_dec = String::from("3735928559");
            db_hex = String::from("deadbeef");
            db_oct = String::from("33653337357");
            db_bin = String::from("11011110101011011011111011101111");
        }
        vec![db_dec, db_hex, db_oct, db_bin]
    }
}
