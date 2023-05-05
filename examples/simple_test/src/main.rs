use carbono::Carbono;

fn main() {
    let carbono = Carbono::now(); // 2022-12-15 12:30:00

    println!("{}", carbono); // 2022-12-15 12:30:00 UTC

    assert_eq!(carbono.timestamp(), 1671107400);

    assert_eq!(carbono.rfc3339(), "2022-12-15T12:30:00+00:00");
    assert_eq!(carbono.rfc2822(), "Thu, 15 Dec 2022 12:30:00 +0000");

    assert_eq!(carbono.year(), 2022);
    assert_eq!(carbono.month(), 12);
    assert_eq!(carbono.day(), 15);
    assert_eq!(carbono.hour(), 12);
    assert_eq!(carbono.minute(), 30);
    assert_eq!(carbono.second(), 0);

    assert_eq!(carbono.datetime(), "2022-12-15 12:30:00");

    assert_eq!(carbono.date(), "2022-12-15");
    assert_eq!(carbono.time(), "12:30:00");

    assert_eq!(format!("{:?}", carbono.iso_week()), "2022-W50");

    assert_eq!(carbono.weekday(), 3); // Thursday

    assert_eq!(carbono.is_wednesday(), false);
    assert_eq!(carbono.is_thursday(), true);
    assert_eq!(carbono.is_friday(), false);

    assert_eq!(carbono.is_past(), true);
    assert_eq!(carbono.is_future(), false);

    assert_eq!(carbono.is_today(), true); // 2022-12-15

    assert_eq!(carbono.add_day().is_today(), false); // 2022-12-16

    assert_eq!(carbono.add_year().add_month().date(), "2024-01-15");

    assert_eq!(carbono.start_year().datetime(), "2022-01-01 00:00:00");
}
