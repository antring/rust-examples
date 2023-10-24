use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use serde;
use serde_json;
use chrono;
use chrono::{DateTime, Utc};
use parquet::file::properties::WriterProperties;
use parquet::file::writer::SerializedFileWriter;
use parquet::record::Row;
use parquet::schema::parser::parse_message_type;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    foo: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct Record {
    id: u8,
    #[serde(rename = "timeCreated")]
    time_created: DateTime<Utc>,
    data: Vec<Data>,
}


fn main() {
    let json_file = File::open("data.json").unwrap();
    let reader = BufReader::new(json_file);
    let record: Record = serde_json::from_reader(reader).unwrap();

    let parquet_file = File::create("output.parquet").unwrap();
    let parquet_schema = Arc::new(parse_message_type("
        message schema {
            required int32 id;
            required binary time_created (UTF8);
            repeated group data {
                optional int32 foo;
            }
        }
    ").unwrap());

    let writer_properties = Arc::new(WriterProperties::builder().build());
    let mut writer = SerializedFileWriter::new(parquet_file, parquet_schema, writer_properties).unwrap();

    let mut row = Row::new(writer.schema().get_fields().len());
}
