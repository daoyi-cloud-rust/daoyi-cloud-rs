use std::env;

/// App environment
#[derive(Debug, Clone, Default)]
pub enum Env {
    /// Development
    #[default]
    Dev,
    /// Test
    Test,
    /// production
    Prod,
    /// Other
    Other(String),
}

impl Env {
    /// Initializes environment variables from the `.env` file and reads `APP_ENV` to determine the active environment for the application.
    pub fn init(env_path: Option<&str>) -> Self {
        let env_path = env_path.unwrap_or(".env");
        match dotenvy::from_path(env_path) {
            Ok(path) => println!(
                "Loaded the environment variable file under the path: \"{:?}\"",
                path
            ),
            Err(e) => eprintln!("Environment variable file not found: {}", e),
        }

        let env = Self::from_env();
        let _ = dotenvy::from_path_override(env.get_env_path(env_path));
        env
    }

    /// Read `APP_ENV` to determine the environment of the active application.
    /// If there is no `APP_ENV` variable, it defaults to Dev
    pub fn from_env() -> Self {
        match env::var("APP_ENV") {
            Ok(var) => Self::from_string(var),
            Err(_) => Self::Dev,
        }
    }

    /// Parse the string to get the corresponding environment
    pub fn from_string<S: Into<String>>(str: S) -> Self {
        match str.into() {
            s if s.eq_ignore_ascii_case("dev") => Self::Dev,
            s if s.eq_ignore_ascii_case("test") => Self::Test,
            s if s.eq_ignore_ascii_case("prod") => Self::Prod,
            s => Self::Other(s),
        }
    }

    pub(crate) fn get_env_path(&self, path: &str) -> String {
        match self {
            Self::Dev => format!("{path}.dev"),
            Self::Test => format!("{path}.test"),
            Self::Prod => format!("{path}.prod"),
            Self::Other(s) => format!("{path}.{s}"),
        }
    }
}

pub(crate) fn interpolate(template: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    let chars: Vec<char> = template.chars().collect();

    while i < chars.len() {
        if chars[i] == '$' && i + 1 < chars.len() && chars[i + 1] == '{' {
            // find "}"
            let mut j = i + 2; // Skip `${`
            while j < chars.len() && chars[j] != '}' {
                j += 1;
            }

            if j < chars.len() && chars[j] == '}' {
                // extract var_name & default_value
                let placeholder: String = chars[i + 2..j].iter().collect();

                // find default_value
                if let Some(pos) = placeholder.find(':') {
                    let var_name = &placeholder[..pos];
                    if let Ok(value) = env::var(var_name) {
                        result.push_str(&value);
                    } else {
                        result.push_str(&placeholder[pos + 1..]);
                    }
                } else if let Ok(value) = env::var(&placeholder) {
                    result.push_str(&value);
                } else {
                    result.push_str("${");
                    result.push_str(&placeholder);
                    result.push('}');
                }

                i = j + 1; // move to next
            } else {
                result.push('$');
                i += 1;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    result
}
