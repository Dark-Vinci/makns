#![allow(dead_code)]

use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::num::ParseIntError;
use std::ptr::{ null_mut };
// use std::fs::
use std::result;
// use std::error::Error;
// use std::any::Any;
// use std::path::{Display, is_separator, Path, PathBuf, Prefix, Component, Components, PrefixComponent};
// use std::thread::{available_parallelism, panicking, park, park_timeout, spawn, Thread, ThreadId, JoinHandle, Result, Builder, current, AccessError, scope, Scope, LocalKey, sleep, yield_now };
// use std::sync::{Mutex, MutexGuard, RwLockReadGuard, RwLockWriteGuard, RwLock};
// use std::sync::mpsc::{Receiver, Sender, SyncSender, SendError, TrySendError, RecvError, TryRecvError, TryIter, channel, sync_channel};

use log::{Metadata, Record};
use serde_json::json;

#[derive(Debug)]
struct Node<T> {
  data: T,
  next: *mut Node<T>,
  prev: *mut Node<T>,
}

impl<T> Node<T> {
  fn new(v: T) -> Self {
    Self {
      data: v,
      next: null_mut(),
      prev: null_mut(),
    }
  }
}

#[derive(Debug)]
struct LinkedList<T> {
  head: *mut Node<T>,
  tail: *mut Node<T>,
}

impl<T> LinkedList<T> {
  fn new() -> Self {
    Self {
      head: null_mut(),
      tail: null_mut(),
    }
  }

  fn is_empty(&self) -> bool {
    self.head.is_null()
  }

  fn front(&mut self, v: T) {
    let mut node = Box::into_raw(Box::new(Node::new(v)));

    if self.head.is_null() {
      self.head = node;
      self.tail = node;
      return;
    }

    unsafe {
      let h = self.head;

      (*node).next = h;
      (*h).prev = node;

      self.head = node;
      return;
    }
  }

  fn back(&mut self, v: T) {
    let mut node = Box::into_raw(Box::new(Node::new(v)));

    if self.tail.is_null() {
      self.head = node;
      self.tail = node;
      return;
    }

    unsafe {
      let t = self.tail;

      (*node).prev = t;
      (*t).next = node;

      self.tail = node;
      return;
    }
  }

  fn pop(&mut self) -> Option<T> {
    if self.tail.is_null() {
      return None;
    }

    unsafe {
      let l = self.tail;

      let prev = (*l).prev;

      if !prev.is_null() {
        (*prev).next = null_mut();
        self.tail = null_mut();
      } else {
        self.tail = prev;
      }

      return Some(Box::from_raw(l).data);
    }
  }

  fn up(&mut self) -> Option<T> {
    if self.head.is_null() {
      return None;
    }

    unsafe {
      let h = self.head;

      let next = (*h).next;

      if !next.is_null() {
        (*next).prev = null_mut();
        self.head = null_mut();
      } else {
        self.head = next;
      }

      return Some(Box::from_raw(h).data);
    }
  }

  fn len(&self) -> usize {
    1.into()
  }
}

fn three_remainder(number: u8) -> Cow<'static, str> {
  match number % 3 {
    0 => "zero".into(),
    1 => "wan".into(),
    remainder => format!("the remainder is {remainder}", remainder = remainder).into()
  }
}

#[derive(Debug, Default)]
struct Human {
  first_name: String,
  last_name: String,
  age: u8,
  friend: Vec<Human>,
}

impl PartialEq<Self> for Human {
  fn eq(&self, other: &Self) -> bool {
    self.friend.len() == other.friend.len()
  }
}

impl Display for Human {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(
      f,
      "My name is {first_name} - {last_name}, i'm {age:0>pad$} years old",
      first_name = self.first_name,
      last_name = self.last_name,
      age = self.age,
      pad = 3
    )?;

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
    Human {
      first_name: "".to_string(),
      last_name: "".to_string(),
      age: value,
      friend: vec![],
    }
  }
}

impl TryFrom<Vec<Human>> for Human {
  type Error = String;

  fn try_from(value: Vec<Human>) -> result::Result<Self, Self::Error> {
    if value.is_empty() {
      return Err(format!("something definitely went wrong"));
    }

    let age = value.len() as u8;

    Ok(Human {
      friend: value,
      first_name: "".to_string(),
      last_name: "".to_string(),
      age,
    })
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

fn main() -> Result<&'static str, ParseIntError> {
  let logger = KeyValueLogger::new();
  log::set_boxed_logger(Box::new(logger)).expect("Failed to set logger");
  log::set_max_level(log::LevelFilter::Info);

  let human: Human = 4u8.into();
  let hum = Human::from(23u8);

  let turbo_fish = "23".parse::<i32>().expect("something went wrong");

  println!("{:#?}", human);
  println!("turbo fish {}", turbo_fish);
  println!("hummus {:#?}", hum);

  for num in 0..=5 {
    match three_remainder(num) {
      Cow::Borrowed(a) => println!("this is a borrowed value, ||| {0}", a),
      Cow::Owned(b) => println!("this is an owned value||| {0}", b),
    }
  }

  let _m = LogsMessage::new(
    "message".to_owned(),
    "request_id".to_owned(),
    "object".to_owned(),
    "main".to_owned(),
  );

  Ok(("finally"))
}
