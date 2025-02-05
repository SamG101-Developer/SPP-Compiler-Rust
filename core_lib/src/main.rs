#![feature(trait_alias)]
#![feature(unboxed_closures)]

mod spp_cli;
mod spp;

fn main() {
    let matches = spp_cli::cli().get_matches();
    match matches.subcommand() {
        Some(("init", _)) => {
            println!("Creating a new S++ project...");
        }
        Some(("vcs", _)) => {
            println!("Managing version control systems...");
        }
        Some(("build", build_matches)) => {
            let mode = build_matches.get_one::<String>("mode").unwrap();
            println!("Building the S++ project in {} mode...", mode);
        }
        Some(("run", run_matches)) => {
            let mode = run_matches.get_one::<String>("mode").unwrap();
            println!("Running the S++ project in {} mode...", mode);
        }
        Some(("test", test_matches)) => {
            let mode = test_matches.get_one::<String>("mode").unwrap();
            println!("Testing the S++ project in {} mode...", mode);
        }
        Some(("clean", clean_matches)) => {
            let mode = clean_matches.get_one::<String>("mode").unwrap();
            println!("Cleaning the S++ project in {} mode...", mode);
        }
        Some(("version", _)) => {
            println!("S++ version 0.1.0");
        }
        _ => {
            unreachable!("Invalid subcommand");
        }
    }
}
