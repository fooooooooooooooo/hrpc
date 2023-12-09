use std::sync::atomic::{AtomicU8, Ordering};

pub const NONE: u8 = 0;

static READING: AtomicU8 = AtomicU8::new(NONE);

pub fn set(reading: u8) { READING.store(reading, Ordering::Relaxed); }

pub fn get() -> u8 { READING.load(Ordering::Relaxed) }
