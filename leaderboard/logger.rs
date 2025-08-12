use std::fmt::Arguments;

use afire::{
    Middleware, Request,
    prelude::MiddleResult,
    trace::{Formatter, Level as AfireLevel},
};
use log::{Level, RecordBuilder, trace};

pub struct AfireLogger;

impl Formatter for AfireLogger {
    fn format(&self, level: AfireLevel, _color: bool, msg: Arguments) {
        let level = match level {
            AfireLevel::Off => return,
            AfireLevel::Error => Level::Error,
            AfireLevel::Trace => Level::Info,
            AfireLevel::Debug => Level::Debug,
        };

        let record = RecordBuilder::new()
            .level(level)
            .target("afire::logger")
            .args(msg)
            .build();

        log::logger().log(&record);
    }
}

pub struct RequestLogger;

impl Middleware for RequestLogger {
    fn pre(&self, req: &mut Request) -> MiddleResult {
        trace!("{} {}{}", req.method, req.path, req.query);
        MiddleResult::Continue
    }
}
