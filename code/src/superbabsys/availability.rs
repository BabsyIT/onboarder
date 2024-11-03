use chrono::{Datelike, NaiveDate, NaiveDateTime, TimeDelta, Weekday};

#[derive(Debug, Clone)]
pub struct Availability {
    dates: Vec<AvailabilityRange>,
}

impl Availability {
    pub fn new() -> Self {
        Self { dates: vec![] }
    }

    pub fn get_dates(&self) -> Vec<AvailabilityRange> {
        self.clone().dates
    }

    pub fn add_date(&mut self, from: NaiveDateTime, to: NaiveDateTime) {
        self.dates.push(AvailabilityRange::new(from, to));
    }

    pub fn get_available(&self, date: NaiveDate) -> bool {
        self.dates
            .iter()
            .any(|range| date >= range.from.date() && date <= range.to.date())
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

    pub fn get_from(&self) -> NaiveDateTime {
        self.from
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
            .filter(|date| date >= &self.from)
            .collect();
        without_sundays
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDateTime, TimeDelta};

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
    fn should_not_change_start_after_second_call() {
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
