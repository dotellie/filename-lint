pub enum SpecialCase {
    Ignore,
    LintSubset(String),
}

pub fn is_special_case(filename: &str) -> Option<SpecialCase> {
    [scss_partial, next_path, dotfiles]
        .iter()
        .find_map(|f| f(filename))
}

/// Check for a file name like `.gitignore` to ignore it.
/// It's assumed that all files starting with a dot are configuration files
/// and should therefore be ignored.
fn dotfiles(file: &str) -> Option<SpecialCase> {
    if file.starts_with('.') {
        Some(SpecialCase::Ignore)
    } else {
        None
    }
}

/// Check for a file name like `_foo.scss` to allow for partials in SCSS.
fn scss_partial(file: &str) -> Option<SpecialCase> {
    if file.starts_with('_') && file.ends_with(".scss") {
        let subset = file[1..file.len() - 5].to_string();
        Some(SpecialCase::LintSubset(subset))
    } else {
        None
    }
}

/// Checks for a file name like `[foo].tsx` to allow for dynamic routes in Next.js.
fn next_path(file: &str) -> Option<SpecialCase> {
    let mut parts = file.split('.');
    let main_part = parts.next().unwrap();
    if main_part.starts_with('[') && main_part.ends_with(']') {
        let to_lint = main_part[1..main_part.len() - 1].to_string();

        if let Some(extension) = parts.next() {
            if parts.next().is_some() {
                // More than one extension is not allowed
                None
            } else {
                // Check it's a JS file
                ["ts", "tsx", "js", "jsx"]
                    .into_iter()
                    .find(|ext| *ext == extension)
                    .map(|_| SpecialCase::LintSubset(to_lint))
            }
        } else {
            // No extension, we assume it's a directory
            Some(SpecialCase::LintSubset(to_lint))
        }
    } else {
        None
    }
}
