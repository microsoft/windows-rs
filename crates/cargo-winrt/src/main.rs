mod cargo;
mod error;

use cargo_toml::{Manifest, Value};
use futures::future::{BoxFuture, FutureExt};
use structopt::StructOpt;

use std::ffi::OsString;
use std::io::Read;
use std::path::{Path, PathBuf};

use error::Error;

fn main() {
    let Opt::Winrt { subcommand } = Opt::from_args();
    match subcommand {
        Subcommand::Install(i) => i.perform().unwrap(),
        Subcommand::Run(r) => r.perform().unwrap(),
        Subcommand::Build(b) => b.perform().unwrap(),
    }
}

/// A utility for interacting with nuget packages
#[derive(StructOpt, Debug)]
#[structopt(bin_name = "cargo")]
enum Opt {
    #[structopt(name = "winrt")]
    Winrt {
        #[structopt(subcommand)]
        subcommand: Subcommand,
    },
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    Install(Install),
    Build(Build),
    Run(Run),
}

#[derive(Debug, StructOpt)]
pub struct Build {
    #[structopt(short, long)]
    force: bool,
}

impl Build {
    fn perform(&self) -> Result<(), Error> {
        let install = Install { force: self.force };
        install.perform()?;
        cargo::build()
    }
}

#[derive(Debug, StructOpt)]
pub struct Run {
    #[structopt(short, long)]
    force: bool,
}

impl Run {
    fn perform(&self) -> Result<(), Error> {
        let install = Install { force: self.force };
        install.perform()?;
        cargo::run()
    }
}

#[derive(Debug, StructOpt)]
pub struct Install {
    #[structopt(short, long)]
    force: bool,
}

impl Install {
    fn perform(&self) -> error::Result<()> {
        let manifest = cargo::workspace_manifest()?;
        let deps = get_dependency_descriptors(manifest)?;

        self.ensure_dependencies(deps)?;

        Ok(())
    }

    fn ensure_dependencies(
        &self,
        dependency_descriptors: Vec<DependencyDescriptor>,
    ) -> error::Result<()> {
        let dependency_descriptors = if self.force {
            dependency_descriptors
        } else {
            dependency_descriptors
                .into_iter()
                .filter(|d| !d.already_saved())
                .collect()
        };

        let downloaded_deps = download_dependencies(dependency_descriptors)?;
        for dep in downloaded_deps {
            dep.save()?;
        }
        Ok(())
    }
}

fn get_dependency_descriptors(manifest: Manifest) -> error::Result<Vec<DependencyDescriptor>> {
    let metadata = manifest.package.and_then(|p| p.metadata);
    match metadata {
        Some(Value::Table(mut t)) => {
            let mut deps = match t.remove("winrt") {
                Some(Value::Table(deps)) => deps,
                None => return Ok(Vec::new()),
                _ => return Err(Error::MalformedManifest.into()),
            };
            let deps = match deps.remove("dependencies") {
                Some(Value::Table(deps)) => deps,
                None => return Ok(Vec::new()),
                _ => return Err(Error::MalformedManifest.into()),
            };
            deps.into_iter()
                .map(|(key, value)| match value {
                    Value::String(version) => Ok(DependencyDescriptor::new(key, version)),
                    _ => Err(Error::MalformedManifest.into()),
                })
                .collect()
        }
        _ => return Err(Error::MalformedManifest.into()),
    }
}

#[derive(Debug)]
struct DependencyDescriptor {
    name: String,
    version: String,
}

impl DependencyDescriptor {
    fn new(name: String, version: String) -> Self {
        Self { name, version }
    }

    fn url(&self) -> String {
        format!(
            "https://www.nuget.org/api/v2/package/{}/{}",
            self.name, self.version
        )
    }

    async fn download(&self) -> error::Result<Vec<u8>> {
        fn try_download(
            url: String,
            recursion_amount: u8,
        ) -> BoxFuture<'static, error::Result<Vec<u8>>> {
            async move {
                if recursion_amount == 0 {
                    return Err(Error::DownloadError(
                        anyhow::anyhow!("Too many redirects").into(),
                    ));
                }
                let res = reqwest::get(&url)
                    .await
                    .map_err(|e| Error::DownloadError(e.into()))?;
                match res.status().into() {
                    200u16 => {
                        let bytes = res
                            .bytes()
                            .await
                            .map_err(|e| Error::DownloadError(e.into()))?;
                        Ok(bytes.into_iter().collect())
                    }
                    302 => {
                        let headers = res.headers();
                        let redirect_url = headers.get("Location").unwrap();

                        let url = redirect_url.to_str().unwrap();

                        try_download(url.to_owned(), recursion_amount - 1).await
                    }
                    _ => {
                        return Err(Error::DownloadError(
                            anyhow::anyhow!("Non-successful response: {} {}", url, res.status())
                                .into(),
                        ))
                    }
                }
            }
            .boxed()
        }

        try_download(self.url(), 5).await
    }

    fn already_saved(&self) -> bool {
        self.directory_path().exists()
    }

    fn directory_path(&self) -> PathBuf {
        cargo::workspace_target_path()
            .join("nuget")
            .join(&self.name)
    }
}

struct DownloadedDependency {
    descriptor: DependencyDescriptor,
    contents: (Vec<Winmd>, Vec<Dll>),
}

impl DownloadedDependency {
    fn new(descriptor: DependencyDescriptor, bytes: Vec<u8>) -> error::Result<Self> {
        let contents = Self::read_contents(&bytes)?;
        Ok(Self {
            descriptor,
            contents,
        })
    }

    fn winmds(&self) -> &[Winmd] {
        &self.contents.0
    }

    fn dlls(&self) -> &[Dll] {
        &self.contents.1
    }

    fn read_contents(zip: &[u8]) -> error::Result<(Vec<Winmd>, Vec<Dll>)> {
        let reader = std::io::Cursor::new(zip);
        let mut zip = zip::ZipArchive::new(reader).map_err(|e| Error::Other(Box::new(e)))?;
        let mut winmds = Vec::new();
        let mut dlls = Vec::new();
        for i in 0..zip.len() {
            let mut file = zip.by_index(i).unwrap();
            let path = file.sanitized_name();

            match path.extension() {
                Some(e)
                    if e == "winmd" && {
                        let parent = path.parent().and_then(Path::to_str);
                        parent == Some(r#"lib\uap10.0"#) || parent == Some(r#"ref\netstandard2.0"#)
                    } =>
                {
                    let name = path.file_name().unwrap().to_owned();
                    let mut contents = Vec::with_capacity(file.size() as usize);

                    if let Err(e) = file.read_to_end(&mut contents) {
                        eprintln!("Could not read winmd file: {:?}", e);
                        continue;
                    }
                    winmds.push(Winmd { name, contents });
                }
                Some(e) if e == "dll" && path.starts_with("runtimes") => {
                    let name: PathBuf = path
                        .components()
                        .filter(|c| match c {
                            std::path::Component::Normal(p) => *p != "native" && *p != "runtimes",
                            _ => false,
                        })
                        .collect();
                    let mut contents = Vec::with_capacity(file.size() as usize);

                    if let Err(e) = file.read_to_end(&mut contents) {
                        eprintln!("Could not read dll: {:?}", e);
                        continue;
                    }
                    dlls.push(Dll { name, contents });
                }
                _ => {}
            }
        }
        Ok((winmds, dlls))
    }

    fn save(self) -> error::Result<()> {
        let dep_directory = self.descriptor.directory_path();
        // create the dependency directory
        if !dep_directory.exists() {
            std::fs::create_dir_all(&dep_directory).unwrap();
        }

        for winmd in self.winmds() {
            winmd.write(&dep_directory).unwrap();
        }

        for dll in self.dlls() {
            dll.write(&dep_directory).unwrap();
        }

        Ok(())
    }
}

fn download_dependencies(
    deps: Vec<DependencyDescriptor>,
) -> error::Result<Vec<DownloadedDependency>> {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let results = deps.into_iter().map(|dep| async move {
            let bytes = dep.download().await?;
            Ok(DownloadedDependency::new(dep, bytes)?)
        });

        futures::future::try_join_all(results).await
    })
}

struct Winmd {
    name: OsString,
    contents: Vec<u8>,
}

impl Winmd {
    fn write(&self, dir: &Path) -> std::io::Result<()> {
        let path = dir.join(&self.name);
        if !path.exists() {
            return std::fs::write(dir.join(&self.name), &self.contents);
        }
        Ok(())
    }
}

struct Dll {
    name: PathBuf,
    contents: Vec<u8>,
}

impl Dll {
    fn write(&self, dir: &Path) -> std::io::Result<()> {
        let path = dir.join(&self.name);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if !path.exists() {
            std::fs::write(&path, &self.contents).unwrap();
        }
        for profile in &["debug", "release"] {
            let profile_path = cargo::workspace_target_path().join(profile);
            std::fs::create_dir_all(&profile_path).unwrap();
            let arch = self.name.parent().unwrap();
            let dll_path = profile_path.join(&self.name.strip_prefix(&arch).unwrap());
            if arch.as_os_str() == "win-x64" && std::fs::read_link(&dll_path).is_err() {
                std::os::windows::fs::symlink_file(&path, dll_path).unwrap();
            }
        }

        Ok(())
    }
}
