/// solana-gadgets common resuable modules
/// Includes
///
use std::io::prelude::*;
use std::{fs::File, io};
use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;

pub fn load_yaml_file(yaml_file: &str) -> Result<Vec<Yaml>, io::Error> {
    let mut file = File::open(yaml_file)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    let docs = YamlLoader::load_from_str(&contents).unwrap();
    Ok(docs)
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::env;
    const SCLI: &str = "../../samples/yamldecls/SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.yml";
    const NONEXIST: &str = "../../samples/yamldecls/nothere.yml";

    fn print_indent(indent: usize) {
        for _ in 0..indent {
            print!("    ");
        }
    }

    fn dump_node(doc: &Yaml, indent: usize) {
        match *doc {
            Yaml::Array(ref v) => {
                for x in v {
                    dump_node(x, indent + 1);
                }
            }
            Yaml::Hash(ref h) => {
                for (k, v) in h {
                    print_indent(indent);
                    println!("{:?}:", k);
                    dump_node(v, indent + 1);
                }
            }
            _ => {
                print_indent(indent);
                println!("{:?}", doc);
            }
        }
    }

    #[test]
    fn load_yaml_file_pass() {
        let path = env::current_dir().unwrap();
        println!("The current directory is {}", path.display());
        let result = load_yaml_file(SCLI);
        assert!(result.is_ok());
        dump_node(&result.unwrap()[0], 0);
    }
    #[test]
    fn load_yaml_file_fail() {
        let y = load_yaml_file(NONEXIST);
        assert!(y.is_err());
    }
}
