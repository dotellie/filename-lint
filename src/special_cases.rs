pub enum SpecialCase {
    Ignore,
    LintSubset(String),
}

pub fn is_special_case(filename: &str) -> Option<SpecialCase> {
    [scss_partial, next_path, dotfiles]
        .iter()
        .find_map(|f| f(filename))
}

fn dotfiles(file: &str) -> Option<SpecialCase> {
    if file.starts_with('.') {
        Some(SpecialCase::Ignore)
    } else {
        None
    }
}

fn scss_partial(file: &str) -> Option<SpecialCase> {
    if file.starts_with('_') && file.ends_with(".scss") {
        let subset = file[1..file.len() - 5].to_string();
        Some(SpecialCase::LintSubset(subset))
    } else {
        None
    }
}

fn next_path(file: &str) -> Option<SpecialCase> {
    let main_part = file.split('.').next().unwrap();
    if main_part.starts_with('[') && main_part.ends_with(']') {
        Some(SpecialCase::LintSubset(
            main_part[1..main_part.len() - 1].to_string(),
        ))
    } else {
        None
    }
}
