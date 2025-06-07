pub struct Versions {
    versions: std::collections::HashMap<String, String>,
}

impl Versions {
    pub async fn new() -> Result<Self, anyhow::Error> {
        match fs::read_to_string("versions.json").await {
            Ok(data) => {
                let versions: HashMap<String, String> = serde_json::from_str(&data)?;
                Ok(Self { versions })
            }
            Err(e) if e.kind() == ErrorKind::NotFound => {
                let versions = memoized_latest_versions().await?;
                let json = serde_json::to_string_pretty(&versions)?;
                fs::write("versions.json", json).await?;
                Ok(Self { versions })
            }
            Err(e) => Err(e.into()),
        }
    }

    pub fn get(&self, name: &str) -> Option<String> {
        self.versions.get(name).cloned()
    }
}
