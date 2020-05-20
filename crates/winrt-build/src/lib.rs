use std::{fs, io::Write, path::Path, process::Stdio};

pub fn build<P: AsRef<Path>>(
    namespaces: &[&str],
    output: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let output = output.as_ref();
    let tr = winmd::TypeReader::from_os();

    let mut tl: winmd::TypeLimits = Default::default();
    for ns in namespaces {
        tl.insert(&tr, ns);
    }

    let ts = winmd::TypeStage::from_limits(&tr, &tl);
    let tt = ts.into_tree();

    fs::create_dir_all(output.parent().unwrap())?;
    let mut f = fs::File::create(output)?;

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
        writeln!(&mut stdin, "#![allow(overflowing_literals)]").unwrap();

        let tokens = tt.to_tokens();

        writeln!(&mut stdin, "{}", tokens).unwrap();
        // drop stdin to close that end of the pipe
        drop(stdin);

        t.join().unwrap();
    }

    let status = cmd.status()?;
    assert!(status.success());

    Ok(())
}
