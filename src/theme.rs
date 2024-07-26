use ::serde::Serialize;
use time::{serde, Date};

serde::format_description!(date, Date, "[year]-[month]-[day]");

#[derive(Debug, Serialize)]
pub(crate) struct Theme {
    #[serde(with = "date::option")]
    pub(crate) activated: Option<Date>,
    pub(crate) culled: bool,
    pub(crate) title: String,
}
