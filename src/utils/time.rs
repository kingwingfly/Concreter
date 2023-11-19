use snafu::{ResultExt, Snafu};
use time::{Duration, OffsetDateTime};

pub use time::format_description::well_known::Rfc3339;

pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub fn format_time(time: OffsetDateTime) -> String {
    time.format(&Rfc3339).unwrap() // TODO: need to check if safe.
}

pub fn now_utc_plus_sec_str(sec: f64) -> String {
    let new_time = now_utc() + Duration::seconds_f64(sec);
    format_time(new_time)
}

pub fn parse_utc(moment: &str) -> TimeResult<OffsetDateTime> {
    OffsetDateTime::parse(moment, &Rfc3339).context(time_error::FailToDateParse {
        moment: moment.to_owned(),
    })
}

// region:    --- Error

#[derive(Debug, Snafu)]
#[snafu(module, visibility(pub), context(suffix(false)))]
pub enum TimeError {
    #[snafu(display("Fail to parse date: {}", moment))]
    FailToDateParse {
        source: time::error::Parse,
        moment: String,
    },
}

pub type TimeResult<T> = Result<T, TimeError>;
// endregion: --- Error Boilerplate

// endregion: --- Error
