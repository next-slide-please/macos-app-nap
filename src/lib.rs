#![allow(non_snake_case)]

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

use cocoa_foundation::base::{nil};
use cocoa_foundation::foundation::{NSProcessInfo, NSString};

#[cfg(target_os = "macos")]
#[link(name = "thread_priority_helper")]
extern "C" {
    fn setMaxPriority();
}

// Prevent display from sleeping/powering down, prevent system
// from sleeping, prevent sudden termination for any reason.
#[cfg(target_os = "macos")]
pub fn prevent() {
    let NSActivityIdleDisplaySleepDisabled = 1u64 << 40;
    let NSActivityIdleSystemSleepDisabled = 1u64 << 20;
    let NSActivitySuddenTerminationDisabled = 1u64 << 14;
    let NSActivityAutomaticTerminationDisabled = 1u64 << 15;
    let NSActivityUserInitiated = 0x00FFFFFFu64 | NSActivityIdleSystemSleepDisabled;
    let NSActivityLatencyCritical = 0xFF00000000u64;

    let options = NSActivityIdleDisplaySleepDisabled
        | NSActivityIdleSystemSleepDisabled
        | NSActivitySuddenTerminationDisabled
        | NSActivityAutomaticTerminationDisabled;
    let options = options | NSActivityUserInitiated | NSActivityLatencyCritical;

    unsafe {
        let pinfo = NSProcessInfo::processInfo(nil);
        let s = NSString::alloc(nil).init_str("prevent app nap");
        let _:() = msg_send![pinfo, beginActivityWithOptions:options reason:s];

        setMaxPriority();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::prevent();
    }
}
