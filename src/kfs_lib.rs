pub fn strlen(s: *const u8) -> usize {
    let mut len = 0;
    unsafe {
        while *s.add(len) != 0 {
            len += 1;
        }
    }
    len
}

pub fn strcmp(s1: *const u8, s2: *const u8) -> i32 {
    unsafe {
        let mut i = 0;
        loop {
            let c1 = *s1.add(i);
            let c2 = *s2.add(i);
            
            if c1 != c2 {
                return c1 as i32 - c2 as i32;
            }
            
            if c1 == 0 {
                return 0;
            }
            
            i += 1;
        }
    }
}

pub fn strcpy(dest: *mut u8, src: *const u8) -> *mut u8 {
    unsafe {
        let mut i = 0;
        loop {
            let c = *src.add(i);
            *dest.add(i) = c;
            if c == 0 {
                break;
            }
            i += 1;
        }
    }
    dest
}

pub fn strncpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        for i in 0..n {
            let c = *src.add(i);
            *dest.add(i) = c;
            if c == 0 {
                // Remplir le reste avec des zéros
                for j in (i + 1)..n {
                    *dest.add(j) = 0;
                }
                break;
            }
        }
    }
    dest
}

pub fn memset(ptr: *mut u8, value: u8, size: usize) -> *mut u8 {
    unsafe {
        for i in 0..size {
            *ptr.add(i) = value;
        }
    }
    ptr
}

pub fn memcpy(dest: *mut u8, src: *const u8, size: usize) -> *mut u8 {
    unsafe {
        for i in 0..size {
            *dest.add(i) = *src.add(i);
        }
    }
    dest
}

pub fn memcmp(ptr1: *const u8, ptr2: *const u8, size: usize) -> i32 {
    unsafe {
        for i in 0..size {
            let c1 = *ptr1.add(i);
            let c2 = *ptr2.add(i);
            if c1 != c2 {
                return c1 as i32 - c2 as i32;
            }
        }
    }
    0
}

pub fn itoa(mut value: i32, buffer: &mut [u8], base: u32) -> &str {
    if buffer.is_empty() {
        return "";
    }

    let mut pos = 0;
    let is_negative = value < 0;
    
    if is_negative {
        value = -value;
    }
    
    if value == 0 {
        buffer[pos] = b'0';
        pos += 1;
    } else {
        while value > 0 && pos < buffer.len() - 1 {
            let digit = (value % base as i32) as u8;
            buffer[pos] = if digit < 10 {
                b'0' + digit
            } else {
                b'a' + digit - 10
            };
            value /= base as i32;
            pos += 1;
        }
        
        if is_negative && pos < buffer.len() - 1 {
            buffer[pos] = b'-';
            pos += 1;
        }
        
        buffer[..pos].reverse();
    }
    
    if pos < buffer.len() {
        buffer[pos] = 0;
    }
    
    unsafe {
        core::str::from_utf8_unchecked(&buffer[..pos])
    }
}

use crate::{print, println};
use crate::vga_buffer::Color;

pub fn kprintf(format: &str, args: &[PrintfArg]) {
    let mut chars = format.chars().peekable();
    let mut arg_index = 0;
    
    while let Some(ch) = chars.next() {
        if ch == '%' {
            if let Some(&next_ch) = chars.peek() {
                if next_ch == '%' {
                    print!("%");
                    chars.next(); // consume the second %
                } else if arg_index < args.len() {
                    match chars.next() {
                        Some('d') | Some('i') => {
                            if let PrintfArg::Int(val) = args[arg_index] {
                                print!("{}", val);
                            }
                            arg_index += 1;
                        }
                        Some('x') => {
                            if let PrintfArg::Int(val) = args[arg_index] {
                                print!("{:x}", val);
                            }
                            arg_index += 1;
                        }
                        Some('X') => {
                            if let PrintfArg::Int(val) = args[arg_index] {
                                print!("{:X}", val);
                            }
                            arg_index += 1;
                        }
                        Some('s') => {
                            if let PrintfArg::Str(val) = args[arg_index] {
                                print!("{}", val);
                            }
                            arg_index += 1;
                        }
                        Some('c') => {
                            if let PrintfArg::Char(val) = args[arg_index] {
                                print!("{}", val);
                            }
                            arg_index += 1;
                        }
                        Some('p') => {
                            if let PrintfArg::Ptr(val) = args[arg_index] {
                                print!("0x{:x}", val as usize);
                            }
                            arg_index += 1;
                        }
                        _ => print!("{}", ch),
                    }
                } else {
                    print!("{}", ch);
                }
            } else {
                print!("{}", ch);
            }
        } else {
            print!("{}", ch);
        }
    }
}

pub fn debug_printf(color: Color, format: &str, args: &[PrintfArg]) {
    crate::vga_buffer::set_color(color, Color::Black);
    print!("[DEBUG] ");
    kprintf(format, args);
    println!();
    crate::vga_buffer::set_color(Color::White, Color::Black); // Restore
}

pub fn error_printf(format: &str, args: &[PrintfArg]) {
    debug_printf(Color::Red, format, args);
}

pub fn info_printf(format: &str, args: &[PrintfArg]) {
    debug_printf(Color::LightBlue, format, args);
}

pub fn warn_printf(format: &str, args: &[PrintfArg]) {
    debug_printf(Color::Yellow, format, args);
}

pub fn success_printf(format: &str, args: &[PrintfArg]) {
    debug_printf(Color::Green, format, args);
}

#[derive(Debug, Clone, Copy)]
pub enum PrintfArg<'a> {
    Int(i32),
    Str(&'a str),
    Char(char),
    Ptr(*const u8),
}

#[macro_export]
macro_rules! kprintf {
    ($fmt:expr) => {
        $crate::kfs_lib::kprintf($fmt, &[])
    };
    ($fmt:expr, $($arg:expr),*) => {
        $crate::kfs_lib::kprintf($fmt, &[$($crate::kfs_lib::PrintfArg::from($arg)),*])
    };
}

#[macro_export]
macro_rules! debug_print {
    ($fmt:expr) => {
        $crate::kfs_lib::debug_printf($crate::vga_buffer::Color::LightBlue, $fmt, &[])
    };
    ($fmt:expr, $($arg:expr),*) => {
        $crate::kfs_lib::debug_printf($crate::vga_buffer::Color::LightBlue, $fmt, &[$($crate::kfs_lib::PrintfArg::from($arg)),*])
    };
}

#[macro_export]
macro_rules! error_print {
    ($fmt:expr) => {
        $crate::kfs_lib::error_printf($fmt, &[])
    };
    ($fmt:expr, $($arg:expr),*) => {
        $crate::kfs_lib::error_printf($fmt, &[$($crate::kfs_lib::PrintfArg::from($arg)),*])
    };
}

impl<'a> From<i32> for PrintfArg<'a> {
    fn from(val: i32) -> Self {
        PrintfArg::Int(val)
    }
}

impl<'a> From<&'a str> for PrintfArg<'a> {
    fn from(val: &'a str) -> Self {
        PrintfArg::Str(val)
    }
}

impl<'a> From<char> for PrintfArg<'a> {
    fn from(val: char) -> Self {
        PrintfArg::Char(val)
    }
}

impl<'a> From<*const u8> for PrintfArg<'a> {
    fn from(val: *const u8) -> Self {
        PrintfArg::Ptr(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test_case]
    fn test_strlen() {
        let test_str = b"Hello\0";
        assert_eq!(strlen(test_str.as_ptr()), 5);
    }
    
    #[test_case]
    fn test_strcmp() {
        let str1 = b"Hello\0";
        let str2 = b"Hello\0";
        let str3 = b"World\0";
        
        assert_eq!(strcmp(str1.as_ptr(), str2.as_ptr()), 0);
        assert!(strcmp(str1.as_ptr(), str3.as_ptr()) < 0);
    }
}
