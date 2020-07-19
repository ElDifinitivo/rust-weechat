use std::collections::HashMap;
use std::ffi::c_void;

use weechat_sys::{t_hashtable, WEECHAT_HASHTABLE_STRING};

use crate::{LossyCString, Weechat};

impl Weechat {
    pub(crate) fn hashmap_to_weechat(&self, hashmap: HashMap<&str, &str>) -> *mut t_hashtable {
        let hashtable_new = self.get().hashtable_new.unwrap();

        let table_type: *const i8 = WEECHAT_HASHTABLE_STRING as *const _ as *const i8;

        let hashtable = unsafe { hashtable_new(8, table_type, table_type, None, None) };

        for (key, value) in hashmap {
            let key = LossyCString::new(key);
            let value = LossyCString::new(value);

            unsafe {
                self.get().hashtable_set.unwrap()(
                    hashtable,
                    key.as_ptr() as *const c_void,
                    value.as_ptr() as *const c_void,
                );
            }
        }

        hashtable
    }
}
