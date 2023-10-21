macro_rules! erase_lifetime {
    ($ptr:expr) => {{
        let temp: *mut _ = &mut $ptr;
        unsafe { &mut *temp }
    }};
}

pub(crate) use erase_lifetime;
