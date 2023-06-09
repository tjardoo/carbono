use chrono::{Datelike, Timelike};

use crate::Carbono;

impl Carbono {
    pub fn end_year(&self) -> Self {
        Carbono {
            datetime: self.datetime
                .with_month(12)
                .unwrap()
                .with_day(31)
                .unwrap()
                .with_hour(23)
                .unwrap()
                .with_minute(59)
                .unwrap()
                .with_second(59)
                .unwrap()
        }
    }

    pub fn end_month(&self) -> Self {
        let days_in_month = match self.month() {
            2 => match self.is_leap_year() {
                true => 29,
                false => 28,
            }
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        };

        Carbono {
            datetime: self.datetime
                .with_day(days_in_month)
                .unwrap()
                .with_hour(23)
                .unwrap()
                .with_minute(59)
                .unwrap()
                .with_second(59)
                .unwrap()
        }
    }

    pub fn end_day(&self) -> Self {
        Carbono {
            datetime: self.datetime
                .with_hour(23)
                .unwrap()
                .with_minute(59)
                .unwrap()
                .with_second(59)
                .unwrap()
        }
    }

    pub fn end_hour(&self) -> Self {
        Carbono {
            datetime: self.datetime
                .with_minute(59)
                .unwrap()
                .with_second(59)
                .unwrap()
        }
    }

    pub fn end_minute(&self) -> Self {
        Carbono {
            datetime: self.datetime
                .with_second(59)
                .unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use super::*;

    #[test]
    fn it_can_move_to_the_end_of_the_year() {
        let carbono = Carbono::now().end_year();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2022, 12, 31, 23, 59, 59).unwrap());
    }

    #[test]
    fn it_can_move_to_the_end_of_the_month() {
        let carbono = Carbono::now().end_month();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2022, 12, 31, 23, 59, 59).unwrap());
    }

    #[test]
    fn it_can_move_to_the_end_of_the_month_february_without_leap_year() {
        let carbono = Carbono::now().sub_year().sub_months(10).end_month();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2021, 2, 28, 23, 59, 59).unwrap());
    }

    #[test]
    fn it_can_move_to_the_end_of_the_month_february_with_leap_year() {
        let carbono = Carbono::now().sub_years(2).sub_months(10).end_month();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2020, 2, 29, 23, 59, 59).unwrap());
    }

    #[test]
    fn it_can_move_to_the_end_of_the_day() {
        let carbono = Carbono::now().end_day();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2022, 12, 15, 23, 59, 59).unwrap());
    }

    #[test]
    fn it_can_move_to_the_end_of_the_hour() {
        let carbono = Carbono::now().end_hour();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2022, 12, 15, 12, 59, 59).unwrap());
    }

    #[test]
    fn it_can_move_to_the_end_of_the_minute() {
        let carbono = Carbono::now().end_minute();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2022, 12, 15, 12, 30, 59).unwrap());
    }
}
