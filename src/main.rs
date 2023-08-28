mod generator;
use generator::Generator;

mod parser;
use parser::Parser;

use std::{
    fs::{self, File},
    io::Write,
    process::{self, Command},
};

use clap::Parser as ClapParser;

#[derive(ClapParser)]
struct Cli {
    file: std::path::PathBuf,
    output: String,
}

fn main() {
    let args = Cli::parse();

    let mut parser = Parser::new(fs::read_to_string(args.file).expect("Failed to read file"));
    let node_prog = parser.parse();

    let mut generator = Generator::new(node_prog);
    let generated_code = generator.gen_prog();

    let mut file = File::create("out.asm").expect("Failed to create file");
    file.write_all(generated_code.as_bytes())
        .expect("Failed to write to file");

    let nasm_check = Command::new("which").arg("nasm").output();

    if nasm_check.is_ok() && nasm_check.unwrap().stdout.len() == 0 {
        eprintln!("nasm not found (try `apt install nasm` or `brew install nasm`)");
        process::exit(1);
    }

    let ld_check = Command::new("which").arg("ld").output();

    if ld_check.is_ok() && ld_check.unwrap().stdout.len() == 0 {
        eprintln!("ld not found (try `apt install binutils` or `brew install binutils`)");
        process::exit(1);
    }

    let status_nasm = Command::new("nasm")
        .arg("-felf64")
        .arg("out.asm")
        .status()
        .expect("Failed to execute nasm");

    if status_nasm.success() {
        fs::remove_file("out.asm").expect("Failed to remove file");

        let status_ld = Command::new("ld")
            .arg("-o")
            .arg(args.output)
            .arg("out.o")
            .status()
            .expect("Failed to execute ld");

        if !status_ld.success() {
            eprintln!("ld command failed");
            process::exit(1);
        }

        fs::remove_file("out.o").expect("Failed to remove file");
    } else {
        eprintln!("nasm command failed");
        process::exit(1);
    }
}
