use anyhow::{anyhow, Result};
use minc_cranelift::parser::parser;
use std::env;

fn arg_parse() -> Result<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(anyhow!("minc-clanelift required compile target"));
    }

    let compile_target = &args[1];
    Ok(compile_target.to_owned())
}

fn main() {
    let source = arg_parse().unwrap();
    let ast = parser::statements(&source).unwrap();
    println!("{:#?}", ast);
}
