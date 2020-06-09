mod cargo;
mod error;
mod manifest;

use anyhow::{bail, Context};
use futures::future::{BoxFuture, FutureExt};
use structopt::StructOpt;

use std::ffi::OsString;
use std::io::Read;
use std::path::{Path, PathBuf};

use error::Error;
use manifest::Manifest;

fn main() {
    let Opt::Winrt { subcommand } = Opt::from_args();
    let result = match subcommand {
        Subcommand::Install(i) => i.perform(),
        Subcommand::Run(r) => r.perform(),
        Subcommand::Build(b) => b.perform(),
    };
    if let Err(ref e) = result {
        eprintln!("{}: {}", console::style("error").red(), e);
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
    fn perform(&self) -> anyhow::Result<()> {
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
    fn perform(&self) -> anyhow::Result<()> {
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
    fn perform(&self) -> anyhow::Result<()> {
        let manifest = cargo::package_manifest()?;
        let local_dependencies = manifest.local_dependencies()?;
        for dep_manifest in local_dependencies {
            self.install_from_manifest(dep_manifest)?;
        }
        self.install_from_manifest(manifest)?;
        Ok(())
    }

    fn install_from_manifest(&self, manifest: Manifest) -> anyhow::Result<()> {
        let deps = manifest.get_dependency_descriptors()?;
        self.ensure_dependencies(deps)
    }

    fn ensure_dependencies(
        &self,
        dependency_descriptors: Vec<DependencyDescriptor>,
    ) -> anyhow::Result<()> {
        let dependency_descriptors = if self.force {
            dependency_descriptors
        } else {
            dependency_descriptors
                .into_iter()
                .filter_map(|d| match d.already_saved() {
                    Ok(true) => None,
                    Ok(false) => Some(Ok(d)),
                    Err(e) => Some(Err(e)),
                })
                .collect::<anyhow::Result<Vec<DependencyDescriptor>>>()?
        };

        let downloaded_deps = download_dependencies(dependency_descriptors)?;
        for dep in downloaded_deps {
            dep.save()?;
        }
        Ok(())
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

    async fn download(&self) -> anyhow::Result<Vec<u8>> {
        fn try_download(
            url: String,
            recursion_amount: u8,
        ) -> BoxFuture<'static, anyhow::Result<Vec<u8>>> {
            async move {
                if recursion_amount == 0 {
                    bail!(Error::DownloadError(
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
                    s => bail!(Error::DownloadError(
                        anyhow::anyhow!("Non-successful response: {} {}", url, s).into(),
                    )),
                }
            }
            .boxed()
        }

        try_download(self.url(), 5).await
    }

    fn already_saved(&self) -> anyhow::Result<bool> {
        Ok(self.directory_path()?.exists())
    }

    fn directory_path(&self) -> anyhow::Result<PathBuf> {
        Ok(cargo::workspace_target_path()?
            .join("nuget")
            .join(&self.name))
    }
}

struct DownloadedDependency {
    descriptor: DependencyDescriptor,
    contents: (Vec<Winmd>, Vec<Dll>),
}

impl DownloadedDependency {
    fn new(descriptor: DependencyDescriptor, bytes: Vec<u8>) -> anyhow::Result<Self> {
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

    fn read_contents(zip: &[u8]) -> anyhow::Result<(Vec<Winmd>, Vec<Dll>)> {
        let reader = std::io::Cursor::new(zip);
        let mut zip = zip::ZipArchive::new(reader)?;
        let mut winmds = Vec::new();
        let mut dlls = Vec::new();
        for i in 0..zip.len() {
            let mut file = zip.by_index(i)?;
            let path = file.sanitized_name();

            match path.extension() {
                Some(e)
                    if e == "winmd" && {
                        let parent = path.parent().and_then(Path::to_str);
                        parent == Some(r#"lib\uap10.0"#) || parent == Some(r#"ref\netstandard2.0"#)
                    } =>
                {
                    let name = path
                        .file_name()
                        .context("windmd file name is not utf-8")?
                        .to_owned();
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

    fn save(self) -> anyhow::Result<()> {
        let dep_directory = self.descriptor.directory_path()?;
        // create the dependency directory
        if !dep_directory.exists() {
            std::fs::create_dir_all(&dep_directory)
                .context("failed to create nuget dependency directory")?;
        }

        for winmd in self.winmds() {
            winmd.write(&dep_directory)?;
        }

        for dll in self.dlls() {
            dll.write(&dep_directory).unwrap();
        }

        Ok(())
    }
}

fn download_dependencies(
    deps: Vec<DependencyDescriptor>,
) -> anyhow::Result<Vec<DownloadedDependency>> {
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
    fn write(&self, dir: &Path) -> anyhow::Result<()> {
        let path = dir.join(&self.name);
        std::fs::create_dir_all(path.parent().unwrap())?;
        if !path.exists() {
            std::fs::write(&path, &self.contents)?;
        }
        for profile in &["debug", "release"] {
            let profile_path = cargo::workspace_target_path()?.join(profile);
            std::fs::create_dir_all(&profile_path)?;
            let arch = self.name.parent().unwrap();
            let dll_path = profile_path.join(&self.name.strip_prefix(&arch).unwrap());
            if arch.as_os_str() == "win-x64" && std::fs::read_link(&dll_path).is_err() {
                std::os::windows::fs::symlink_file(&path, dll_path)?;
            }
        }

        Ok(())
    }
}
