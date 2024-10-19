use time::format_description::well_known::{iso8601, Iso8601};

const FORMAT: Iso8601<6651332276412969266533270467398074368> = Iso8601::<{ iso8601::Config::DEFAULT.set_year_is_six_digits(false).encode() }>;
time::serde::format_description!(iso_8601_4dy_internal, OffsetDateTime, FORMAT);

pub mod iso_8601_4dy {
    use serde::{Deserializer, Serializer};
    use time::OffsetDateTime;

    use crate::http::serde::iso_8601_4dy_internal;

    pub fn serialize<S>(datetime: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
    {
        iso_8601_4dy_internal::serialize(datetime, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where D: Deserializer<'de>
    {
        iso_8601_4dy_internal::deserialize(deserializer)
    }
}