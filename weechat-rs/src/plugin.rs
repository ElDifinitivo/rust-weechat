use libc::{c_int, c_char};
use weechat::Weechat;
use std::ffi::CStr;
use std::ops::Index;

extern crate weechat_sys;

#[macro_export]
macro_rules! weechat_plugin(
    ($plugin:ty, $($name:ident: $value:expr; $length:expr),+) => {
        static mut __PLUGIN: Option<$plugin> = None;
        #[no_mangle]
        pub static mut weechat_plugin_api_version: [u8; weechat_sys::WEECHAT_PLUGIN_API_VERSION_LENGTH]
            = *weechat_sys::WEECHAT_PLUGIN_API_VERSION;

        #[no_mangle]
        pub extern "C" fn weechat_plugin_init(plugin: *mut weechat_sys::t_weechat_plugin,
                                              argc: libc::c_int, argv: *mut *mut ::libc::c_char) -> libc::c_int {
            let plugin = Weechat::from_ptr(plugin);
            let args = WeechatPluginArgs {
                argc: argc as u32,
                argv: argv,
            };
            match <$plugin as $crate::WeechatPlugin>::init(plugin, args) {
                Ok(p) => {
                    unsafe {
                        __PLUGIN = Some(p)
                    }
                    return weechat_sys::WEECHAT_RC_OK;
                }
                Err(_e) => {
                    return weechat_sys::WEECHAT_RC_ERROR;
                }
            }
        }
        #[no_mangle]
        pub extern "C" fn weechat_plugin_end(_plugin: *mut weechat_sys::t_weechat_plugin) -> ::libc::c_int {
            unsafe {
                // Invokes drop() on __PLUGIN, which should be used for cleanup.
                __PLUGIN = None;
            }

            weechat_sys::WEECHAT_RC_OK
        }
        $(
            weechat_plugin!(@attribute $name, $value, $length);
        )*

    };

    (@attribute $name:ident, $value:expr, $length:expr) => {
        weechat_plugin_info!($name, $value, $length);
    };
);

pub trait WeechatPlugin: Sized {
    fn init(weechat: Weechat, args: WeechatPluginArgs) -> WeechatResult<Self>;
}

pub struct Error(c_int);
pub type WeechatResult<T> = Result<T, Error>;

pub struct WeechatPluginArgs {
    pub argc: u32,
    pub argv: *mut *mut c_char,
}

impl WeechatPluginArgs {
    pub fn len(&self) -> usize {
        self.argc as usize
    }
}

impl Index<usize> for WeechatPluginArgs {
    type Output = CStr;

    fn index<'a>(&'a self, index: usize) -> &'a CStr {
        assert!(index < self.len());

        unsafe {
            let ptr = self.argv.offset(index as isize);
            CStr::from_ptr(ptr as *const c_char)
        }
    }
}

#[macro_export]
macro_rules! weechat_plugin_name(
    ($name:expr, $length:expr) => {
        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static weechat_plugin_name: [u8; $length] = *$name;
    }
);

#[macro_export]
macro_rules! weechat_plugin_author(
    ($author:expr, $length:expr) => {
        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static mut weechat_plugin_author: [u8; $length] = *$author;
    }
);

#[macro_export]
macro_rules! weechat_plugin_description(
    ($description:expr, $length:expr) => {
        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static mut weechat_plugin_description: [u8; $length] = *$description;
    }
);

#[macro_export]
macro_rules! weechat_plugin_version(
    ($version:expr, $length:expr) => {
        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static mut weechat_plugin_version: [u8; $length] = *$version;
    }
);

#[macro_export]
macro_rules! weechat_plugin_license(
    ($license:expr, $length:expr) => {
        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static mut weechat_plugin_license: [u8; $length] = *$license;
    }
);

#[macro_export]
macro_rules! weechat_plugin_info(
    (name, $name:expr, $length:expr) => {
        weechat_plugin_name!($name, $length);
    };
    (author, $author:expr, $length:expr) => {
        weechat_plugin_author!($author, $length);
    };
    (description, $description:expr, $length:expr) => {
        weechat_plugin_description!($description, $length);
    };
    (version, $version:expr, $length:expr) => {
        weechat_plugin_version!($version, $length);
    };
    (license, $license:expr, $length:expr) => {
        weechat_plugin_license!($license, $length);
    };
);