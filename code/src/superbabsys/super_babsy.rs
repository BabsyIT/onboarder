use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use super::{Availability, AvailabilityRange, LanguageCompetency};

#[derive(Debug, Clone)]
pub struct SuperBabsy {
    pub id: uuid::Uuid,
    pub name: String,
    pub image_url: Option<String>,
    pub description: String,
    pub sitter: Option<Vec<LanguageCompetency>>,
    pub parent: Option<Vec<LanguageCompetency>>,
    pub availability: Availability,
    pub video_chat_link: String,
}

impl SuperBabsy {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: uuid::Uuid,
        name: String,
        description: String,
        image_url: Option<String>,
        sitter: Option<Vec<LanguageCompetency>>,
        parent: Option<Vec<LanguageCompetency>>,
        availability: Availability,
        video_chat_link: String,
    ) -> Self {
        Self {
            id,
            name,
            description,
            image_url,
            sitter,
            parent,
            availability,
            video_chat_link,
        }
    }

    pub fn new_base(name: String, description: String) -> Self {
        Self::new(
            uuid::Uuid::new_v4(),
            name.clone(),
            description,
            None,
            None,
            None,
            Availability::new(),
            format!(
                "https://jitsi.babsy.ch/{}",
                name.split(' ').collect::<Vec<&str>>().first().unwrap()
            ),
        )
    }

    pub fn is_available(&self, date: NaiveDate) -> bool {
        let date = NaiveDateTime::new(date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        self.availability.get_available(date)
    }

    pub fn get_sitter(&self) -> Option<Vec<LanguageCompetency>> {
        self.sitter.clone()
    }

    pub fn get_parent(&self) -> Option<Vec<LanguageCompetency>> {
        self.parent.clone()
    }

    pub fn parent_comp_as_string(&self) -> String {
        match &self.parent {
            Some(p) => p
                .iter()
                .map(|lang| lang.name())
                .collect::<Vec<&str>>()
                .join(", "),
            None => "None".to_string(),
        }
    }

    pub fn sitter_comp_as_string(&self) -> String {
        match &self.sitter {
            Some(p) => p
                .iter()
                .map(|lang| lang.name())
                .collect::<Vec<&str>>()
                .join(", "),
            None => "None".to_string(),
        }
    }

    pub fn get_image_url_string_or_none(&self) -> String {
        self.image_url.clone().unwrap_or("None".to_string())
    }

    pub fn get_id(&self) -> uuid::Uuid {
        self.id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_availability(&self) -> Availability {
        self.availability.clone()
    }

    //returns all available hours from a given date toward the future
    pub fn get_available_hours(&self, from: NaiveDateTime) -> Vec<NaiveDateTime> {
        let mut available_hours: Vec<NaiveDateTime> = self
            .get_availability()
            .get_dates()
            .iter()
            .flat_map(|range: &AvailabilityRange| {
                if range.is_available(from) {
                    range.every_possible_hour(from)
                } else {
                    Vec::new()
                }
            })
            .collect();

        available_hours.dedup();

        available_hours.sort();

        available_hours
    }

    pub fn get_available_dates_from_first(&self) -> Vec<NaiveDateTime> {
        let a = self.get_availability();
        let dates = a.get_dates();
        let Some(first) = dates.first() else {
            return Vec::new();
        };
        let mut available_hours: Vec<NaiveDateTime> = self
            .get_availability()
            .get_dates()
            .iter()
            .flat_map(|range: &AvailabilityRange| {
                if range.is_available(first.get_from()) {
                    range.every_possible_hour(first.get_from())
                } else {
                    Vec::new()
                }
            })
            .collect();

        available_hours.dedup();

        available_hours.sort();

        available_hours
    }
}
