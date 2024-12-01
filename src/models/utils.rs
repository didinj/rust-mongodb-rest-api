use chrono::{ DateTime, Utc };
use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime as BsonDateTime;
use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::Deserialize;

// Deserialize MongoDB's DateTime or an RFC3339 string into Chrono's DateTime<Utc>
pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where D: serde::Deserializer<'de>
{
    struct DateTimeVisitor;

    impl<'de> serde::de::Visitor<'de> for DateTimeVisitor {
        type Value = DateTime<Utc>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a valid BSON datetime or an RFC3339 formatted string")
        }

        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where A: serde::de::MapAccess<'de>
        {
            let bson_datetime: BsonDateTime = serde::Deserialize::deserialize(
                serde::de::value::MapAccessDeserializer::new(map)
            )?;
            Ok(bson_to_chrono(bson_datetime))
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> where E: serde::de::Error {
            DateTime::parse_from_rfc3339(value)
                .map(|dt| dt.with_timezone(&Utc))
                .map_err(|_| E::invalid_value(serde::de::Unexpected::Str(value), &self))
        }
    }

    deserializer.deserialize_any(DateTimeVisitor)
}

// Serialize Chrono's DateTime<Utc> into a BSON-compatible format or string
pub fn serialize_datetime<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    serializer.serialize_str(&date.to_rfc3339())
}

// Converts a `mongodb::bson::DateTime` to `chrono::DateTime<Utc>`.
pub fn bson_to_chrono(bson_datetime: BsonDateTime) -> DateTime<Utc> {
    let timestamp_millis = bson_datetime.timestamp_millis();
    let seconds = timestamp_millis / 1000;
    let nanos = (timestamp_millis % 1000) * 1_000_000;

    // Unwrap the Option to get the DateTime<Utc>
    DateTime::from_timestamp(seconds, nanos as u32).expect("Invalid timestamp in BSON DateTime")
}

pub fn serialize_object_id<S>(id: &Option<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    match id {
        Some(ref oid) => serializer.serialize_str(&oid.to_string()),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_object_id<'de, D>(deserializer: D) -> Result<Option<ObjectId>, D::Error>
    where D: Deserializer<'de>
{
    Option::deserialize(deserializer).and_then(|opt| {
        match opt {
            Some(v) => Ok(Some(v)),
            None => Ok(None),
        }
    })
}
