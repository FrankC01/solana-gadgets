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

    fn print_indent(indent: usize) {
        for _ in 0..indent {
            print!("    ");
        }
    }

    fn walk_node(doc: &Yaml, indent: usize) {
        match *doc {
            Yaml::Array(ref v) => {
                for x in v {
                    walk_node(x, indent + 1);
                }
            }
            Yaml::Hash(ref h) => {
                for (k, v) in h {
                    print_indent(indent);
                    println!("{:?}:", k);
                    walk_node(v, indent + 1);
                }
            }
            _ => {
                print_indent(indent);
                println!("{:?}", doc);
            }
        }
    }

    fn walk_enum(doc: &Yaml, indent: usize) {
        match *doc {
            Yaml::Real(ref value) => {
                print_indent(indent);
                println!("Real {}", value)
            }
            Yaml::Integer(ref value) => {
                print_indent(indent);
                println!("Integer {}", value)
            }
            Yaml::String(ref value) => {
                print_indent(indent);
                println!("String {}", value)
            }
            Yaml::Boolean(ref value) => {
                print_indent(indent);
                println!("Boolean {}", value)
            }
            Yaml::Array(ref value) => {
                for x in value {
                    walk_enum(x, indent + 1);
                }
            }
            Yaml::Hash(ref value) => {
                for (k, v) in value {
                    print_indent(indent);
                    if let Yaml::String(value) = k {
                        print!("{}:", value);
                    }
                    if let Yaml::Array(_) = v {
                        println!();
                        walk_enum(v, indent + 1);
                    } else if let Yaml::Hash(_) = v {
                        walk_enum(v, indent + 1);
                    } else {
                        println!("{:?}", v);
                    }
                }
            }
            Yaml::Alias(_) => todo!(),
            Yaml::Null => todo!(),
            Yaml::BadValue => todo!(),
        }
    }

    #[test]
    fn load_yaml_file_pass() {
        let result = load_yaml_file("yaml_samps/test.yml");
        assert!(result.is_ok());
        walk_node(&result.unwrap()[0], 0);
    }
    #[test]
    fn load_yaml_file_fail() {
        let y = load_yaml_file("../yaml_samps/test_noexist.yml");
        assert!(y.is_err());
    }
}
