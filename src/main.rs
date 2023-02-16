mod formats;
mod ignore;
mod special_ignore;

use anyhow::Result;
use clap::Parser;
use formats::{CheckResult, Format};
use ignore::Ignore;
use std::{fs, path::Path};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The root path to lint
    #[arg(short, long, default_value = ".")]
    path: String,

    /// Expected formatting for files
    #[arg(value_enum, short, long, default_value_t = Format::KebabCase)]
    formatting: Format,
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.path);

    let gitignore_path = path.join(".gitignore");
    let ignore = ignore::Ignore::new(&gitignore_path).unwrap();

    let mut invalid_entries: Vec<(String, String)> = Vec::new();
    // dbg!(ignore.is_ignored(&path.join("node_modules")));
    check_dir(path, args.formatting, &ignore, &mut invalid_entries).unwrap();

    if invalid_entries.is_empty() {
        println!("✨ All file names are formatted correctly");
    } else {
        println!("🐷 The following files are not formatted correctly: \n");
        for (path, expected) in invalid_entries {
            println!("{path} => {expected}");
        }

        std::process::exit(1);
    }
}

fn check_dir(
    path: &Path,
    formatting: Format,
    ignore: &Ignore,
    invalid: &mut Vec<(String, String)>,
) -> Result<()> {
    let paths = fs::read_dir(path)?;

    for path in paths {
        let path = path?;
        let path = path.path();

        let path_string = path.file_name().unwrap().to_string_lossy();
        if ignore.is_ignored(&path) {
            continue;
        }
        match formatting.check(&path_string) {
            CheckResult::Ok => (),
            CheckResult::Expected(expected) => {
                invalid.push((path.to_string_lossy().into_owned(), expected))
            }
        };

        if path.is_dir() {
            let gitignore_path = path.join(".gitignore");
            let new_ignore = if gitignore_path.exists() {
                Some(ignore.concat(&gitignore_path)?)
            } else {
                None
            };
            check_dir(
                &path,
                formatting,
                new_ignore.as_ref().unwrap_or(ignore),
                invalid,
            )?;
        }
    }

    Ok(())
}
