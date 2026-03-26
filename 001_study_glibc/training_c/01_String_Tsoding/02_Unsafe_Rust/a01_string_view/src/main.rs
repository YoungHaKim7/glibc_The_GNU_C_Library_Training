use std::{ffi::CStr, os::raw::c_char, ptr};

#[repr(C)]
struct String_View {
    data: *const u8,
    count: usize,
}

// equivalent to strlen
fn c_strlen(mut s: *const u8) -> usize {
    let mut len = 0;

    unsafe {
        while *s != 0 {
            s = s.add(1);
            len += 1;
        }
    }

    len
}

// String_View sv(const char *cstr)
fn sv(cstr: *const u8) -> String_View {
    String_View {
        data: cstr,
        count: c_strlen(cstr),
    }
}

// void sv_chop_left(String_View *sv, size_t n)
fn sv_chop_left(sv: *mut String_View, mut n: usize) {
    unsafe {
        if n > (*sv).count {
            n = (*sv).count;
        }

        (*sv).count -= n;
        (*sv).data = (*sv).data.add(n);
    }
}

// void sv_chop_right(String_View *sv, size_t n)
fn sv_chop_right(sv: *mut String_View, mut n: usize) {
    unsafe {
        if n > (*sv).count {
            n = (*sv).count;
        }

        (*sv).count -= n;
    }
}

fn main() {
    unsafe {
        // C string literal
        let cstr = b"Hello, World\0";

        let mut s = sv(cstr.as_ptr());

        sv_chop_right(&mut s, 3);
        sv_chop_left(&mut s, 2);

        // printf("%.*s\n", (int)s.count, s.data);
        let slice = std::slice::from_raw_parts(s.data, s.count);

        let text = std::str::from_utf8_unchecked(slice);

        println!("{}", text);
    }
}
