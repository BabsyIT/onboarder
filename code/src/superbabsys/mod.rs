use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

#[derive(Debug, Clone)]
pub struct SuperBabsy {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub sitter: Option<Vec<LanguageCompetency>>,
    pub parent: Option<Vec<LanguageCompetency>>,
    pub availability: Availability,
}

#[derive(Debug, Clone)]
pub struct Availability {
    dates: Vec<AvailabilityRange>,
}

impl Availability {
    pub fn new() -> Self {
        Self { dates: vec![] }
    }

    pub fn add_date(&mut self, from: NaiveDate, to: NaiveDate) {
        self.dates.push(AvailabilityRange::new(from, to));
    }

    pub fn get_available(&self, date: NaiveDate) -> bool {
        self.dates
            .iter()
            .any(|range| date >= range.from && date <= range.to)
    }
}

#[derive(Debug, Clone)]
pub struct AvailabilityRange {
    from: NaiveDate,
    to: NaiveDate,
}

impl AvailabilityRange {
    pub fn new(from: NaiveDate, to: NaiveDate) -> Self {
        Self { from, to }
    }
}

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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn code(&self) -> &str {
        &self.code
    }
}

impl SuperBabsy {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        description: String,
        sitter: Option<Vec<LanguageCompetency>>,
        parent: Option<Vec<LanguageCompetency>>,
        availability: Availability,
    ) -> Self {
        Self {
            id,
            name,
            description,
            sitter,
            parent,
            availability,
        }
    }

    pub fn new_base(name: String, description: String) -> Self {
        Self::new(
            uuid::Uuid::new_v4(),
            name,
            description,
            None,
            None,
            Availability { dates: Vec::new() },
        )
    }

    pub fn add_language_to_sitter_comp(&mut self, lang: LanguageCompetency) {
        match &mut self.sitter {
            Some(s) => s.push(lang),
            None => self.sitter = Some(vec![lang]),
        }
    }

    pub fn add_language_to_parent_comp(&mut self, lang: LanguageCompetency) {
        match &mut self.parent {
            Some(s) => s.push(lang),
            None => self.parent = Some(vec![lang]),
        }
    }

    pub fn is_available(&self, date: NaiveDate) -> bool {
        self.availability.get_available(date)
    }

    pub fn get_sitter(&self) -> Option<Vec<LanguageCompetency>> {
        self.sitter.clone()
    }

    pub fn get_parent(&self) -> Option<Vec<LanguageCompetency>> {
        self.parent.clone()
    }
}
