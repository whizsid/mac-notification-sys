use objc2::msg_send_id;
use objc2::rc::Id;
use objc2::runtime::NSObjectProtocol;
use objc2::{declare_class, mutability::InteriorMutable, ClassType};
// IOS 2.0 MacOS 10.0 MacCatalyst 13.1 TvOS 9.0 WatchOS 2.0 VisionOS 1.0
use icrate::block2::Block;
use icrate::Foundation::NSObject;
use icrate::UserNotifications::{
    // IOS 10.0 MacOS 10.14 MacCatalyst 13.1 WatchOS 3.0 VisionOS 1.0
    UNNotificationResponse,
    // IOS 10.0 MacOS 10.14 MacCatalyst 13.1 TvOS 10.0 WatchOS 3.0 VisionOS 1.0
    UNUserNotificationCenter,
    // IOS 10.0 MacOS 10.14 MacCatalyst 13.1 TvOS 10.0 WatchOS 3.0 VisionOS 1.0
    UNUserNotificationCenterDelegate,
};

declare_class! {
    #[derive(Debug)]
    pub(super) struct RustNotificationDelegate {

    }

    unsafe impl ClassType for RustNotificationDelegate {
        type Super = NSObject;
        type Mutability = InteriorMutable;
        const NAME: &'static str = "RustNotificationDelegate";
    }

    unsafe impl UNUserNotificationCenterDelegate for RustNotificationDelegate {
        #[method(userNotificationCenter:didReceiveNotificationResponse:withCompletionHandler:)]
        unsafe fn userNotificationCenter_didReceiveNotificationResponse_withCompletionHandler(
            &self,
            center: &UNUserNotificationCenter,
            response: &UNNotificationResponse,
            completion_handler: &Block<(), ()>,
        ) {
            println!("Received Response");
        }
    }
}

unsafe impl NSObjectProtocol for RustNotificationDelegate {}

impl RustNotificationDelegate {
    pub fn new() -> Id<Self> {
        unsafe { msg_send_id![Self::alloc(), init] }
    }
}
