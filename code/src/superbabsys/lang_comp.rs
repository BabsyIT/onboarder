#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageCompetency {
    id: i32,
    code: String,
    name: String,
}

impl TryFrom<String> for LanguageCompetency {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "en" => Ok(Self::english()),
            "de" => Ok(Self::german()),
            "pt" => Ok(Self::portuguese()),
            _ => Err("Invalid language competency".to_string()),
        }
    }
}

impl LanguageCompetency {
    pub fn english() -> Self {
        Self {
            id: 1,
            code: "en".to_string(),
            name: "English".to_string(),
        }
    }
    pub fn german() -> Self {
        Self {
            id: 2,
            code: "de".to_string(),
            name: "German".to_string(),
        }
    }

    pub fn portuguese() -> Self {
        Self {
            id: 3,
            code: "pt".to_string(),
            name: "Portuguese".to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn code(&self) -> &str {
        &self.code
    }
}
