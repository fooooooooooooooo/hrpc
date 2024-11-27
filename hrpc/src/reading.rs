use std::fmt::Display;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

const ZERO: u8 = 0;

static READING: AtomicU8 = AtomicU8::new(ZERO);
static FROZEN: AtomicBool = AtomicBool::new(false);
static NO_VALUE: AtomicBool = AtomicBool::new(false);

pub fn set(reading: Reading) {
  match reading {
    Reading::None => {
      READING.store(ZERO, Ordering::Relaxed);
      FROZEN.store(false, Ordering::Relaxed);
      NO_VALUE.store(true, Ordering::Relaxed);
    }
    Reading::Frozen(reading) => {
      READING.store(reading, Ordering::Relaxed);
      FROZEN.store(true, Ordering::Relaxed);
      NO_VALUE.store(false, Ordering::Relaxed);
    }
    Reading::Value(reading) => {
      READING.store(reading, Ordering::Relaxed);
      FROZEN.store(false, Ordering::Relaxed);
      NO_VALUE.store(false, Ordering::Relaxed);
    }
  }
}

pub fn get() -> Reading {
  if NO_VALUE.load(Ordering::Relaxed) {
    return Reading::None;
  }

  let reading = READING.load(Ordering::Relaxed);

  if FROZEN.load(Ordering::Relaxed) {
    return Reading::Frozen(reading);
  }

  Reading::Value(reading)
}

pub enum Reading {
  None,
  Frozen(u8),
  Value(u8),
}

impl Reading {
  pub fn as_u8(&self) -> u8 {
    match self {
      Reading::None => ZERO,
      Reading::Frozen(reading) => *reading,
      Reading::Value(reading) => *reading,
    }
  }

  pub fn is_none(&self) -> bool {
    matches!(self, Reading::None)
  }
}

impl Display for Reading {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Reading::None => write!(f, "None"),
      Reading::Frozen(reading) => write!(f, "~{}", reading),
      Reading::Value(reading) => write!(f, "{}", reading),
    }
  }
}
