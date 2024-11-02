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

#[derive(Debug, Clone)]
pub struct Both {
    sitter: Vec<LanguageCompetency>,
    parent: Vec<LanguageCompetency>,
}

#[derive(Debug, Clone)]
pub struct LanguageCompetency {
    id: i32,
    name: String,
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
}
