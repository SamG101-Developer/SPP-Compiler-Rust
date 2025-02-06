use clap::{arg, Command};
use crate::spp::entry::compiler_entry::CompilerEntry;

const VERSION: &str = env!("CARGO_PKG_VERSION");


/*
spp commands:
    spp init: create a new spp project
    spp vcs: manage version control systems
    spp build: build the spp project (--mode: [debug, release])
    spp run: run the spp project (--mode: [debug, release])
    spp test: test the spp project (--mode: [debug, release])
    spp clean: clean the spp project (--mode: [debug, release])
    spp version: display the spp version
 */


pub fn cli() -> Command {
    Command::new("spp")
        .about("The S++ programming language")
        .subcommand_required(true)
        .subcommand(
            Command::new("init")
                .about("Create a new S++ project")
        )
        .subcommand(
            Command::new("vcs")
                .about("Manage version control systems")
        )
        .subcommand(
            Command::new("build")
                .about("Build the S++ project")
                .arg(arg!("mode")
                    .short('m')
                    .long("mode")
                    .help("The build mode")
                    .value_parser(["debug", "release"])
                    .default_value("debug"))
        )
        .subcommand(
            Command::new("run")
                .about("Run the S++ project")
                .arg(arg!("mode")
                    .short('m')
                    .long("mode")
                    .help("The run mode")
                    .value_parser(["debug", "release"])
                    .default_value("debug"))
        )
        .subcommand(
            Command::new("test")
                .about("Test the S++ project")
                .arg(arg!("mode")
                    .short('m')
                    .long("mode")
                    .help("The test mode")
                    .value_parser(["debug", "release"])
                    .default_value("debug"))
        )
        .subcommand(
            Command::new("clean")
                .about("Clean the S++ project")
                .arg(arg!("mode")
                    .short('m')
                    .long("mode")
                    .help("The clean mode")
                    .value_parser(["debug", "release"])
                    .default_value("debug"))
        )
        .subcommand(
            Command::new("version")
                .about("Display the S++ version")
        )
}

pub fn handle_init() {
    // Check if the current directory is empty or not.
    let cwd = std::env::current_dir().unwrap();
    if let Ok(entries) = std::fs::read_dir(&cwd) {
        if entries.count() > 0 {
            eprintln!("The current directory is not empty.");
            return;
        }
    }

    // Determine the directory structure (src and bin folders).
    let bin_dir = cwd.join("bin");
    let src_dir = cwd.join("src");
    let src_folder = src_dir.join(cwd.file_name().unwrap());
    let main_file = src_folder.join("main.spp");
    let toml_file = cwd.join("spp.toml");

    // Create the directory structure.
    std::fs::create_dir(&bin_dir).expect("Failed to create bin directory.");
    std::fs::create_dir(&src_dir).expect("Failed to create src directory.");
    std::fs::create_dir(&src_folder).expect("Failed to create src folder.");

    // Create src/main.spp and spp.toml files.
    std::fs::write(
        &main_file,
        "fun main(args: std::Vec[std::Str]) -> std::Void {\n    ret\n}"
    ).expect("Failed to create src/main.spp file.");

    let project_name = cwd.file_name().unwrap().to_str().unwrap();
    std::fs::write(
        &toml_file,
        format!("[project]\nname = \"{project_name}\"\nversion = \"0.1.0\"\n\n[vcs]\nstd = {{ git = \"https://github.com/SamG101-Developer/SPP-STL\", branch = \"master\" }}")
    ).expect("Failed to create spp.toml file.");
}


pub fn handle_vcs() {
    // Check if the spp.toml file exists.
    let cwd = std::env::current_dir().unwrap();
    let toml_file = cwd.join("spp.toml");
    if !toml_file.exists() {
        eprintln!("The spp.toml file does not exist.");
        return;
    }

    // Parse the spp.toml file.
    let toml_str = std::fs::read_to_string(&toml_file).expect("Failed to read spp.toml file.");
    let toml = toml::from_str::<toml::Value>(&toml_str).expect("Failed to parse spp.toml file.");

    // Check if the vcs section exists.
    let vcs = toml.get("vcs").expect("The vcs section does not exist.");

    // Check if the "vcs" folder exists, and create it if it doesn't.
    let vcs_dir = cwd.join("vcs");
    if !vcs_dir.exists() {
        std::fs::create_dir(&vcs_dir).expect("Failed to create vcs directory.");
    }

    // Move into the "vcs" folder.
    std::env::set_current_dir(&vcs_dir).expect("Failed to move into the vcs directory.");

    for (key, info) in vcs.as_table().expect("The vcs section is not a table.") {
        // Determine the repository URL and branch name.
        let git = info.get("git").expect("The git section does not exist.");
        let branch = info.get("branch").expect("The branch section does not exist.");
        let git_url = git.as_str().expect("The git section is not a string.");
        let branch_name = branch.as_str().expect("The branch section is not a string.");

        // If the repository has been cloned, pull the latest changes.
        if let Ok(entries) = std::fs::read_dir(&key) {
            if entries.count() > 0 {
                // Move into the repository folder.
                std::env::set_current_dir(&key).expect("Failed to move into the repository directory.");

                // Pull the latest changes.
                let status = std::process::Command::new("git")
                    .arg("pull")
                    .status().expect(format!("Failed to pull the latest changes for the repository '{key}'.").as_str());

                // Check if the latest changes were successfully pulled.
                if status.success() {
                    println!("Successfully pulled the latest changes for the repository {key}.");
                } else {
                    eprintln!("Failed to pull the latest changes for the repository {key}.");
                }

                // Exit the repository folder.
                std::env::set_current_dir(&vcs_dir).expect("Failed to move back to the vcs directory.");
                continue;
            }
        }

        // Otherwise, clone the standard library repository.
        let status = std::process::Command::new("git")
            .arg("clone")
            .arg(git_url)
            .arg("--branch")
            .arg(branch_name)
            .arg("std")
            .status().expect(format!("Failed to clone the repository '{key}'.").as_str());

        // Check if the repository was successfully cloned.
        if status.success() {
            println!("Successfully cloned the repository {key}.");
        } else {
            eprintln!("Failed to clone the repository {key}.");
        }
    }

    // Exit the "vcs" folder.
    std::env::set_current_dir(&cwd).expect("Failed to move back to the project directory.");
}

pub fn handle_build(mode: &String) {
    // Check if the bin directory exists.
    let cwd = std::env::current_dir().unwrap();
    let bin_dir = cwd.join("bin");
    if !bin_dir.exists() {
        eprintln!("The bin directory does not exist.");
        return;
    }

    // If the bin/<mode> directory exists, clear it. Otherwise, create it.
    let mode_dir = bin_dir.join(mode.clone());
    if mode_dir.exists() {
        handle_clean(mode);
    } else {
        std::fs::create_dir(&mode_dir).expect("Failed to create the mode directory.");
    }

    // Handle vcs operations.
    handle_vcs();

    // Create the compiler.
    let compiler = CompilerEntry{};
    compiler.compile(mode);
}

pub fn handle_run(mode: &String) {
    // Build the project.
    handle_build(mode);

    // Todo: Run the project.
}

pub fn handle_test(mode: &String) {
    // Todo: Test the project
}

pub fn handle_clean(mode: &String) {
    // Check if the bin directory exists.
    let cwd = std::env::current_dir().unwrap();
    let bin_dir = cwd.join("bin");
    if !bin_dir.exists() {
        eprintln!("The bin directory does not exist.");
        return;
    }

    // If the bin/<mode> directory exists, clear it.
    let mode_dir = bin_dir.join(mode);
    if mode_dir.exists() {
        std::fs::remove_dir_all(&mode_dir).expect("Failed to remove the mode directory.");
    } else {
        eprintln!("The mode directory does not exist.");
    }
}

pub fn handle_version() {
    println!("S++ version {}", VERSION);
}
