pub struct WorldPregenConfig {
    pub name: String,
}

impl Default for WorldPregenConfig {
    fn default() -> Self {
        Self {
            name: "".to_string(),
        }
    }
}