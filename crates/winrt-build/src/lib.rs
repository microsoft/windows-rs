use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process::Stdio,
};

pub struct Builder {
    namespaces: Vec<String>,
    output: PathBuf,
    dependencies: Dependencies,
}

pub enum Dependencies {
    Os,
    Nuget(Vec<String>),
    OsAndNuget(Vec<String>),
}

impl Dependencies {
    fn winmds(&self) -> Vec<winmd::WinmdFile> {
        let mut result = std::collections::BTreeSet::new();
        let deps = match self {
            Dependencies::Os => vec![winmd::dependencies::system_metadata_root()],
            Dependencies::Nuget(deps) => deps
                .iter()
                .map(|d| winmd::dependencies::nuget_root().join(d))
                .collect(),
            Dependencies::OsAndNuget(deps) => {
                let mut deps: Vec<PathBuf> = deps
                    .iter()
                    .map(|d| winmd::dependencies::nuget_root().join(d))
                    .collect();

                deps.push(winmd::dependencies::system_metadata_root());
                deps
            }
        };
        for dep in deps {
            let dep = winmd::dependencies::nuget_root().join(dep);
            winmd::dependencies::expand_paths(dep, &mut result, true)
        }
        result.iter().map(winmd::WinmdFile::new).collect()
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            dependencies: Dependencies::Os,
            namespaces: Vec::new(),
            output: PathBuf::from(env::var("OUT_DIR").expect("No `OUT_DIR` env variable set"))
                .join("bindings.rs"),
        }
    }
}

impl Builder {
    pub fn insert_namespaces<T>(mut self, namespaces: T) -> Self
    where
        T: IntoIterator,
        T::Item: std::string::ToString,
    {
        self.namespaces
            .extend(namespaces.into_iter().map(|s| s.to_string()));
        self
    }

    pub fn insert_nuget<T>(mut self, nuget_deps: T) -> Self
    where
        T: IntoIterator,
        T::Item: std::string::ToString,
    {
        let new_deps = nuget_deps.into_iter().map(|i| i.to_string());
        match &mut self.dependencies {
            Dependencies::Os => self.dependencies = Dependencies::OsAndNuget(new_deps.collect()),
            Dependencies::Nuget(deps) => deps.extend(new_deps),
            Dependencies::OsAndNuget(deps) => deps.extend(new_deps),
        }
        self
    }

    pub fn output<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.output = path.as_ref().to_owned();
        self
    }

    pub fn build(&self) -> Result<(), Box<dyn std::error::Error>> {
        let dependencies = self.dependencies.winmds();
        let tr = winmd::TypeReader::new(dependencies);

        let mut tl = winmd::TypeLimits::new(&tr);
        for ns in &self.namespaces {
            tl.insert(winmd::NamespaceTypes {
                namespace: ns.clone(),
                limit: winmd::TypeLimit::All,
            });
        }

        let ts = winmd::TypeStage::from_limits(&tr, &tl);
        let tt = ts.into_tree();

        fs::create_dir_all(self.output.parent().unwrap())?;
        let mut f = fs::File::create(&self.output)?;

        let mut cmd = std::process::Command::new("rustfmt");
        cmd.arg("--emit").arg("stdout");
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        {
            let child = cmd.spawn()?;
            let mut stdin = child.stdin.unwrap();
            let stdout = child.stdout.unwrap();

            let t = std::thread::spawn(move || {
                let mut s = stdout;
                std::io::copy(&mut s, &mut f).unwrap();
            });

            // Only rerun if the output file has changed
            println!("cargo:rerun-if-env-changed={}", self.output.display());

            let tokens = tt.to_tokens();

            writeln!(&mut stdin, "{}", tokens)?;
            // drop stdin to close that end of the pipe
            drop(stdin);

            t.join().unwrap();
        }

        let status = cmd.status()?;
        assert!(status.success());

        Ok(())
    }
}
