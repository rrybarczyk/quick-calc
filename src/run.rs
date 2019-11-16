// use crate::operations;
//
// pub fn run() -> Result<(), ()> {
//     // Define valid cli commands
//     let commands = ["format", "add", "sub", "mul", "div", "endian", "len", "bytelen", "charlen"];
//
//     // Grab useful cli args
//     let mut args = std::env::args().collect::<Vec<String>>();
//     args.remove(0);
//
//     // If no arguments are given, panic.
//     if args.is_empty() {
//         panic!("Give me some numbers to format");
//     }
//
//     // If an invliad command is given, panic.
//     if !args.iter().any(|x| commands.contains(&x.as_str())) {
//         panic!("Give me a valid command");
//     }
//
//     // If a valid command but no arguments are given, panic.
//     if args.iter().any(|x| commands.contains(&x.as_str())) && args.len() == 1 {
//         panic!("Give me some numbers to format");
//     }
//
//     match args.remove(0).as_str() {
//         "format"    => operations::format_args(&mut args),
//         "add"       => operations::calculate_operation(&mut args, |acc, x| acc + x),
//         "sub"       => operations::calculate_operation(&mut args, |acc, x| acc - x),
//         "mul"       => operations::calculate_operation(&mut args, |acc, x| acc * x),
//         "div"       => operations::calculate_operation(&mut args, |acc, x| acc / x),
//         "len" | "bytelen"   => operations::calculate_length(&mut args, true),
//         "charlen"   => operations::calculate_length(&mut args, false),
//         "endian"    => operations::swap_endianness(&mut args),
//         _           => panic!("Give me a valid command"),
//     };
//     Ok(())
// }
