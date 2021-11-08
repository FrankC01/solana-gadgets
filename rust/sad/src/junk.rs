mod tests {

    use std::collections::HashMap;

    use borsh::{
        schema::{Definition, Fields},
        BorshDeserialize, BorshSchema, BorshSerialize,
    };

    #[test]
    fn test_schema() {
        #[derive(BorshSerialize, Debug)]
        struct A {
            foo: u64,
            bar: String,
        }
        #[derive(BorshSchema, BorshDeserialize, Debug)]
        struct B;
        fn setup_faux_struc() -> Definition {
            Definition::Struct {
                fields: Fields::NamedFields(vec![
                    ("foo".to_string(), "u64".to_string()),
                    ("bar".to_string(), "string".to_string()),
                ]),
            }
        }
        let my_type = setup_faux_struc();
        let mut my_def = HashMap::<String, Definition>::new();
        my_def.insert("B".to_string(), setup_faux_struc());
        B::add_definition("B".to_string(), my_type, &mut my_def);
        // SadDerived::add_definitions_recursively(&mut my_def);
        println!("{:?}", B::declaration());

        let a_with_val = A {
            foo: 1000,
            bar: "goofy".to_string(),
        };
        let b = a_with_val.try_to_vec().unwrap();
        println!("{:?}", b);
        let c = B::try_from_slice(&b).unwrap();
        println!("{:?}", c);
    }
}
