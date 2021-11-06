use std::fs;
use std::path::Path;

#[macro_export]
macro_rules! ws_path {
    ($s:literal) => {
        ::std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
                                                              .join($s)
    };
}

pub fn reformat(f: &Path)
{
    let rustfmt_conf = ws_path!("rustfmt.toml");

    let cmd =
        xshell::cmd!("rustfmt --edition=2021 --config-path {rustfmt_conf} {f}");

    cmd.run().unwrap();
}

pub fn ensure_file_contents(path: &Path, contents: &str)
{
    let parent = path.parent().unwrap();
    if !parent.exists() {
        fs::create_dir_all(parent).unwrap();
    }

    fs::write(path, contents).unwrap();
}

pub fn read_file<P: AsRef<Path>>(p: P) -> String
{
    fs::read_to_string(p.as_ref()).unwrap()
}

pub fn add_preamble(file: &str, generated_by: &str) -> String
{
    format!(
            indoc::indoc! {r#"
        // This file was generated by {}.

        #![allow(unused_imports)]
        
        {}
        "#},
            generated_by, file
    )
}
