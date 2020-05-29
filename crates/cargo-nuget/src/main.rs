use cargo_toml::{Manifest, Value};
use futures::future::{BoxFuture, FutureExt};
use structopt::StructOpt;
use thiserror::Error;

use std::ffi::OsString;
use std::io::Read;
use std::path::{Path, PathBuf};

fn main() {
    let Opt::Nuget { subcommand } = Opt::from_args();
    match subcommand {
        Subcommand::Install(i) => i.perform().unwrap(),
    }
}

/// A utility for interacting with nuget packages
#[derive(StructOpt, Debug)]
#[structopt(bin_name = "cargo")]
enum Opt {
    #[structopt(name = "nuget")]
    Nuget {
        #[structopt(subcommand)]
        subcommand: Subcommand,
    },
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    Install(Install),
}

#[derive(Debug, Error)]
enum Error {
    #[error("No Cargo.toml could be found")]
    NoCargoToml,
    #[error("There was an error downloading the NuGet package {0}")]
    DownloadError(Box<dyn std::error::Error>),
    #[error("The Cargo.toml file was malformed")]
    MalformedManifest,
    #[error("There was some other error {0}")]
    Other(Box<dyn std::error::Error>),
}

#[derive(Debug, StructOpt)]
pub struct Install {}

impl Install {
    fn perform(&self) -> Result<(), Error> {
        let bytes = std::fs::read("Cargo.toml").map_err(|_| Error::NoCargoToml)?;
        let manifest = Manifest::from_slice(&bytes).map_err(|_| Error::MalformedManifest)?;
        let deps = get_deps(manifest)?;
        let downloaded_deps = download_dependencies(deps)?;
        for dep in downloaded_deps {
            let dep_directory = workspace_root()
                .join("target")
                .join("nuget")
                .join(&dep.dependency.name);
            // create the dependency directory
            std::fs::create_dir_all(&dep_directory).unwrap();
            for winmd in dep.winmds() {
                winmd.write(&dep_directory).unwrap();
            }

            for dll in dep.dlls() {
                dll.write(&dep_directory).unwrap();
            }
        }

        Ok(())
    }
}

fn workspace_root() -> PathBuf {
    // TODO: improve this
    PathBuf::new()
}

fn get_deps(manifest: Manifest) -> Result<Vec<Dependency>, Error> {
    let metadata = manifest.package.and_then(|p| p.metadata);
    match metadata {
        Some(Value::Table(mut t)) => {
            let deps = match t.remove("nuget_dependencies") {
                Some(Value::Table(deps)) => deps,
                _ => return Err(Error::MalformedManifest.into()),
            };
            deps.into_iter()
                .map(|(key, value)| match value {
                    Value::String(version) => Ok(Dependency::new(key, version)),
                    _ => Err(Error::MalformedManifest.into()),
                })
                .collect()
        }
        _ => return Err(Error::MalformedManifest.into()),
    }
}

#[derive(Debug)]
struct Dependency {
    name: String,
    version: String,
}

impl Dependency {
    fn new(name: String, version: String) -> Self {
        Self { name, version }
    }

    fn url(&self) -> String {
        format!(
            "https://www.nuget.org/api/v2/package/{}/{}",
            self.name, self.version
        )
    }

    async fn download(&self) -> Result<Vec<u8>, Error> {
        fn try_download(
            url: String,
            recursion_amount: u8,
        ) -> BoxFuture<'static, Result<Vec<u8>, Error>> {
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
                            anyhow::anyhow!("Non-successful response: {}", res.status()).into(),
                        ))
                    }
                }
            }
            .boxed()
        }

        try_download(self.url(), 5).await
    }
}

struct DownloadedDependency {
    dependency: Dependency,
    contents: (Vec<Winmd>, Vec<Dll>),
}

impl DownloadedDependency {
    fn new(dependency: Dependency, bytes: Vec<u8>) -> Result<Self, Error> {
        let contents = Self::read_contents(&bytes)?;
        Ok(Self {
            dependency,
            contents,
        })
    }

    fn winmds(&self) -> &[Winmd] {
        &self.contents.0
    }

    fn dlls(&self) -> &[Dll] {
        &self.contents.1
    }

    fn read_contents(zip: &[u8]) -> Result<(Vec<Winmd>, Vec<Dll>), Error> {
        let reader = std::io::Cursor::new(zip);
        let mut zip = zip::ZipArchive::new(reader).map_err(|e| Error::Other(Box::new(e)))?;
        let mut winmds = Vec::new();
        let mut dlls = Vec::new();
        for i in 0..zip.len() {
            let mut file = zip.by_index(i).unwrap();
            let path = file.sanitized_name();
            match path.extension() {
                Some(e)
                    if e == "winmd"
                        && path.parent().and_then(Path::to_str) == Some("lib\\uap10.0") =>
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
                            _ => panic!("Unexpected component"),
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
}

fn download_dependencies(deps: Vec<Dependency>) -> Result<Vec<DownloadedDependency>, Error> {
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
        std::fs::write(dir.join(&self.name), &self.contents)
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
            let profile_path = workspace_root().join("target").join(profile);
            std::fs::create_dir_all(&profile_path).unwrap();
            let arch = self.name.parent().unwrap();
            let dll_path = profile_path.join(&self.name.strip_prefix(&arch).unwrap());
            if arch.as_os_str() == "win10-x64" && std::fs::read_link(&dll_path).is_err() {
                println!("{} {:?}", dll_path.display(), dll_path.exists());
                std::os::windows::fs::symlink_file(&path, dll_path).unwrap();
            }
        }

        Ok(())
    }
}
