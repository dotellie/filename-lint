use clap::ValueEnum;
use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase,
    ToTitleCase, ToTrainCase, ToUpperCamelCase,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[allow(clippy::enum_variant_names)]
pub enum Format {
    KebabCase,
    LowerCamelCase,
    PascalCase,
    ShoutyKebabCase,
    ShoutySnakeCase,
    SnakeCase,
    TitleCase,
    TrainCase,
    UpperCamelCase,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckResult {
    Ok,
    Expected(String),
}

impl Format {
    pub fn convert(&self, name: &str) -> String {
        match self {
            Format::KebabCase => name.to_kebab_case(),
            Format::LowerCamelCase => name.to_lower_camel_case(),
            Format::PascalCase => name.to_pascal_case(),
            Format::ShoutyKebabCase => name.to_shouty_kebab_case(),
            Format::ShoutySnakeCase => name.to_shouty_snake_case(),
            Format::SnakeCase => name.to_snake_case(),
            Format::TitleCase => name.to_title_case(),
            Format::TrainCase => name.to_train_case(),
            Format::UpperCamelCase => name.to_upper_camel_case(),
        }
    }

    pub fn check(&self, name: &str) -> CheckResult {
        let mut parts = name.split('.');

        let main_part = parts.next().unwrap();

        let expected_main_part = self.convert(main_part);
        if expected_main_part != main_part {
            let mut expected = expected_main_part;

            for part in parts {
                expected += &format!(".{part}");
            }
            CheckResult::Expected(expected)
        } else if parts.all(|part| part.to_lowercase() == part) {
            CheckResult::Ok
        } else {
            let tail = parts
                .map(|part| part.to_lowercase())
                .collect::<Vec<_>>()
                .join(".");
            CheckResult::Expected(expected_main_part + &tail)
        }
    }
}
