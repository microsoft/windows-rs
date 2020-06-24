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
        eprintln!("{}: {}", console::style("error").red().bold(), e);
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
    #[structopt(short, long)]
    verbose: bool,
}

impl Build {
    fn perform(&self) -> anyhow::Result<()> {
        let install = Install {
            force: self.force,
            verbose: self.verbose,
        };
        install.perform()?;
        cargo::build()
    }
}

#[derive(Debug, StructOpt)]
pub struct Run {
    #[structopt(short, long)]
    force: bool,
    #[structopt(short, long)]
    verbose: bool,
}

impl Run {
    fn perform(&self) -> anyhow::Result<()> {
        let install = Install {
            force: self.force,
            verbose: self.verbose,
        };
        install.perform()?;
        cargo::run()
    }
}

#[derive(Debug, StructOpt)]
pub struct Install {
    #[structopt(short, long)]
    force: bool,
    #[structopt(short, long)]
    verbose: bool,
}

use once_cell::sync::OnceCell;
static VERBOSITY: OnceCell<bool> = OnceCell::new();

#[inline(always)]
fn verbose() -> bool {
    VERBOSITY.get().copied().unwrap_or(false)
}

macro_rules! debug {
    ($($arg:tt)*) => {
        if verbose() {
            eprintln!("\t{}", format!($($arg)*));
        }
    };
}

impl Install {
    fn perform(&self) -> anyhow::Result<()> {
        let _ = VERBOSITY.set(self.verbose);
        let manifest = cargo::package_manifest()?;
        let local_dependencies = manifest.local_dependencies()?;
        for dep_manifest in local_dependencies {
            self.install_from_manifest(dep_manifest)?;
        }
        self.install_from_manifest(manifest)?;
        Ok(())
    }

    fn install_from_manifest(&self, manifest: Manifest) -> anyhow::Result<()> {
        debug!(
            "{} dependencies for {}",
            console::style("Resolving").green().bold(),
            manifest.package_name()
        );
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
        debug!(
            "{} {} nuget dependencies",
            console::style("Installing").green().bold(),
            dependency_descriptors.len()
        );

        let deps = self.get_dependencies(dependency_descriptors)?;
        for dep in deps {
            dep.save()?;
        }
        Ok(())
    }

    fn get_dependencies(
        &self,
        deps: Vec<DependencyDescriptor>,
    ) -> anyhow::Result<Vec<ResolvedDependency>> {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let results = deps.into_iter().map(|dep| async move {
                println!(
                    "\t{}: {}",
                    console::style("Fetching").green().bold(),
                    dep.name()
                );
                let raw = dep.get().await?;
                Ok(ResolvedDependency::new(dep, raw)?)
            });

            futures::future::try_join_all(results).await
        })
    }
}

#[derive(Debug)]
enum DependencyDescriptor {
    NugetOrg { name: String, version: String },
    Url { name: String, url: reqwest::Url },
    Local { name: String, path: PathBuf },
}

impl DependencyDescriptor {
    async fn get(&self) -> anyhow::Result<RawNuget> {
        match self {
            DependencyDescriptor::NugetOrg { name, version } => {
                let url = format!("https://www.nuget.org/api/v2/package/{}/{}", name, version);
                let bytes = try_download(url, 5).await?;
                Ok(RawNuget::Zipped {
                    bytes,
                    name: name.clone(),
                })
            }
            DependencyDescriptor::Url { url, name } => {
                let bytes = try_download(url.as_str().to_owned(), 5).await?;

                Ok(RawNuget::Zipped {
                    bytes,
                    name: name.clone(),
                })
            }
            DependencyDescriptor::Local { path: p, name } => {
                let mut path = cargo::package_manifest_path()?
                    .parent()
                    .expect("package mainfest must have parent path")
                    .to_owned();
                path.extend(p);

                Ok(RawNuget::Unzipped {
                    path,
                    name: name.clone(),
                })
            }
        }
    }

    fn name(&self) -> &str {
        match self {
            DependencyDescriptor::NugetOrg { name, .. } => name,
            DependencyDescriptor::Url { name, .. } => name,
            DependencyDescriptor::Local { name, .. } => name,
        }
    }

    fn already_saved(&self) -> anyhow::Result<bool> {
        Ok(self.directory_path()?.exists())
    }

    fn directory_path(&self) -> anyhow::Result<PathBuf> {
        Ok(cargo::workspace_target_path()?
            .join("nuget")
            .join(&self.name()))
    }
}

fn try_download(url: String, recursion_amount: u8) -> BoxFuture<'static, anyhow::Result<Vec<u8>>> {
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
                debug!(" retrieved data from {}", url);
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

enum RawNuget {
    Zipped { name: String, bytes: Vec<u8> },
    Unzipped { name: String, path: PathBuf },
}

impl RawNuget {
    fn contents(&self) -> anyhow::Result<(Vec<Winmd>, Vec<Dll>)> {
        debug!(" starting extraction of '{}'", self.name());
        match self {
            RawNuget::Zipped { bytes, .. } => unzip(bytes),
            RawNuget::Unzipped { path, .. } => unpack(path),
        }
    }

    fn name(&self) -> &str {
        match self {
            RawNuget::Zipped { name, .. } => name,
            RawNuget::Unzipped { name, .. } => name,
        }
    }
}

fn unpack<P: AsRef<Path>>(path: P) -> anyhow::Result<(Vec<Winmd>, Vec<Dll>)> {
    fn recursively_extract_files(
        root: &Path,
        path: PathBuf,
        winmds: &mut Vec<Winmd>,
        dlls: &mut Vec<Dll>,
    ) -> anyhow::Result<()> {
        if path.is_dir() {
            let dir = std::fs::read_dir(&path)
                .with_context(|| format!("could not read the nuget folder: {}", path.display()))?;

            for entry in dir {
                let entry = entry?;
                let path = entry.path();
                recursively_extract_files(root, path, winmds, dlls)?;
            }
        } else {
            let file = std::fs::File::open(&path)
                .with_context(|| format!("could not read the nuget file: {}", path.display()))?;
            let file_size = file.metadata().map(|md| md.len()).unwrap_or(1024);
            let path = path
                .strip_prefix(root)
                .expect("path must have root as a prefix");
            extract_files(path, file, file_size, winmds, dlls)?;
        }
        Ok(())
    }

    let mut winmds = Vec::new();
    let mut dlls = Vec::new();
    recursively_extract_files(
        path.as_ref(),
        path.as_ref().to_path_buf(),
        &mut winmds,
        &mut dlls,
    )?;
    Ok((winmds, dlls))
}

fn unzip(bytes: &[u8]) -> anyhow::Result<(Vec<Winmd>, Vec<Dll>)> {
    let reader = std::io::Cursor::new(bytes);
    let mut zip = zip::ZipArchive::new(reader)?;
    let mut winmds = Vec::new();
    let mut dlls = Vec::new();
    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        let path = file.sanitized_name();
        let file_size = file.size();
        extract_files(&path, file, file_size, &mut winmds, &mut dlls)?;
    }
    Ok((winmds, dlls))
}

fn extract_files<F: Read>(
    path: &Path,
    mut file: F,
    file_size: u64,
    winmds: &mut Vec<Winmd>,
    dlls: &mut Vec<Dll>,
) -> anyhow::Result<()> {
    debug!("   searching zip file: {:?}", path.display());
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
            debug!(" {} winmd file {:?}", console::style("found").green(), name);
            let mut contents = Vec::with_capacity(file_size as usize);

            if let Err(e) = file.read_to_end(&mut contents) {
                eprintln!(
                    "{}: could not read winmd file {}",
                    console::style("warning").red(),
                    e
                );
                return Ok(());
            }
            winmds.push(Winmd { name, contents });
        }
        Some(e) if e == "dll" && path.starts_with("runtimes") => {
            let mut name: Option<OsString> = None;
            let mut arch: Option<OsString> = None;
            for component in path.components() {
                match component {
                    std::path::Component::Normal(s) if s.to_string_lossy().starts_with("win10") => {
                        arch = Some(s.to_owned());
                    }
                    std::path::Component::Normal(s) if s.to_string_lossy().ends_with("dll") => {
                        name = Some(s.to_owned());
                    }
                    std::path::Component::Normal(s) if s.to_string_lossy() == "debug" => {
                        // skip debug dlls
                        return Ok(());
                    }
                    _ => {}
                }
            }
            let (name, arch) = match (name, arch) {
                (Some(n), Some(a)) => (n, a),
                _ => {
                    return Err(anyhow::anyhow!(
                        "{} is not a valid dll path",
                        path.display()
                    ))
                }
            };
            debug!(
                "{} dll {:?} with arch {:?} at path {}",
                console::style("found").green(),
                name,
                arch,
                path.display()
            );
            let mut contents = Vec::with_capacity(file_size as usize);

            if let Err(e) = file.read_to_end(&mut contents) {
                eprintln!(
                    "{}: could not read dll file {}",
                    console::style("warning").red(),
                    e
                );
                return Ok(());
            }
            dlls.push(Dll {
                name,
                arch,
                contents,
            });
        }
        _ => {}
    }
    Ok(())
}

struct ResolvedDependency {
    descriptor: DependencyDescriptor,
    contents: (Vec<Winmd>, Vec<Dll>),
}

impl ResolvedDependency {
    fn new(descriptor: DependencyDescriptor, raw: RawNuget) -> anyhow::Result<Self> {
        let contents = raw.contents()?;
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

    fn save(self) -> anyhow::Result<()> {
        debug!(
            "{} {} winmd files and {} dlls",
            console::style("Saving").green().bold(),
            self.winmds().len(),
            self.dlls().len(),
        );
        let dep_directory = self.descriptor.directory_path()?;
        // create the dependency directory
        if !dep_directory.exists() {
            std::fs::create_dir_all(&dep_directory)
                .context("failed to create nuget dependency directory")?;
        }

        for winmd in self.winmds() {
            debug!(
                "writing winmd file {:?} into {}",
                winmd.name,
                dep_directory.display()
            );
            winmd.write(&dep_directory)?;
        }

        for dll in self.dlls() {
            debug!(
                "writing dll file {:?} into {}",
                dll.name,
                dep_directory.display()
            );
            dll.write(&dep_directory).unwrap();
        }

        Ok(())
    }
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
    name: OsString,
    arch: OsString,
    contents: Vec<u8>,
}

impl Dll {
    fn write(&self, dir: &Path) -> anyhow::Result<()> {
        let proper_arch = self.arch.as_os_str() == ARCH;
        if !proper_arch {
            debug!("   not creating symlink for {:?} because of differing architecture to host architecture: {:?} != {:?}", self.name, self.arch, ARCH);
            return Ok(());
        }
        let path = dir.join(&self.name);
        std::fs::create_dir_all(path.parent().unwrap())?;
        if !path.exists() {
            std::fs::write(&path, &self.contents)?;
        }
        for profile in &["debug", "release"] {
            let profile_path = cargo::workspace_target_path()?.join(profile);
            std::fs::create_dir_all(&profile_path)?;
            let dll_path = profile_path.join(&self.name);
            if std::fs::read_link(&dll_path).is_err() {
                debug!(
                    "   creating symlink for {:?} in {}: '{}' <-> '{}'",
                    self.name,
                    profile,
                    path.display(),
                    dll_path.display()
                );
                std::os::windows::fs::symlink_file(&path, dll_path)?;
            } else {
                debug!(
                    "   not creating symlink for {:?} in {} because it already exists",
                    self.name, profile
                );
            }
        }

        Ok(())
    }
}

#[cfg(target_arch = "x86_64")]
const ARCH: &str = "win10-x64";
#[cfg(target_arch = "x86")]
const ARCH: &str = "win10-x86";
#[cfg(target_arch = "arm")]
const ARCH: &str = "win10-arm";
#[cfg(target_arch = "aarch64")]
const ARCH: &str = "win10-arm64";
