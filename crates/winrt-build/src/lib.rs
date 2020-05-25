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
    pub fn insert_namespaces<T: Iterator<Item = String>>(mut self, namespaces: T) -> Self {
        self.namespaces.extend(namespaces);
        self
    }

    pub fn output<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.output = path.as_ref().to_owned();
        self
    }

    pub fn build(&self) -> Result<(), Box<dyn std::error::Error>> {
        let tr = match self.dependencies {
            Dependencies::Os => winmd::TypeReader::from_os(),
            _ => todo!(),
        };

        let mut tl = winmd::TypeLimits::default();
        for ns in &self.namespaces {
            tl.insert(&tr, &ns);
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

            // workaround for, well, overflowing literals
            writeln!(&mut stdin, "#![allow(overflowing_literals)]")?;

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
