use diesel::{Queryable, Insertable, AsChangeset};
use chrono::{NaiveDateTime, DateTime, Utc};
use serde::{Serialize, Deserialize};

use super::schema::users;

mod date_format {
    use chrono::{NaiveDateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &NaiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    
    #[serde(with = "date_format")]
    pub created_at: NaiveDateTime,
    #[serde(with = "date_format")]
    pub updated_at: NaiveDateTime
}