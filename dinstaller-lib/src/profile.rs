use crate::manager::ManagerClient;
use curl::easy::Easy;
use jsonschema::JSONSchema;
use serde_json;
use std::{
    error::Error,
    fs, io,
    io::{stdout, Write},
    path::Path,
    process::Command,
};
use tempfile::tempdir;

/// Downloads a file a writes it to the stdout()
///
/// TODO: move this code to a struct
/// TODO: add support for YaST-specific URLs
/// TODO: do not write to stdout, but to something implementing the Write trait
pub fn download(url: &str) -> Result<(), Box<dyn Error>> {
    let mut easy = Easy::new();
    easy.url(url)?;
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })?;
    easy.perform()?;
    Ok(())
}

#[derive(Debug)]
pub enum ValidationResult {
    Valid,
    NotValid(Vec<String>),
}

/// Checks whether an autoinstallation profile is valid
///
/// ```
/// # use dinstaller_lib::profile::{ProfileValidator, ValidationResult};
/// # use std::path::Path;
/// let validator = ProfileValidator::default_schema()
///   .expect("the default validtor");
///
/// // you can validate a &str
/// let wrong_profile = r#"
///   { "product": { "name": "Tumbleweed" } }
/// "#;
/// let result = validator.validate_str(&wrong_profile).unwrap();
/// assert!(matches!(ValidationResult::NotValid, result));
///
/// // or a file
/// validator.validate_file(Path::new("share/examples/profile.json"));
/// assert!(matches!(ValidationResult::Valid, result));
/// ```
pub struct ProfileValidator {
    schema: JSONSchema,
}

impl ProfileValidator {
    pub fn default_schema() -> Result<Self, Box<dyn Error>> {
        let relative_path = Path::new("dinstaller-lib/share/profile.schema.json");
        let path = if relative_path.exists() {
            relative_path
        } else {
            Path::new("/usr/share/dinstaller-rs/profile.schema.json")
        };
        Self::new(path)
    }

    pub fn new(schema_path: &Path) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(schema_path)?;
        let schema = serde_json::from_str(&contents)?;
        let schema = JSONSchema::compile(&schema).expect("A valid schema");
        Ok(Self { schema })
    }

    pub fn validate_file(&self, profile_path: &Path) -> Result<ValidationResult, Box<dyn Error>> {
        let contents = fs::read_to_string(profile_path)?;
        self.validate_str(&contents)
    }

    pub fn validate_str(&self, profile: &str) -> Result<ValidationResult, Box<dyn Error>> {
        let contents = serde_json::from_str(&profile)?;
        let result = self.schema.validate(&contents);
        if let Err(errors) = result {
            let messages: Vec<String> = errors.map(|e| format!("{e}")).collect();
            return Ok(ValidationResult::NotValid(messages));
        }
        Ok(ValidationResult::Valid)
    }
}

/// Evaluates a profile
///
/// Evaluating a profile means injecting the hardware information (coming from D-Bus)
/// and running the jsonnet code to generate a plain JSON file. For this struct to
/// work, the `/usr/bin/jsonnet` command must be available.
pub struct ProfileEvaluator<'a> {
    manager_client: ManagerClient<'a>,
}

impl<'a> ProfileEvaluator<'a> {
    pub async fn new() -> Result<ProfileEvaluator<'a>, zbus::Error> {
        let manager_client = ManagerClient::new(super::connection().await?).await?;
        Ok(Self { manager_client })
    }

    pub async fn evaluate(&self, profile_path: &Path) -> Result<(), Box<dyn Error>> {
        let dir = tempdir()?;

        let working_path = dir.path().join("profile.jsonnet");
        fs::copy(profile_path, working_path)?;

        let hwinfo_path = dir.path().join("dinstaller.libsonnet");
        let hwinfo = self.manager_client.hwinfo().await?;
        fs::write(hwinfo_path, serde_json::to_string(&hwinfo)?)?;

        let result = Command::new("/usr/bin/jsonnet")
            .arg("profile.jsonnet")
            .current_dir(&dir)
            .output()?;
        io::stdout().write_all(&result.stdout)?;
        Ok(())
    }
}
