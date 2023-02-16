use std::path::Path;

pub fn special_ignore(path: &Path) -> bool {
    let file = path.file_name().unwrap().to_string_lossy();

    scss_partial(&file) || next_path(&file)
}

fn scss_partial(file: &str) -> bool {
    file.starts_with('_') && file.ends_with(".scss")
}

fn next_path(file: &str) -> bool {
    let main_part = file.split('.').next().unwrap();
    main_part.starts_with('[') && main_part.ends_with(']')
}
