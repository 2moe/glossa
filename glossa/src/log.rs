#![allow(unused_macros, unused_imports)]

#[cfg(feature = "log")]
macro_rules! error {
    ($($arg:tt)*) => {{
        log::error!($($arg)*);
    }}
}

#[cfg(not(feature = "log"))]
macro_rules! error {
    ($($arg:tt)*) => {{}};
}

#[cfg(feature = "log")]
macro_rules! warning {
    ($($arg:tt)*) => {{
        log::warn!($($arg)*);
    }}
}

#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {{}};
}

#[cfg(feature = "log")]
macro_rules! info {
    ($($arg:tt)*) => {{
        log::info!($($arg)*);
    }}
}

#[cfg(not(feature = "log"))]
macro_rules! info {
    ($($arg:tt)*) => {{}};
}

#[cfg(feature = "log")]
macro_rules! debug {
    ($($arg:tt)*) => {{
        log::debug!($($arg)*);
    }}
}

#[cfg(not(feature = "log"))]
macro_rules! debug {
    ($($arg:tt)*) => {{}};
}

#[cfg(feature = "log")]
macro_rules! trace {
    ($($arg:tt)*) => {{
        log::trace!($($arg)*);
    }}
}

#[cfg(not(feature = "log"))]
macro_rules! trace {
    ($($arg:tt)*) => {{}};
}

pub(crate) use debug;
pub(crate) use error;
pub(crate) use info;
pub(crate) use trace;
pub(crate) use warning;
