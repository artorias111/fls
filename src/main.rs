use std::path::Path;
use std::fs;


fn main() {
    let rna_path_str = "~/Documents"; // replace with
                                                                                   // clap stuff
    let rna_path = Path::new(rna_path_str);
    let paths = fs::read_dir(rna_path)
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
