use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("Usage: miniforth inputfilename");
        std::process::exit(1);
    }
    let input = std::fs::read_to_string(&args[0])?;
    let input = input.split_whitespace().collect();
    eval(input, Vec::new(), HashMap::new())?;

    Ok(())
}

#[derive(Debug)]
struct Error;
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error")
    }
}

#[derive(Debug, Copy, Clone)]
enum Word {
    I64(i64),
    Bool(bool),
}

fn eval<'a>(
    input: Vec<&'a str>,
    mut stack: Vec<Word>,
    mut definitions: HashMap<&'a str, Vec<&'a str>>,
) -> Result<(Vec<Word>, HashMap<&'a str, Vec<&'a str>>), Box<dyn std::error::Error>> {
    let mut idx = 0;

    while let Some(&word) = input.get(idx) {
        match word {
            "+" => {
                let a = stack.pop().ok_or(Error)?;
                let b = stack.pop().ok_or(Error)?;
                match (a, b) {
                    (Word::I64(a), Word::I64(b)) => stack.push(Word::I64(b + a)),
                    _ => return Err(Error.into()),
                }
            }
            "-" => {
                let a = stack.pop().ok_or(Error)?;
                let b = stack.pop().ok_or(Error)?;
                match (a, b) {
                    (Word::I64(a), Word::I64(b)) => stack.push(Word::I64(b - a)),
                    _ => return Err(Error.into()),
                }
            }
            "*" => {
                let a = stack.pop().ok_or(Error)?;
                let b = stack.pop().ok_or(Error)?;
                match (a, b) {
                    (Word::I64(a), Word::I64(b)) => stack.push(Word::I64(b * a)),
                    _ => return Err(Error.into()),
                }
            }
            "/" => {
                let a = stack.pop().ok_or(Error)?;
                let b = stack.pop().ok_or(Error)?;
                match (a, b) {
                    (Word::I64(a), Word::I64(b)) => stack.push(Word::I64(b / a)),
                    _ => return Err(Error.into()),
                }
            }
            "and" => {
                let a = stack.pop().ok_or(Error)?;
                let b = stack.pop().ok_or(Error)?;
                match (a, b) {
                    (Word::I64(a), Word::I64(b)) => stack.push(Word::I64(b & a)),
                    (Word::Bool(a), Word::Bool(b)) => stack.push(Word::Bool(b && a)),
                    _ => return Err(Error.into()),
                }
            }
            "or" => {
                let a = stack.pop().ok_or(Error)?;
                let b = stack.pop().ok_or(Error)?;
                match (a, b) {
                    (Word::I64(a), Word::I64(b)) => stack.push(Word::I64(b | a)),
                    (Word::Bool(a), Word::Bool(b)) => stack.push(Word::Bool(b || a)),
                    _ => return Err(Error.into()),
                }
            }
            "xor" => {
                let a = stack.pop().ok_or(Error)?;
                let b = stack.pop().ok_or(Error)?;
                match (a, b) {
                    (Word::I64(a), Word::I64(b)) => stack.push(Word::I64(b ^ a)),
                    (Word::Bool(a), Word::Bool(b)) => stack.push(Word::Bool(b ^ a)),
                    _ => return Err(Error.into()),
                }
            }
            "lshift" => {
                let a = stack.pop().ok_or(Error)?;
                let b = stack.pop().ok_or(Error)?;
                match (a, b) {
                    (Word::I64(a), Word::I64(b)) => stack.push(Word::I64(b << a)),
                    _ => return Err(Error.into()),
                }
            }
            "rshift" => {
                let a = stack.pop().ok_or(Error)?;
                let b = stack.pop().ok_or(Error)?;
                match (a, b) {
                    (Word::I64(a), Word::I64(b)) => stack.push(Word::I64(b >> a)),
                    _ => return Err(Error.into()),
                }
            }
            "=" => {
                let a = stack.pop().ok_or(Error)?;
                let b = stack.pop().ok_or(Error)?;
                match (a, b) {
                    (Word::I64(a), Word::I64(b)) => stack.push(Word::Bool(b == a)),
                    (Word::Bool(a), Word::Bool(b)) => stack.push(Word::Bool(b == a)),
                    _ => return Err(Error.into()),
                }
            }
            ">" => {
                let a = stack.pop().ok_or(Error)?;
                let b = stack.pop().ok_or(Error)?;
                match (a, b) {
                    (Word::I64(a), Word::I64(b)) => stack.push(Word::Bool(b > a)),
                    _ => return Err(Error.into()),
                }
            }
            "<" => {
                let a = stack.pop().ok_or(Error)?;
                let b = stack.pop().ok_or(Error)?;
                match (a, b) {
                    (Word::I64(a), Word::I64(b)) => stack.push(Word::Bool(b < a)),
                    _ => return Err(Error.into()),
                }
            }
            "negate" => {
                let a = stack.pop().ok_or(Error)?;
                match a {
                    Word::I64(a) => stack.push(Word::I64(-a)),
                    Word::Bool(a) => stack.push(Word::Bool(!a)),
                }
            }
            "dup" => {
                let copy = *stack.last().ok_or(Error)?;
                stack.push(copy);
            }
            "drop" => {
                stack.pop().ok_or(Error)?;
            }
            "swap" => {
                let len = stack.len();
                if len < 2 {
                    return Err(Error.into());
                }
                stack.swap(len - 1, len - 2);
            }
            "over" => {
                let len = stack.len();
                if len < 2 {
                    return Err(Error.into());
                }
                let a = *(stack.get(len - 2).ok_or(Error))?;
                stack.push(a);
            }
            "." => {
                let len = stack.len();
                if len < 1 {
                    return Err(Error.into());
                }
                match stack.pop().ok_or(Error)? {
                    Word::Bool(a) => {
                        println!("{}", a);
                    }
                    Word::I64(a) => {
                        println!("{}", a);
                    }
                }
            }
            "true" => stack.push(Word::Bool(true)),
            "false" => stack.push(Word::Bool(false)),
            ":" => {
                let mut def = Vec::new();
                idx += 1;
                loop {
                    match input.get(idx) {
                        Some(&";") | None => break,
                        Some(&word) => {
                            idx += 1;
                            def.push(word);
                        }
                    }
                }
                if def.len() < 2 {
                    return Err(Error.into());
                }
                match def[0] {
                    "+" | "-" | "*" | "/" | "and" | "or" | "xor" | "lshift" | "rshift" | "="
                    | ">" | "<" | "negate" | "dup" | "drop" | "swap" | "over" | "true"
                    | "false" | ":" | ";" => return Err(Error.into()),
                    _ => {}
                }
                definitions.insert(def[0], def[1..].to_owned());
            }
            ";" => return Err(Error.into()),
            string => {
                if let Ok(num) = string.parse::<i64>() {
                    stack.push(Word::I64(num));
                    idx += 1;
                    continue;
                };

                let fun = definitions.get(string).ok_or(Error)?.clone();
                let res = eval(fun, stack, definitions)?;
                stack = res.0;
                definitions = res.1;
            }
        }
        idx += 1;
    }

    Ok((stack, definitions))
}
