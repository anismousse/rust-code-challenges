// fn info< T:std::fmt::Display>(a: &T) {
//     println!("{a}")
// }

// fn info<T:ToString>(a: &T) {
//     println!("{}", a.to_string())
// }

use std::ffi::OsStr;

fn info<T: AsRef<OsStr>>(a: &T) {
    println!("{}", a.as_ref().to_string_lossy())
}

// trait InfoData{
//     fn info(&self);
// }

// impl InfoData for String{
//     fn info(&self){
//         print!("{}", self)
//     }
// }

fn main() {
    let a = "?";
    let b = "?".to_string();
    info(&a);
    info(&b);

    // Advanced 1
    use std::ffi::CString;
    
    let c = CString::new("?").unwrap();
    info(&c);

    // Advanced 2
    // use std::path::Path;
    // let d = Path::new("/tmp/linkedin-learning");
    // info(d);
}


#[test]
fn str() {
    let input = "Rust";
    info(&input);
}

#[test]
fn string() {
    let input = String::from("Rust");
    info(&input);
}

// #[test]
// fn chars() {
//     let input = 'r';
//     info(&input);
// }

// #[test]
// fn cstring() {
//     use std::ffi::{CString};
//     let input = CString::new("Rust").unwrap();
//     info(&input);
// }

// #[test]
// fn path() {
//     use std::path::Path;
//     let input = Path::new("/tmp/rust");
//     info(input);
// }
