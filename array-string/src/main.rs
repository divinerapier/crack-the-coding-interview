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
    {
        let mut s = "123".to_owned();
        unsafe {
            reverse(&mut s);
        }
        println!("{:?}", s);
    }
    {
        let s = "Mr John Smith";
        unsafe {
            println!("{}", urlify(&s));
        }
    }
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

// 8.1.4

// URLify: Write a method to replace all spaces in a string with '%20'. You may assume that the string
// has sufficient space at the end to hold the additional characters, and that you are given the "true"
// length of the string. (Note: If implementing in Java, please use a character array so that you can
// perform this operation in place.)
// EXAMPLE
// Input: "Mr 3ohn Smit h 13
// Output: "Mr%203ohn%20Smith"

unsafe fn urlify(s: &str) -> String {
    let result = c_urlify(s.as_ptr() as *const c_char);
    let length = c_strlen(result);
    String::from_raw_parts(result as *mut u8, length, length)
}

unsafe fn c_urlify(s: *const c_char) -> *mut c_char {
    let count = c_str_count(s, ' ' as c_char);
    let length = c_strlen(s);
    let buffer: *mut c_char = malloc(length + count << 1);
    println!("length: {}", length);
    println!("count: {}", count << 1);
    let (mut scursor, mut tcursor) = (s.offset(0), buffer.offset(0));

    loop {
        if scursor == (0 as *mut i8) || *scursor == ('\0' as i8) {
            *tcursor = 0;
            break;
        }
        if *scursor != (' ' as c_char) {
            *tcursor = *scursor;
        } else {
            *tcursor = '%' as c_char;
            *tcursor.offset(1) = '2' as c_char;
            *tcursor.offset(2) = '0' as c_char;
            tcursor = tcursor.offset(2);
        }
        scursor = scursor.offset(1);
        tcursor = tcursor.offset(1);
    }

    return buffer;
}

unsafe fn malloc<T>(size: usize) -> *mut T
where
    T: Sized,
{
    let mut buffer: std::vec::Vec<T> = std::vec::Vec::with_capacity(size);
    let result = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    result
}

unsafe fn c_str_count(s: *const c_char, c: c_char) -> usize {
    let mut count: usize = 0;
    let mut ptr = s.offset(0);
    loop {
        if ptr == (0 as *const i8) || *ptr == ('\0' as i8) {
            break;
        }
        ptr = ptr.offset(1);
        if *ptr == c {
            count += 1;
        }
    }
    return count;
}

unsafe fn c_strlen(s: *const c_char) -> usize {
    let mut count: usize = 0;
    let mut ptr = s.offset(0);
    loop {
        if ptr == (0 as *const i8) || *ptr == ('\0' as i8) {
            break;
        }
        ptr = ptr.offset(1);
    }
    return count;
}
