//! Errors returning from the library
use core::fmt::Debug;

use icrate::Foundation::NSError;
use objc2::rc::Id;

/// All the errors that returning from this library
#[derive(Clone)]
pub enum NotificationError<'a> {
    /// Error from the Objective C User Notifications framework
    NSError(Id<NSError>),
    /// Not supported for the current OS version
    NotSupported,
    /// When invalid URL provided
    InvalidUrl(&'a str),
    /// Required field not provided
    ValidationError(&'a str),
}

impl <'a> From<Id<NSError>> for NotificationError<'a> {
    fn from(value: Id<NSError>) -> Self {
        Self::NSError(value)
    }
}

impl <'a> Debug for NotificationError<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            NotificationError::NSError(ns_error) => {
                f.debug_struct("NSError")
                    .field("code", &ns_error.code())
                    .field("domain", &ns_error.domain().to_string())
                    .field("message", &ns_error.localizedDescription().to_string())
                    .finish()
            },
            NotificationError::NotSupported => {
                f.write_str("NotSupported")
            },
            NotificationError::InvalidUrl(url) => {
                f.debug_tuple("InvalidUrl").field(url).finish()
            },
            NotificationError::ValidationError(field) => {
                f.debug_tuple("ValidationError").field(field).finish()
            }
        }
    }
}
