use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::time::Instant;
use crate::debugging::PrettyPrint;
use crate::parsing::parse;

mod lexing;
mod parsing;
mod typing;
mod debugging;
mod stack;
mod passes;
mod assembly;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args[1].as_str());

    let out_path = path.with_extension("s");

    let reader: BufReader<File> = BufReader::new(File::open(path)?);

    let start_time = Instant::now();
    let res = lexing::tokenize(reader);

    println!("Done Tokenizing in {}ms", start_time.elapsed().as_millis());

    let start_time = Instant::now();
    let mut ast = parse(res);
    passes::validate_vars(&mut ast);
    passes::scope_vars(&mut ast);
    passes::validate_returns(&mut ast);
    passes::solve_types(&mut ast);
    passes::validate_types(&mut ast);

    println!("Done Generating AST in {}ms", start_time.elapsed().as_millis());

    ast.pretty_println(0);

    let start_time = Instant::now();
    assembly::generate_asm(ast, &mut BufWriter::new(File::create(out_path)?))?;

    println!("Done Generating Assembly in {}ms", start_time.elapsed().as_millis());

    Ok(())
}
