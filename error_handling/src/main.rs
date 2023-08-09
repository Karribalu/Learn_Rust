// use std::{fs::File, io::ErrorKind};
// fn main() {
//     let _file = File::open("hello.txt");
//     let greeting_file = match _file {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("error while creating the file {:?}", e),
//             },
//             other_error => {
//                 panic!("Problem while opening the file :{:?}", other_error);
//             }
//         },
//     };
//     println!("{:?}😃", greeting_file);
// }
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}