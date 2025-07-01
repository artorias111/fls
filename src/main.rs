use std::env;
use std::fs;


fn main() {
    let path = env::current_dir()
        .unwrap();
    let paths = fs::read_dir(path)
        .unwrap();

    for path in paths {
        let filepath = path
            .unwrap()
            .path();
        let filepath_str = filepath
            .display()
            .to_string();

         println!("{}", filepath_str);

   }
}
