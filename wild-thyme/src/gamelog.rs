use std::fmt;
use wasm_bindgen::prelude::*;

pub struct GameLog {
    entries: Vec<LogEntry>,
    unread: Vec<LogEntry>,
}

#[derive(Clone)]
pub enum LogEntry {
    Action {
        subject: String,
        object: String,
        verb: String,
        suffix: String,
    },
    Quip {
        subject: String,
        quip: String,
    },
    Notification {
        notification: String,
    },
    Alert {
        alert: String,
    },
}

pub struct ReadLogEntry<'a> {
    pub read: bool,
    pub entry: &'a LogEntry,
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEntry::Action {
                subject,
                object,
                verb,
                suffix,
            } => write!(f, "{} {} {} {}", subject, verb, object, suffix),
            LogEntry::Quip { subject, quip } => write!(f, "{}: {}", subject, quip),
            LogEntry::Notification { notification } => write!(f, "{}", notification),
            LogEntry::Alert { alert } => write!(f, "{}", alert),
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

impl GameLog {
    pub fn new(new_entries: Vec<LogEntry>) -> GameLog {
        GameLog {
            entries: vec![],
            unread: new_entries,
        }
    }
    pub fn log(&mut self, entry: LogEntry) {
        #[allow(unused_unsafe)]
        unsafe {
            log(&entry.to_string());
        }
        self.unread.push(entry);
    }
    pub fn entries(&self) -> impl DoubleEndedIterator<Item = ReadLogEntry<'_>> {
        let read_entries = self.entries.iter().map(|e| ReadLogEntry {
            read: true,
            entry: e,
        });
        let unread_entries = self.unread.iter().map(|e| ReadLogEntry {
            read: false,
            entry: e,
        });
        read_entries.chain(unread_entries)
    }
    pub fn mark_all_read(&mut self) {
        let mut queue = vec![];
        while !self.unread.is_empty() {
            queue.push(
                self.unread
                    .pop()
                    .expect("should be guaranteed by while check"),
            );
        }
        while !queue.is_empty() {
            self.entries
                .push(queue.pop().expect("should be guaranteed by while check"));
        }
    }
}
