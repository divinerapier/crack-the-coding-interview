#![feature(str_as_mut_ptr)]

use std::os::raw::c_char;

fn main() {
    // println!("{}", char_in_string_is_unique_02("abc"));
    // println!("{}", char_in_string_is_unique_02("aba"));
    // println!("{}", char_in_string_is_unique_02("abcABC"));
    // println!("{}", char_in_string_is_unique_02("abcABCC"));
    // println!("{}", char_in_string_is_unique_01("abc"));
    // println!("{}", char_in_string_is_unique_01("aba"));
    // println!("{}", char_in_string_is_unique_01("abcABC"));
    // println!("{}", char_in_string_is_unique_01("abcABCC"));
    let mut s = "123".to_owned();
    unsafe {
        reverse(&mut s);
    }
    println!("{:?}", s);
}

// 8.1.1

// ascii
fn char_in_string_is_unique_01(s: &str) -> bool {
    if s.len() > 256 {
        return false;
    }
    let mut set: [u8; 256] = [0; 256];
    for &c in s.as_bytes() {
        set[c as usize] += 1;
        if set[c as usize] > 1 {
            return false;
        }
    }
    return true;
}

// chars
fn char_in_string_is_unique_02(s: &str) -> bool {
    if s.len() > 26 {
        return false;
    }
    let mut result: u64 = 0;
    for &c in s.as_bytes() {
        let c: char = c as char;

        let offset = if c >= 'a' && c <= 'z' {
            (c as i32) - ('a' as i32)
        } else if c >= 'A' && c <= 'Z' {
            (c as i32) - ('A' as i32) + 26
        } else {
            -1
        };
        if offset == -1 {
            return false;
        }
        if result & (1 << offset) > 0 {
            return false;
        }

        result = result | (1 << offset);
    }
    return true;
}

// 8.1.2

unsafe fn c_reverse(s: *mut c_char) {
    let mut begin = s.offset(0);
    let mut end = s.offset(0);
    loop {
        if *end == '\0' as i8 {
            end = end.offset(-1);
            break;
        }
        end = end.offset(1);
    }
    loop {
        if begin >= end {
            break;
        }
        let tmp = *begin;
        *begin = *end;
        *end = tmp;
        begin = begin.offset(1);
        end = end.offset(-1);
    }
}

unsafe fn reverse(s: &mut String) {
    if s.len() < 2 {
        return;
    }
    c_reverse(s.as_mut_ptr() as *mut c_char);
}
