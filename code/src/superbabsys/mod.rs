use chrono::{NaiveDateTime, NaiveTime};

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
struct Availability {
    dates: Vec<AvailabilityRange>,
}

#[derive(Debug, Clone)]
struct AvailabilityRange {
    from: NaiveDateTime,
    to: NaiveDateTime,
}

#[derive(Debug, Clone)]
struct Both {
    sitter: Vec<LanguageCompetency>,
    parent: Vec<LanguageCompetency>,
}

#[derive(Debug, Clone)]
struct LanguageCompetency {
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
}
