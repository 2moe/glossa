#![allow(unused_macros, unused_imports)]

#[cfg(feature = "log")]
#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {{
        $crate::assets::log::error!($($arg)*);
    }}
}

#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {{}};
}

#[cfg(feature = "log")]
#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {{
        $crate::assets::log::warn!($($arg)*);
    }}
}

#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {{}};
}

#[cfg(feature = "log")]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::assets::log::info!($($arg)*);
    }}
}

#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{}};
}

#[cfg(feature = "log")]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        $crate::assets::log::debug!($($arg)*);
    }}
}

#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{}};
}

#[cfg(feature = "log")]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{
        $crate::assets::log::trace!($($arg)*);
    }}
}

#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{}};
}

pub use debug;
pub use err;
pub use info;
pub use trace;
pub use warning;
