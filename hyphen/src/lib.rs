#![deny(warnings)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HyphenError {
    #[error("an interior nul byte was found")]
    BadString(#[from] std::ffi::NulError),
    #[error("failed to hyphenate")]
    FailedHyphenate,
}

pub struct HyphenDict {
    dict: *mut hyphen_sys::_HyphenDict,
}

impl HyphenDict {
    pub fn new(path: &str) -> Result<Self, HyphenError> {
        let c_str = std::ffi::CString::new(path)?;
        let dict = unsafe { hyphen_sys::hnj_hyphen_load(c_str.as_ptr() as *const i8) };
        Ok(HyphenDict { dict })
    }

    pub fn hyphenate(&self, word: &str) -> Result<String, HyphenError> {
        // See <https://github.com/hunspell/hyphen/blob/73dd2967c8e1e4f6d7334ee9e539a323d6e66cbd/example.c#L136>.
        let hyphens: Vec<u8> = vec![0; word.len() + 5];

        // See <https://github.com/hunspell/hyphen/blob/73dd2967c8e1e4f6d7334ee9e539a323d6e66cbd/example.c#L156>.
        let hword: Vec<u8> = vec![0; word.len() * 2];

        let mut rep: *mut *mut ::std::os::raw::c_char = std::ptr::null_mut();

        let mut pos: *mut ::std::os::raw::c_int = std::ptr::null_mut();

        let mut cut: *mut ::std::os::raw::c_int = std::ptr::null_mut();

        let c_str = std::ffi::CString::new(word)?;
        let ret = unsafe {
            hyphen_sys::hnj_hyphen_hyphenate2(
                self.dict,
                c_str.as_ptr() as *const i8,
                word.len() as i32,
                hyphens.as_ptr() as *mut i8,
                hword.as_ptr() as *mut i8,
                &mut rep,
                &mut pos,
                &mut cut,
            )
        };
        if ret != 0 {
            return Err(HyphenError::FailedHyphenate);
        }

        let nul_range_end = hword
            .iter()
            .position(|&c| c == b'\0')
            .unwrap_or(hword.len());
        let actual = unsafe { String::from_utf8_unchecked(hword[0..nul_range_end].to_vec()) };
        if !rep.is_null() {
            for i in 0..word.len() {
                let rep_i = unsafe { *rep.add(i) };
                if !rep_i.is_null() {
                    unsafe { libc::free(rep_i as *mut libc::c_void) };
                }
            }

            unsafe { libc::free(rep as *mut libc::c_void) };
            unsafe { libc::free(pos as *mut libc::c_void) };
            unsafe { libc::free(cut as *mut libc::c_void) };
        }

        Ok(actual)
    }
}

impl Drop for HyphenDict {
    fn drop(&mut self) {
        unsafe {
            hyphen_sys::hnj_hyphen_free(self.dict);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hyphenate() {
        let dict =
            HyphenDict::new("/usr/share/hyphen/hyph_hu_HU.dic").expect("HyphenDict::new() failed");
        let hyphenated = dict
            .hyphenate("asszonnyal")
            .expect("HyphenDict::hyphenate() failed");
        assert_eq!(hyphenated, "asz=szony=nyal");
    }
}
