extern crate libc;
extern crate weechat_sys;

pub mod plugin;
pub mod weechat;

pub use weechat::Weechat;
pub use weechat::Buffer;
pub use weechat::Nick;
pub use plugin::WeechatPlugin;
pub use plugin::WeechatResult;
pub use plugin::WeechatPluginArgs;