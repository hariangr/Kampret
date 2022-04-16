use std::fs;

#[derive(Debug)]
pub enum DataTypes {
    KString(String),
    KNil,
    KI64(i64),
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

        if it.is_i64() {
            panic!("i64 can only be created inside a variable")
        }
        if it.is_string() {
            panic!("string can only be created inside a variable")
        }

        if it.is_symbol() {
            match it.as_symbol().unwrap() {
                STRING_CONST => {
                    let content = lexed
                        .next()
                        .unwrap()
                        .as_str()
                        .expect("failed adding string to stack");
                    stacks.push(DataTypes::KString(content.to_owned()));
                    continue;
                }
                "print" => {
                    let pop_stack = stacks
                        .pop()
                        .expect("Print expect at least one item in stack");
                    match pop_stack {
                        DataTypes::KString(it) => print!("{}", it),
                        DataTypes::KI64(it) => print!("{}", it),
                        _ => panic!("Echo can only print string and i64 from stack at this time"),
                    }
                    continue;
                }
                "println" => {
                    let pop_stack = stacks
                        .pop()
                        .expect("Print expect at least one item in stack");
                    match pop_stack {
                        DataTypes::KString(it) => println!("{}", it),
                        DataTypes::KI64(it) => println!("{}", it),
                        _ => panic!("Print can only print string and i64 from stack at this time"),
                    }
                    continue;
                }
                "i64.const" => {
                    let content = lexed
                        .next()
                        .unwrap()
                        .as_i64()
                        .expect("failed adding i64 to stack");
                    stacks.push(DataTypes::KI64(content));
                }
                "i64.add" => {
                    if let DataTypes::KI64(rhs) =
                        stacks.pop().expect("can't get right hand operand")
                    {
                        if let DataTypes::KI64(lhs) =
                            stacks.pop().expect("can't get left hand operand")
                        {
                            stacks.push(DataTypes::KI64(lhs + rhs));
                            continue;
                        }
                    }
                    // This code will ever get executed if either left or right operand isn't a KI64 variant
                    // There's not `if let not` in Rust it seems, see https://github.com/rust-lang/rfcs/issues/2616
                    panic!("i64 addition require the two operand to be i64 number")
                }
                stmt => {
                    panic!("Statement of {} is not supported, maybe yet, or ever", stmt);
                }
            }
        }
    }

    println!("\n--------------\nStacks: {:?}\n", stacks);
}

fn main() {
    let src = fs::read_to_string("./kasm/42.wat").unwrap();
    interpret(&src)
}
