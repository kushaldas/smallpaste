pub mod paste {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    pub fn read_file(name: &str) -> String {
        let path = Path::new(name);
        let mut file = match File::open(&path) {
            Err(why) => panic!("Error in opening {:?} {}", path, why),
            Ok(file) => file,
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let testdata = "output: \"shell.example.com:public_html/\"\nurl_prefix: \"https://example.com/volatile/\"\n\n";
        let data = paste::read_file("./example/config.yml");
        assert_eq!(data, testdata);
    }
}
