pub mod paste {
    use yaml_rust::YamlLoader;
    use yaml_rust::Yaml;
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
    pub fn get_config(name: &str) -> Vec<Yaml> {
        let config_str = read_file(name);
        let values = YamlLoader::load_from_str(&config_str[..]).unwrap();
        values

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
    #[test]
    fn test_get_config() {
        let data = paste::get_config("./example/config.yml");
        let conf = &data[0];
        assert_eq!(conf["output"].as_str().unwrap(), "shell.example.com:public_html/");
    }

}
