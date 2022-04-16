use std::fs;

#[derive(Debug)]
pub enum DataTypes {
    KString(String),
    KNil,
}

const STRING_CONST: &str = "string.const";

fn interpret(src: &str) {
    let lexed = lexpr::from_str(&src).unwrap();
    let mut lexed = lexed.list_iter().expect("entah");

    let mut stacks: Vec<DataTypes> = vec![];

    let it = lexed.next().unwrap();
    if it.as_symbol().unwrap() != "module" {
        panic!("root node must be a module")
    }

    while let Some(it) = lexed.next() {
        // println!("{:?}\n", it);

        if it.is_symbol() {
            let it = it.as_symbol().unwrap();

            if it == STRING_CONST {
                let content = lexed
                    .next()
                    .unwrap()
                    .as_str()
                    .expect("failed adding string to stack");
                stacks.push(DataTypes::KString(content.to_owned()));
                continue;
            }

            if it == "print" {
                let pop_stack = stacks
                    .pop()
                    .expect("Print expect at least one item in stack");
                match pop_stack {
                    DataTypes::KString(it) => println!("{}", it),
                    _ => panic!("Print can only print string from stack at this time")
                }
                continue;
            }
        }
    }

    println!("\n--------------\nStacks: {:?}\n", stacks);

    // for it in lexed.list_iter().expect("Expected at least one module") {
    //     if it.is_symbol() {
    //         println!("{:?}\n", it.as_symbol().unwrap());
    //     } else {
    //         println!("{:?}\n", it);
    //     }
    // }
}

fn main() {
    let src = fs::read_to_string("./kasm/42.wat").unwrap();
    interpret(&src)
}
