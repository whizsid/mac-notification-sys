//! Wrappers around Apple notification APIs
#![warn(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![cfg(target_os = "macos")]
#![allow(improper_ctypes)]
#![no_std]

use icrate::Foundation::{
    // IOS 2.0 MacOS 10.0 MacCatalyst 13.0 TvOS 9.0 WatchOS 2.0 VisionOS 1.0
    NSDate,
    // IOS 2.0 MacOS 10.0 MacCatalyst 13.0 TvOS 9.0 WatchOS 2.0 VisionOS 1.0
    NSDefaultRunLoopMode,
    // IOS 2.0 MacOS 10.0 MacCatalyst 13.0 TvOS 9.0 WatchOS 2.0 VisionOS 1.0
    NSRunLoop};

#[cfg(feature="un")]
pub mod un;
#[cfg(feature="ns")]
pub mod ns;
pub mod error;
mod os;

/// Run the RunLoop once
pub fn run_ns_run_loop_once() {
    unsafe {
        let main_loop = NSRunLoop::mainRunLoop();
        let limit_date = NSDate::dateWithTimeIntervalSinceNow(0.1);
        main_loop.acceptInputForMode_beforeDate(NSDefaultRunLoopMode, &limit_date);
    }
}
