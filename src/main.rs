#![allow(dead_code)]

use std::fmt::{Display, Formatter, Result};

use log::{Metadata, Record};
use serde_json::json;

#[derive(Debug, Default)]
struct Human {
  first_name: String,
  last_name: String,
  age: u8,
  friend: Vec<Human>,
}

impl Display for Human {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "My name is {first_name} - {last_name}, i'm {age:0>pad$} years old", first_name = self.first_name, last_name = self.last_name, age = self.age, pad = 3)?;

    if self.friend.is_empty() {
      write!(f, "")?
    }

    write!(f, "my friends name are: ")?;

    for (key, value) in self.friend.iter().enumerate() {
      if key != 0 {
        write!(f, ", ")?;
      }

      write!(f, "{count}-{name}", count = key, name = value.first_name)?;
    }

    write!(f, ".")
  }
}

impl From<u8> for Human {
  fn from(value: u8) -> Self {
    Human{ first_name: "".to_string(), last_name: "".to_string(), age: value, friend: vec![] }
  }
}

impl TryFrom<Vec<Human>> for Human {
  type Error = String;

  fn try_from(value: Vec<Human>) -> std::result::Result<Self, Self::Error> {
    if value.is_empty() {
      return Err(format!("something definitely went wrong"));
    }

    let age = value.len() as u8;

    Ok(
      Human{
        friend: value,
        first_name: "".to_string(),
        last_name: "".to_string(),
        age,
      }
    )
  }
}

impl Human {
  fn describe(&self) -> String {
    return format!(
      "my name is {}|{}, i'm {} years old and I also have {} friends",
      self.first_name,
      self.last_name,
      self.age,
      self.friend.len()
    );
  }

  fn new(f: String, l: String, a: u8, fr: Vec<Human>) -> Self {
    Self {
      first_name: f,
      last_name: l,
      age: a,
      friend: fr,
    }
  }
}

struct KeyValueLogger;

impl KeyValueLogger {
  fn new() -> Self {
    Self
  }

  fn format_log(&self, record: &Record) -> String {
    let log_entry = json!({
      "timestamp": chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
      "level": record.level().to_string(),
      "target": record.target(),
      "message": record.args(),
      "line": line!(),
      "file": file!(),
    });

    serde_json::to_string(&log_entry).unwrap()
  }
}

impl log::Log for KeyValueLogger {
  fn enabled(&self, _metadata: &Metadata) -> bool {
    true // Enable all log levels
  }

  fn log(&self, record: &Record) {
    if self.enabled(record.metadata()) {
      let log_message = self.format_log(record);
      println!("{}", log_message);
    }
  }

  fn flush(&self) {}
}

#[macro_export]
macro_rules! log_info {
  ($($key:ident = $value:expr),* $(,)?) => {{
    let logger = log::logger();

    let log_record = log::Record::builder()
      .args(format_args!("all things been equal"))
      .level(log::Level::Info)
      .target(module_path!())
      .build();

    logger.log(&log_record);
  }};
}

struct LogsMessage {
  request_id: String,
  message: String,
  object: String,
  function_name: String,
}

impl LogsMessage {
  fn new(message: String, request_id: String, object: String, function_name: String) -> Self {
    Self {
      message,
      request_id,
      object,
      function_name,
    }
  }

  fn log(&self, level: log::LevelFilter, error: Option<String>) -> String {
    if error.is_none() || level == log::LevelFilter::Error {
      log::error!("something gruesome happened");
    }
    return "".to_owned();
  }
}

fn main() {
  let logger = KeyValueLogger::new();
  log::set_boxed_logger(Box::new(logger)).expect("Failed to set logger");
  log::set_max_level(log::LevelFilter::Info);

  let human: Human = 4u8.into();
  let hum = Human::from(23u8);

  let turbo_fish = "23".parse::<i32>().expect("something went wrong");

  println!("{:#?}", human);
  println!("turbo fish {}", turbo_fish);
  println!("hummus {:#?}", hum);

  let _m = LogsMessage::new(
    "message".to_owned(),
    "request_id".to_owned(),
    "object".to_owned(),
    "main".to_owned(),
  );
}
