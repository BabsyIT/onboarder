use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, Weekday};

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

#[derive(Debug, Clone)]
pub struct Availability {
    dates: Vec<AvailabilityRange>,
}

impl Availability {
    pub fn new() -> Self {
        Self { dates: vec![] }
    }

    pub fn add_date(&mut self, from: NaiveDateTime, to: NaiveDateTime) {
        self.dates.push(AvailabilityRange::new(from, to));
    }

    pub fn get_available(&self, date: NaiveDateTime) -> bool {
        self.dates
            .iter()
            .any(|range| date >= range.from && date <= range.to)
    }
}

#[derive(Debug, Clone)]
pub struct AvailabilityRange {
    from: NaiveDateTime,
    to: NaiveDateTime,
}

impl AvailabilityRange {
    pub fn new(from: NaiveDateTime, to: NaiveDateTime) -> Self {
        Self { from, to }
    }

    pub fn is_available(&self, date: NaiveDateTime) -> bool {
        date >= self.from && date <= self.to
    }

    pub fn every_possible_hour(&self, from: NaiveDateTime) -> Vec<NaiveDateTime> {
        // the from is decieded by the sitter/parent
        // We only want forward looking hours
        let mut start = from;
        let end = self.to;
        let mut hours = vec![];

        while start < end {
            hours.push(start);
            start += TimeDelta::hours(1);
        }

        let without_sundays: Vec<NaiveDateTime> = hours
            .into_iter()
            .filter(|date| date.weekday() != Weekday::Sun)
            .collect();
        without_sundays
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeDelta;

    #[test]
    fn gets_all_hours_in_two_hours() {
        let from =
            NaiveDateTime::parse_from_str("2021-01-01 08:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let to = NaiveDateTime::parse_from_str("2021-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let range = AvailabilityRange::new(from, to);
        let hours = range.every_possible_hour(from);
        assert_eq!(hours.len(), 2);
        assert_eq!(hours[0], from);
        assert_eq!(
            hours[1],
            to.checked_sub_signed(TimeDelta::hours(1)).unwrap()
        );
    }

    #[test]
    fn gets_all_hours_in_two_days() {
        let from =
            NaiveDateTime::parse_from_str("2021-01-01 08:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let to = NaiveDateTime::parse_from_str("2021-01-02 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let range = AvailabilityRange::new(from, to);
        let hours = range.every_possible_hour(from);
        assert_eq!(hours.len(), 26);
        assert_eq!(hours[0], from);
        assert_eq!(
            hours.last().unwrap(),
            &to.checked_sub_signed(TimeDelta::hours(1)).unwrap()
        );
    }

    #[test]
    fn should_change_start_after_second_call() {
        let from =
            NaiveDateTime::parse_from_str("2021-01-01 08:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let to = NaiveDateTime::parse_from_str("2021-01-02 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let range = AvailabilityRange::new(from, to);
        let _ = range.every_possible_hour(from);
        let hours = range.every_possible_hour(from);

        assert_eq!(hours.len(), 26);
        assert_eq!(hours[0], from);
        assert_eq!(
            hours.last().unwrap(),
            &to.checked_sub_signed(TimeDelta::hours(1)).unwrap()
        );
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
            Availability { dates: Vec::new() },
            format!(
                "https://jitsi.babsy.ch/{}",
                name.split(' ').collect::<Vec<&str>>().first().unwrap()
            ),
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
        let date = NaiveDateTime::new(date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        self.availability.get_available(date)
    }

    pub fn get_sitter(&self) -> Option<Vec<LanguageCompetency>> {
        self.sitter.clone()
    }

    pub fn get_parent(&self) -> Option<Vec<LanguageCompetency>> {
        self.parent.clone()
    }

    //returns all available hours from a given date toward the future
    pub fn get_available_hours(&self, from: NaiveDateTime) -> Vec<NaiveDateTime> {
        let mut available_hours: Vec<NaiveDateTime> = self
            .availability
            .dates
            .iter()
            .map(|range: &AvailabilityRange| {
                if range.is_available(from) {
                    range.every_possible_hour(from)
                } else {
                    Vec::new()
                }
            })
            .flatten()
            .collect();

        available_hours.dedup();

        available_hours.sort();

        available_hours
    }
}
