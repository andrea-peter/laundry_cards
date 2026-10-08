use std::fs;

use clap::Parser;

use laundry_cards::db::create_db;

// Command line parameters
#[derive(Parser)]
struct Args {
    out_file: String,
    #[arg(short = 'f', help = "Force file creation")]
    force: bool,
}

fn main() {
    let args = Args::parse();

    if fs::exists(&args.out_file).unwrap() {
        if args.force {
            println!("Removing existing DB {}", args.out_file);
            fs::remove_file(&args.out_file).expect("Could not remove existing DB");
        } else {
            panic!("File already exists");
        }
    }

    println!("Creating DB at {}", args.out_file);
    create_db(&args.out_file).unwrap();

    println!("Done");
}
