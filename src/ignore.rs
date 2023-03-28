use anyhow::Result;
use std::path::Path;

const DEFAULT_IGNORED_FILES: &[&str] = &[
    // Node.js
    "yarn.lock",
    "package.json",
    "package-lock.json",
    // JS
    "vite.config.js",
    "rollup.config.js",
    "webpack.config.js",
    "jest.config.js",
    "tsconfig.json",
    // Next.js
    "next.config.js",
    "_app.tsx",
    "_app.js",
    "_app.jsx",
    "_document.tsx",
    "_document.js",
    "_document.jsx",
    // PHP
    "composer.lock",
    "composer.phar",
    "composer.json",
    // Rust
    "Cargo.lock",
    "Cargo.toml",
    // Ruby
    "Gemfile.lock",
    "Gemfile",
    // Docker
    "Dockerfile",
    "docker-compose.yml",
    // Git/GitHub files
    "README.md",
    "CONTRIBUTING.md",
    "CODE_OF_CONDUCT.md",
    "FUNDING.yml",
    "ISSUE_TEMPLATE",
    "PULL_REQUEST_TEMPLATE.md",
    "LICENSE",
    "SECURITY.md",
    "CODEOWNERS",
    // Husky empty folder
    "_",
    // Python
    "__pycache__",
];

#[derive(Clone)]
struct IgnorePattern {
    pattern: glob::Pattern,
    base_path: Option<String>,
}

impl std::fmt::Debug for IgnorePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IgnorePattern")
            .field("pattern", &self.pattern.as_str())
            .field("base_path", &self.base_path)
            .finish()
    }
}

#[derive(Debug)]
pub struct Ignore {
    ignore_patterns: Vec<IgnorePattern>,
    default_ignored_files: Vec<String>,
    base_path: String,
}

impl Ignore {
    pub fn new(gitignore_path: &Path) -> Result<Self> {
        let ignore_patterns = Self::parse_gitignore(gitignore_path, None)?;

        let mut default_ignored_files: Vec<String> = DEFAULT_IGNORED_FILES
            .iter()
            .map(|s| s.to_owned().to_owned())
            .collect();

        let base_path = gitignore_path
            .parent()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default()
            + "/";

        default_ignored_files.sort_unstable();
        Ok(Self {
            ignore_patterns,
            default_ignored_files,
            base_path,
        })
    }

    pub fn concat(&self, gitignore_path: &Path) -> Result<Self> {
        let relative_base_path = gitignore_path
            .strip_prefix(&self.base_path)?
            .parent()
            .map(|p| p.to_string_lossy().into_owned());
        let mut new_ignore_patterns =
            Self::parse_gitignore(gitignore_path, relative_base_path).unwrap_or_default();
        let mut ignore_patterns = self.ignore_patterns.clone();
        ignore_patterns.append(&mut new_ignore_patterns);

        Ok(Ignore {
            ignore_patterns,
            default_ignored_files: self.default_ignored_files.clone(),
            base_path: self.base_path.clone(),
        })
    }

    pub fn is_ignored(&self, path: &Path) -> bool {
        if path
            .file_name()
            .map(|file_name| file_name.to_string_lossy())
            .map(|file_name| {
                self.default_ignored_files
                    .binary_search(&file_name.into_owned())
                    .is_ok()
            })
            .unwrap_or(false)
        {
            return true;
        }

        // Convert from ./path to /path
        let pattern_path_string = String::from("/")
            + path
                .to_str()
                .map(|s| s.trim_start_matches(&self.base_path))
                .unwrap_or_default();
        let pattern_path = Path::new(&pattern_path_string);
        for pattern in &self.ignore_patterns {
            let resolved_path = if let Some(base_path) = &pattern.base_path {
                Path::new(pattern_path.strip_prefix(base_path).unwrap_or(pattern_path))
            } else {
                pattern_path
            };
            if pattern.pattern.matches_path(resolved_path) {
                return true;
            }
        }

        false
    }

    fn parse_gitignore(
        gitignore_path: &Path,
        base_path: Option<String>,
    ) -> Result<Vec<IgnorePattern>> {
        let ignore_patterns = std::fs::read_to_string(gitignore_path)
            .unwrap_or_default()
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.starts_with('#'))
            .filter(|line| !line.is_empty())
            .map(|line| line.trim_end_matches('/'))
            .map(|line| {
                if !line.contains('/') {
                    format!("**/{line}")
                } else {
                    line.to_owned()
                }
            })
            .filter_map(|line| glob::Pattern::new(&line).ok())
            .map(|pattern| IgnorePattern {
                pattern,
                base_path: base_path.as_ref().map(|s| String::from("/") + s),
            })
            .collect();

        Ok(ignore_patterns)
    }
}
