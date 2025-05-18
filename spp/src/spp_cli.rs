use crate::spp::entry::compiler_entry::CompilerEntry;
use clap::{Arg, ArgAction, Command, Parser, Subcommand};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn cli() -> Command {
    Command::new("spp")
        .about("The S++ programming language")
        .subcommand_required(true)
        .subcommand(Command::new("init").about("Create a new S++ project"))
        .subcommand(Command::new("vcs").about("Manage version control systems"))
        .subcommand(
            Command::new("build").about("Build the S++ project").arg(
                Arg::new("mode")
                    .short('m')
                    .long("mode")
                    .value_name("MODE")
                    .action(ArgAction::Set)
                    .help("The build mode")
                    .value_parser(["debug", "release"])
                    .default_value("debug"),
            ),
        )
        .subcommand(
            Command::new("run").about("Build the S++ project").arg(
                Arg::new("mode")
                    .short('m')
                    .long("mode")
                    .value_name("MODE")
                    .action(ArgAction::Set)
                    .help("The build mode")
                    .value_parser(["debug", "release"])
                    .default_value("debug"),
            ),
        )
        .subcommand(
            Command::new("test").about("Build the S++ project").arg(
                Arg::new("mode")
                    .short('m')
                    .long("mode")
                    .value_name("MODE")
                    .action(ArgAction::Set)
                    .help("The build mode")
                    .value_parser(["debug", "release"])
                    .default_value("debug"),
            ),
        )
        .subcommand(
            Command::new("clean").about("Build the S++ project").arg(
                Arg::new("mode")
                    .short('m')
                    .long("mode")
                    .value_name("MODE")
                    .action(ArgAction::Set)
                    .help("The build mode")
                    .value_parser(["debug", "release", "all"])
                    .default_value("all"),
            ),
        )
        .subcommand(Command::new("validate").about("Validate the S++ project"))
        .subcommand(Command::new("version").about("Display the S++ version"))
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
    let project_name = cwd.file_name().unwrap().to_str().unwrap();

    // Create the "src/main.spp" entry file.
    std::fs::write(
        &main_file,
        "fun main(args: std::vector::Vec[std::string::Str]) -> std::void::Void {\n    ret\n}",
    )
    .expect("Failed to create 'src/main.spp' entry file.");

    // Create the "spp.toml" config file.
    std::fs::write(
        &toml_file,
        format!("[project]\nname = \"{project_name}\"\nversion = \"0.1.0\"\n\n[vcs]\nstd = {{ git = \"https://github.com/SamG101-Developer/SPP-STL\", branch = \"master\" }}")
    ).expect("Failed to create 'spp.toml' config file.");
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

    // Check if the "ffi" folder exists, and create it if it doesn't.
    let ffi_dir = cwd.join("ffi");
    if !ffi_dir.exists() {
        std::fs::create_dir(&ffi_dir).expect("Failed to create ffi directory.");
    }

    // Check if the "vcs" folder exists, and create it if it doesn't.
    let vcs_dir = cwd.join("vcs");
    if !vcs_dir.exists() {
        std::fs::create_dir(&vcs_dir).expect("Failed to create vcs directory.");
    }

    // Move into the "vcs" folder.
    std::env::set_current_dir(&vcs_dir).expect("Failed to move into the vcs directory.");

    for (repo_name, info) in vcs.as_table().expect("The vcs section is not a table.") {
        // Determine the repository URL and branch name.
        let repo_url = {
            let url = info.get("git").expect("The git section does not exist.");
            url.as_str().expect("The git section is not a string.")
        };
        let repo_branch = {
            let branch = info.get("branch").expect("The branch section does not exist.");
            branch.as_str().expect("The branch section is not a string.")
        };
        let repo_folder = vcs_dir.join(repo_name);

        // Pull any updates if the repository has already been cloned.
        if let Ok(_) = std::fs::read_dir(&repo_name) {
            std::process::Command::new("git").args(["-C", repo_folder.to_str().unwrap(), "pull", "origin", repo_branch]).status().expect(format!("Failed to pull the latest changes for the repository '{repo_name}'.").as_str());
            std::process::Command::new("git").args(["-C", repo_folder.to_str().unwrap(), "checkout", repo_branch]).status().expect(format!("Failed to checkout the branch '{repo_branch}' for the repository '{repo_name}'.").as_str());
        }

        // Clone the repository if it doesn't exist, and checkout the specified branch.
        else {
            std::process::Command::new("git").args(["clone", repo_url, repo_folder.to_str().unwrap()]).status().expect(format!("Failed to clone the repository '{repo_name}'.").as_str());
            std::process::Command::new("git").args(["-C", repo_folder.to_str().unwrap(), "checkout", repo_branch]).status().expect(format!("Failed to checkout the branch '{repo_branch}' for the repository '{repo_name}'.").as_str());
        }

        // Copy all DLLs from the vcs's ffi folder into this project's ffi folder.
        let ffi_repo_folder = repo_folder.join("ffi");
        if let Ok(_) = std::fs::read_dir(&ffi_repo_folder) {
            for lib in std::fs::read_dir(&ffi_repo_folder).expect("Failed to read the ffi directory.") {
                let src_path = lib.expect("Failed to read the entry.").path();
                let lib_file_name = src_path.file_name().expect("Failed to get the file name.").to_str().expect("Failed to convert the file name to a string.");
                let dest_path = ffi_dir.join(lib_file_name);
                
                // Remove the destination path if it exists.
                if dest_path.exists() {
                    std::fs::remove_dir_all(&dest_path).expect("Failed to remove the destination path.");
                }
                
                copy_dir::copy_dir(src_path, &dest_path).expect("Failed to copy the ffi directory.");
            }
        } else {
            eprintln!("The ffi folder does not exist in the repository '{repo_name}'.");
        }
    }

    // Exit the "vcs" folder.
    std::env::set_current_dir(&cwd).expect("Failed to move back to the project directory.");
}

pub fn handle_build(mode: &String) {
    let cwd = std::env::current_dir().unwrap();

    // Check if the "bin" directory exists.
    let bin_dir = cwd.join("bin");
    if !bin_dir.exists() {
        std::fs::create_dir(&bin_dir).expect("Failed to create the bin directory.");
    }

    // Check if the "bin/<mode>" directory exists.
    let mode_dir = bin_dir.join(mode.clone());
    if !mode_dir.exists() {
        std::fs::create_dir(&mode_dir).expect("Failed to create the mode directory.");
    }

    // Validate the structure of the project and load the vcs code.
    if !handle_validate() {
        eprintln!("The project structure is invalid.");
        return;
    }
    handle_vcs();
    if !handle_validate() {
        eprintln!("The project structure is invalid after loading the vcs code.");
        return;
    }

    // Create the compiler.
    let compiler = CompilerEntry::new();
    compiler.compile(mode).unwrap();
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

pub fn handle_validate() -> bool {
    // Check there is a spp.toml, src, and src/main.spp file.
    let cwd = std::env::current_dir().unwrap();
    let toml_file = cwd.join("spp.toml");
    let src_dir = cwd.join("src");
    let main_file = src_dir.join("main.spp");

    if !toml_file.exists() {
        eprintln!("The spp.toml file does not exist.");
        return false;
    }

    if !src_dir.exists() {
        eprintln!("The src directory does not exist.");
        return false;
    }

    if !main_file.exists() {
        eprintln!("The src/main.spp file does not exist.");
        return false;
    }

    // If there is a vcs folder, run the validation steps recursively.
    let vcs_dir = cwd.join("vcs");
    if vcs_dir.exists() {
        for repo in std::fs::read_dir(&vcs_dir).expect("Failed to read the vcs directory.") {
            let repo_path = repo.expect("Failed to read the entry.").path();
            std::env::set_current_dir(&repo_path).expect("Failed to move into the vcs directory.");
            handle_validate();
            std::env::set_current_dir(&cwd).expect("Failed to move back to the project directory.");
        }
    }

    // If there is an FFI folder, check each subfolder is structured properly.
    let ffi_dir = cwd.join("ffi");
    if ffi_dir.exists() {
        let ext = std::env::consts::DLL_EXTENSION;

        for lib_path in std::fs::read_dir(&ffi_dir).expect("Failed to read the ffi directory.") {
            let lib_path = lib_path.expect("Failed to read the entry.").path();
            if !lib_path.is_dir() {
                eprintln!("The ffi folder contains a file that is not a directory.");
                return false;
            }

            // Check for "library_name/lib/library_name.{ext}" and "library_name/stub.spp" files.
            let lib_name = lib_path.file_name().expect("Failed to get the file name.").to_str().expect("Failed to convert the file name to a string.");
            let lib_file = lib_path.join("lib").join(format!("{lib_name}.{ext}"));
            let stub_file = lib_path.join("stub.spp");

            if !lib_file.exists() {
                eprintln!("The ffi folder does not contain the library file '{lib_name}.{ext}'.");
                return false;
            }

            if !stub_file.exists() {
                eprintln!("The ffi folder does not contain the stub file 'stub.spp'.");
                return false;
            }
        }
    }

    true
}

pub fn handle_version() {
    println!("S++ version {}", VERSION);
}
