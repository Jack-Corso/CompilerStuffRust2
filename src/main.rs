use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;
use crate::debugging::PrettyPrint;
use crate::parsing::parse;

mod lexing;
mod parsing;
mod typing;
mod debugging;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args[1].as_str());

    let out_path = path.with_extension("s");

    let reader: BufReader<File> = BufReader::new(File::open(path)?);

    let start_time = Instant::now();
    let res = lexing::tokenize(reader);

    let ast = parse(res);
    ast.pretty_println(0);

    println!("Done Tokenizing in {}ms", start_time.elapsed().as_millis());
    
    Ok(())
}
