#![feature(trait_alias)]
#![feature(unboxed_closures)]
#![feature(let_chains)]

mod spp;
mod spp_cli;

fn main() {
    unsafe { std::env::set_var("RUST_BACKTRACE", "1"); }
    let matches = spp_cli::cli().get_matches();
    match matches.subcommand() {
        Some(("init", _)) => {
            println!("Creating a new S++ project...");
            spp_cli::handle_init();
        }
        Some(("vcs", _)) => {
            println!("Managing version control systems...");
            spp_cli::handle_vcs();
        }
        Some(("build", build_matches)) => {
            let mode = build_matches.get_one::<String>("mode").unwrap();
            println!("Building the S++ project in {} mode...", mode);
            spp_cli::handle_build(&mode);
        }
        Some(("run", run_matches)) => {
            let mode = run_matches.get_one::<String>("mode").unwrap();
            println!("Running the S++ project in {} mode...", mode);
            spp_cli::handle_run(&mode);
        }
        Some(("test", test_matches)) => {
            let mode = test_matches.get_one::<String>("mode").unwrap();
            println!("Testing the S++ project in {} mode...", mode);
            spp_cli::handle_test(&mode);
        }
        Some(("clean", clean_matches)) => {
            let mode = clean_matches.get_one::<String>("mode").unwrap();
            println!("Cleaning the S++ project in {} mode...", mode);
            spp_cli::handle_clean(&mode);
        }
        Some(("validate", validate_matches)) => {
            println!("Validating the S++ project");
            spp_cli::handle_validate();
        }
        Some(("version", _)) => {
            println!("S++ version 0.1.0");
            spp_cli::handle_version();
        }
        _ => {
            unreachable!("Invalid subcommand");
        }
    }
}
