pub mod utilities {
    use chrono::{DateTime, Utc};

    pub fn parse_date(src: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
        let naive = chrono::NaiveDateTime::parse_from_str(src, "%Y-%m-%d %H:%M:%S")?;
        let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, Utc);
        Ok(datetime)
    }

    pub fn pause() {
        println!("Press any key to continue...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }
}
