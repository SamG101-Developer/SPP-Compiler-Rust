use crate::spp::entry::program_module::ProgramModule;
use std::path::Path;
use std::slice::IterMut;

pub struct ProgramModuleTree {
    src_path: String,
    vcs_path: String,
    modules: Vec<ProgramModule>,
}

impl ProgramModuleTree {
    pub fn new(path: String) -> Self {
        Self {
            src_path: Path::new(&path).join("src").to_str().unwrap().to_string(),
            vcs_path: Path::new(&path).join("vcs").to_str().unwrap().to_string(),
            modules: Vec::new(),
        }
    }

    pub fn scan_modules(&mut self) {
        let cwd = std::env::current_dir().unwrap().to_str().unwrap().to_string();

        // Get all the spp module files from the src and vcs paths.
        let src_modules = glob::glob(&format!("{}/**/*.spp", self.src_path))
            .unwrap()
            .map(|x| x.unwrap().to_str().unwrap().to_string());

        let vcs_modules = glob::glob(&format!("{}/**/*.spp", self.vcs_path))
            .unwrap()
            .map(|x| x.unwrap().to_str().unwrap().to_string());

        // Merge the modules into a single list of ProgramModule instances.
        self.modules = src_modules
            .chain(vcs_modules)
            .map(|path| ProgramModule::new(self.strip_path(&path, cwd.clone())))
            .collect::<Vec<_>>();
    }

    fn strip_path(&self, path: &str, cwd: String) -> String {
        // Replace the cwd with "" in the path, so the path is relative to the cwd.
        path.replace(&cwd, "")
    }

    pub fn modules(&mut self) -> IterMut<ProgramModule> {
        self.modules.iter_mut()
    }

    pub fn modules_clone(&self) -> Vec<ProgramModule> {
        self.modules.clone()
    }

    pub fn remove_module(&mut self, module: &ProgramModule) {
        self.modules.retain(|x| x != module);
    }
}
