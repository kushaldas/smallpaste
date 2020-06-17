pub mod paste {
    use rand::Rng;
    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::Error;
    use std::path::Path;
    use yaml_rust::Yaml;
    use yaml_rust::YamlLoader;
    use dirs;

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
    pub fn get_config_from_home() -> Vec<Yaml> {
        let home_path = dirs::home_dir().unwrap();
        let filepath = format!("{}/.smallpaste.yml", home_path.to_str().unwrap());
        get_config(&filepath)
    }
    pub fn create_local_path(public: bool) -> Result<String, Error> {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789_";
        const PATH_LEN: usize = 15;
        let full_path: String;
        if !public {
            let mut rng = rand::thread_rng();

            let random_path: String = (0..PATH_LEN)
                .map(|_| {
                    let idx = rng.gen_range(0, CHARSET.len());
                    CHARSET[idx] as char
                })
                .collect();
            full_path = format!("/tmp/volatile/{}", random_path);
        } else {
            full_path = "/tmp/volatile".to_string();
        }
        let _a = fs::remove_dir_all("/tmp/volatile");
        let _a = fs::create_dir_all(&full_path);
        Ok(full_path)
    }

    pub fn copy(target: String, files: Vec<String>) -> Vec<String>{
        let mut result = Vec::new();
        for file in files {
            let p = Path::new(&file);
            let dest = format!("{}/{}", target, p.file_name().unwrap().to_string_lossy());
            //println!("Now copying locally {} to {}", file, dest);
            fs::copy(file, &dest).unwrap();
            result.push(dest);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io;
    use std::path::PathBuf;

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
        assert_eq!(
            conf["output"].as_str().unwrap(),
            "shell.example.com:public_html/"
        );
    }
    #[test]
    fn test_create_and_copy_public() {
        let target = paste::create_local_path(true).unwrap();
        let files = vec!["./example/config.yml".to_string(), "Cargo.toml".to_string()];
        paste::copy(target, files);
        let mut entries = fs::read_dir("/tmp/volatile")
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()
            .unwrap();
        entries.sort();
        assert_eq!(
            entries,
            vec![
                PathBuf::from("/tmp/volatile/Cargo.toml"),
                PathBuf::from("/tmp/volatile/config.yml")
            ]
        );
    }
}
