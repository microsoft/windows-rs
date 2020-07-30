mod cargo;
mod error;
mod manifest;

use error::Error;
use manifest::Manifest;

use anyhow::Context;
use curl::easy::Easy;

use std::ffi::OsString;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

fn main() {
    if let Err(i) = run() {
        std::process::exit(i);
    }
}

macro_rules! cmd_err {
    ($($arg:tt)*) => {
        eprintln!("{}: {}", console::style("error").red().bold(), format!($($arg)*));
    };
}

fn run() -> Result<(), i32> {
    let subcommand = match parse_args() {
        Ok(s) => s,
        Err(e) => {
            match e {
                ArgsError::MissingSubcommand => cmd_err!("missing subcommand"),
                ArgsError::NoSuchSubcommand(c) => cmd_err!("no such subcommand: {}", c),
                ArgsError::Pico(pico_args::Error::UnusedArgsLeft(args)) => {
                    cmd_err!("too many arguments supplied: {:?}", args)
                }
                ArgsError::Pico(e) => cmd_err!("there was an error: {}", e),
            };
            let _ = print_help();
            return Err(1);
        }
    };
    let result = match subcommand {
        Subcommand::Install(i) => i.perform(),
        Subcommand::Run(r) => r.perform(),
        Subcommand::Build(b) => b.perform(),
        Subcommand::Help(h) => h.perform(),
    };
    if let Err(ref e) = result {
        cmd_err!("{}", e);
        return Err(1);
    }
    Ok(())
}

fn parse_args() -> Result<Subcommand, ArgsError> {
    let mut args = pico_args::Arguments::from_env();
    let mut subcommand = args.subcommand()?;
    if subcommand.as_deref() == Some("winrt") {
        // presumably running in cargo subcommand mode,
        // so the subcommand is actually the 3rd command line arg
        subcommand = args.subcommand()?;
    }

    if args.contains(["-h", "--help"]) {
        return Ok(Subcommand::Help(Help { subcommand }));
    }

    let verbose = args.contains(["-v", "--verbose"]);
    let force = args.contains(["-f", "--force"]);
    let subcommand = match subcommand.as_deref() {
        Some("install") => {
            args.finish()?;
            Subcommand::Install(Install { verbose, force })
        }
        Some("run") => {
            args.finish()?;
            Subcommand::Run(Run { verbose, force })
        }
        Some("build") => {
            args.finish()?;
            Subcommand::Build(Build { verbose, force })
        }
        Some("help") => {
            let subcommand = args.free()?.first().map(String::to_owned);
            Subcommand::Help(Help { subcommand })
        }
        Some(_) => return Err(ArgsError::NoSuchSubcommand(subcommand.unwrap())),
        None => return Err(ArgsError::MissingSubcommand),
    };

    Ok(subcommand)
}

fn print_help() {
    println!(
        r#"
This utility assists with WinRT operations on the current crate.

USAGE:
    cargo winrt [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    build      Installs the WinRT dependencies and performs a `cargo build`
    install    Installs the WinRT dependencies of the package locally
    run        Installs the WinRT dependencies and performs a `cargo run`

See 'cargo winrt help <command>' for more information on a specific command.
            "#
    );
}

enum ArgsError {
    Pico(pico_args::Error),
    NoSuchSubcommand(String),
    MissingSubcommand,
}

impl std::convert::From<pico_args::Error> for ArgsError {
    fn from(e: pico_args::Error) -> Self {
        ArgsError::Pico(e)
    }
}

#[derive(Debug)]
enum Subcommand {
    Install(Install),
    Build(Build),
    Run(Run),
    Help(Help),
}

#[derive(Debug)]
pub struct Build {
    force: bool,
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

    fn print_help() {
        println!(
            r#"
Installs the WinRT dependencies and performs a `cargo build`

USAGE:
    cargo winrt build [OPTIONS]

OPTIONS:
    -f, --force      Forces reinstallation of WinRT dependencies
    -v, --verbose    Use verbose output
            "#
        );
    }
}

#[derive(Debug)]
pub struct Run {
    force: bool,
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

    fn print_help() {
        println!(
            r#"
Installs the WinRT dependencies and performs a `cargo run`

USAGE:
    cargo winrt run [OPTIONS]

OPTIONS:
    -f, --force      Forces reinstallation of WinRT dependencies
    -v, --verbose    Use verbose output
            "#
        );
    }
}

#[derive(Debug)]
pub struct Install {
    force: bool,
    verbose: bool,
}

use once_cell::sync::OnceCell;
static VERBOSITY: OnceCell<bool> = OnceCell::new();

#[inline(always)]
fn verbose() -> bool {
    VERBOSITY.get().copied().unwrap_or(false)
}

/// Formats the elapsed time to match Cargo's output.
fn elapsed(duration: Duration) -> String {
    let secs = duration.as_secs();

    if secs >= 60 {
        format!("{}m {:02}s", secs / 60, secs % 60)
    } else {
        format!("{}.{:02}s", secs, duration.subsec_nanos() / 10_000_000)
    }
}

macro_rules! print_status {
    ($status:expr, $message:expr) => ({
        println!("{:>12} {}", console::style($status).green().bold(), $message);
    });
    ($status:expr, $message_fmt:expr, $($message_args:tt)*) => ({
        println!("{:>12} {}", console::style($status).green().bold(), format!($message_fmt, $($message_args)*));
    });
}

macro_rules! print_verbose_status {
    ($status:expr, $message:expr) => ({
        if verbose() {
            eprintln!("{:>12} {}", console::style($status).green().bold(), $message);
        }
    });
    ($status:expr, $message_fmt:expr, $($message_args:tt)*) => ({
        if verbose() {
            eprintln!("{:>12} {}", console::style($status).green().bold(), format!($message_fmt, $($message_args)*));
        }
    });
}

impl Install {
    fn perform(&self) -> anyhow::Result<()> {
        let start_time = Instant::now();

        let _ = VERBOSITY.set(self.verbose);
        let manifest = cargo::package_manifest()?;
        let local_dependencies = manifest.local_dependencies()?;
        for dep_manifest in local_dependencies {
            self.install_from_manifest(dep_manifest)?;
        }
        self.install_from_manifest(manifest)?;

        let time_elapsed = elapsed(start_time.elapsed());
        print_status!(
            "Finished",
            "installing WinRT dependencies in {}",
            time_elapsed
        );

        Ok(())
    }

    fn install_from_manifest(&self, manifest: Manifest) -> anyhow::Result<()> {
        print_verbose_status!("Resolving", manifest.package_name());
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

        print_verbose_status!(
            "Installing",
            "{} nuget dependencies",
            dependency_descriptors.len()
        );

        let deps = self.get_dependencies(dependency_descriptors)?;
        for dep in deps {
            dep.save(self.force)?;
        }
        Ok(())
    }

    fn get_dependencies(
        &self,
        deps: Vec<DependencyDescriptor>,
    ) -> anyhow::Result<Vec<ResolvedDependency>> {
        deps.into_iter()
            .map(|dep| {
                print_status!("Fetching", dep.name());
                let raw = dep.get()?;
                Ok(ResolvedDependency::new(dep, raw)?)
            })
            .collect()
    }

    fn print_help() {
        println!(
            r#"
Installs the WinRT dependencies of the package locally

USAGE:
    cargo winrt install [OPTIONS]

OPTIONS:
    -f, --force      Forces the installation, even if up to date
    -v, --verbose    Use verbose output
            "#
        );
    }
}

#[derive(Debug)]
struct Help {
    subcommand: Option<String>,
}

impl Help {
    fn perform(&self) -> anyhow::Result<()> {
        match &self.subcommand {
            Some(subcommand) => match subcommand.as_str() {
                "install" => Install::print_help(),
                "build" => Build::print_help(),
                "run" => Run::print_help(),
                "help" => Help::print_help(),
                unknown => {
                    eprintln!("error: The subcommand '{}' wasn't recognized", unknown);
                    Help::print_help()
                }
            },
            None => crate::print_help(),
        }
        Ok(())
    }

    fn print_help() {
        println!(
            r#"
Prints this message or the help of the given subcommand

USAGE:
    cargo winrt help [SUBCOMMAND]
"#
        );
    }
}

#[derive(Debug)]
enum DependencyDescriptor {
    NugetOrg { name: String, version: String },
    Url { name: String, url: String },
    Local { name: String, path: PathBuf },
}

impl DependencyDescriptor {
    fn get(&self) -> anyhow::Result<RawNuget> {
        match self {
            DependencyDescriptor::NugetOrg { name, version } => {
                let url = format!("https://www.nuget.org/api/v2/package/{}/{}", name, version);
                let bytes = try_download(url)?;
                Ok(RawNuget::Zipped {
                    bytes,
                    name: name.clone(),
                })
            }
            DependencyDescriptor::Url { url, name } => {
                let bytes = try_download(url.as_str().to_owned())?;

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

    fn version(&self) -> &str {
        match self {
            DependencyDescriptor::NugetOrg { version, .. } => version,
            DependencyDescriptor::Url { .. } => "unknown",
            DependencyDescriptor::Local { .. } => "unknown",
        }
    }

    fn already_saved(&self) -> anyhow::Result<bool> {
        Ok(self.directory_path()?.exists())
    }

    fn directory_path(&self) -> anyhow::Result<PathBuf> {
        Ok(cargo::workspace_target_path()?.join("nuget").join(&format!(
            "{}-{}",
            self.name(),
            self.version()
        )))
    }
}

fn try_download(url: String) -> anyhow::Result<Vec<u8>> {
    let mut handle = Easy::new();
    handle
        .url(&url)
        .map_err(|e| Error::DownloadError(e.into()))?;
    // Instruct curl to follow redirections
    handle
        .follow_location(true)
        .map_err(|e| Error::DownloadError(e.into()))?;
    // Optionally set the max number of redirects
    handle
        .max_redirections(5)
        .map_err(|e| Error::DownloadError(e.into()))?;

    let mut bytes = Vec::new();
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|d| {
                bytes.extend_from_slice(d);
                Ok(d.len())
            })
            .map_err(|e| Error::DownloadError(e.into()))?;
        print_verbose_status!("Requesting", &url);
        transfer
            .perform()
            .map_err(|e| Error::DownloadError(e.into()))?;
    }

    match handle
        .response_code()
        .map_err(|e| Error::DownloadError(e.into()))?
    {
        200 => Ok(bytes),
        code => Err(Error::DownloadError(curl::Error::new(code as _).into()).into()),
    }
}

enum RawNuget {
    Zipped { name: String, bytes: Vec<u8> },
    Unzipped { name: String, path: PathBuf },
}

impl RawNuget {
    fn contents(&self) -> anyhow::Result<(Vec<Winmd>, Vec<Dll>)> {
        print_verbose_status!("Starting", "extraction of '{}'", self.name());
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
    print_verbose_status!("Searching", "zip file: {}", path.display());
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
            print_verbose_status!("Found", "winmd file: {:?}", name);
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
                    std::path::Component::Normal(s) if s.to_string_lossy().starts_with("win") => {
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
                (_, _) => {
                    return Err(anyhow::anyhow!(
                        "{} is not a valid dll path",
                        path.display()
                    ));
                }
            };

            print_verbose_status!(
                "Found",
                "dll {:?} with arch {:?} at path {}",
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

    fn save(self, force: bool) -> anyhow::Result<()> {
        print_verbose_status!(
            "Saving",
            "{} winmd files and {} dlls",
            self.winmds().len(),
            self.dlls().len()
        );

        let dep_directory = self.descriptor.directory_path()?;
        // delete any old directories
        if let Ok(parent_dir) = std::fs::read_dir(dep_directory.parent().unwrap()) {
            for entry in parent_dir {
                let entry = match entry {
                    Ok(e) => e,
                    Err(_) => continue,
                };
                let name = entry.file_name();
                let name = name.to_str();
                if entry.path().is_dir()
                    && name
                        .map(|s| {
                            s.starts_with(self.descriptor.name())
                                && !s.ends_with(self.descriptor.version())
                        })
                        .unwrap_or(false)
                {
                    let _ = std::fs::remove_dir_all(&entry.path());
                }
            }
        }
        // create the dependency directory
        if !dep_directory.exists() {
            std::fs::create_dir_all(&dep_directory)
                .context("failed to create nuget dependency directory")?;
        }

        for winmd in self.winmds() {
            print_verbose_status!(
                "Writing",
                "winmd file {:?} into {}",
                winmd.name,
                dep_directory.display()
            );
            winmd.write(&dep_directory, force)?;
        }

        for dll in self.dlls() {
            print_verbose_status!(
                "Writing",
                "dll file {:?} into {}",
                dll.name,
                dep_directory.display(),
            );
            dll.write(&dep_directory, force).unwrap();
        }

        Ok(())
    }
}

struct Winmd {
    name: OsString,
    contents: Vec<u8>,
}

impl Winmd {
    fn write(&self, dir: &Path, force: bool) -> std::io::Result<()> {
        let path = dir.join(&self.name);
        if !path.exists() || force {
            return std::fs::write(path, &self.contents);
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
    fn write(&self, dir: &Path, force: bool) -> anyhow::Result<()> {
        let proper_arch = ARCHES.contains(&&*self.arch.to_string_lossy());
        if !proper_arch {
            print_verbose_status!(
                "",
                "   not creating symlink for {:?} because of differing architecture to host architecture: {:?} not in {:?}",
                self.name,
                self.arch,
                ARCHES);
            return Ok(());
        }
        let path = dir.join(&self.name);
        std::fs::create_dir_all(path.parent().unwrap())?;
        if !path.exists() || force {
            std::fs::write(&path, &self.contents)?;
        }
        for profile in &["debug", "release"] {
            let profile_path = cargo::workspace_target_path()?.join(profile);
            std::fs::create_dir_all(&profile_path)?;
            let dll_path = profile_path.join(&self.name);
            if std::fs::read_link(&dll_path).is_err() {
                print_verbose_status!(
                    "Creating",
                    "symlink for {:?} in {}: '{}' <-> '{}'",
                    self.name,
                    profile,
                    path.display(),
                    dll_path.display()
                );
                std::os::windows::fs::symlink_file(&path, dll_path)?;
            } else {
                print_verbose_status!(
                    "",
                    "   not creating symlink for {:?} in {} because it already exists",
                    self.name,
                    profile
                );
            }
        }

        Ok(())
    }
}

#[cfg(target_arch = "x86_64")]
const ARCHES: &[&str] = &["win10-x64", "win-x64"];
#[cfg(target_arch = "x86")]
const ARCHES: &[&str] = &["win10-x86", "win-x86"];
#[cfg(target_arch = "arm")]
const ARCHES: &[&str] = &["win10-arm", "win-arm"];
#[cfg(target_arch = "aarch64")]
const ARCHES: &[&str] = &["win10-arm64", "win-arm64"];
