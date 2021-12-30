#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn hyphenate() {
        unsafe {
            let dict_path = "/usr/share/hyphen/hyph_hu_HU.dic";
            let c_str = CString::new(dict_path).unwrap();
            let dict = hnj_hyphen_load(c_str.as_ptr() as *const i8);

            let word = "asszonnyal";

            // See <https://github.com/hunspell/hyphen/blob/73dd2967c8e1e4f6d7334ee9e539a323d6e66cbd/example.c#L136>.
            let hyphens: Vec<u8> = vec![0; word.len() + 5];

            // See <https://github.com/hunspell/hyphen/blob/73dd2967c8e1e4f6d7334ee9e539a323d6e66cbd/example.c#L156>.
            let hword: Vec<u8> = vec![0; word.len() * 2];

            let mut rep: *mut *mut ::std::os::raw::c_char = std::ptr::null_mut();

            let mut pos: *mut ::std::os::raw::c_int = std::ptr::null_mut();

            let mut cut: *mut ::std::os::raw::c_int = std::ptr::null_mut();

            let c_str = CString::new(word).unwrap();
            let ret = hnj_hyphen_hyphenate2(
                dict,
                c_str.as_ptr() as *const i8,
                word.len() as i32,
                hyphens.as_ptr() as *mut i8,
                hword.as_ptr() as *mut i8,
                &mut rep,
                &mut pos,
                &mut cut,
            );
            assert_eq!(ret, 0);

            let nul_range_end = hword
                .iter()
                .position(|&c| c == b'\0')
                .unwrap_or(hword.len());
            let actual = String::from_utf8_unchecked(hword[0..nul_range_end].to_vec());
            assert_eq!(actual, "asz=szony=nyal");

            if !rep.is_null() {
                for i in 0..word.len() {
                    let rep_i = *rep.offset(i as isize);
                    if !rep_i.is_null() {
                        libc::free(rep_i as *mut libc::c_void);
                    }
                }

                libc::free(rep as *mut libc::c_void);
                libc::free(pos as *mut libc::c_void);
                libc::free(cut as *mut libc::c_void);
            }

            hnj_hyphen_free(dict);
        }
    }
}
