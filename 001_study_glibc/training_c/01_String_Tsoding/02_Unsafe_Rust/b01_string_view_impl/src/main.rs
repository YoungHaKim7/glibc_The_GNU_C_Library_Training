struct StringView {
    data: *const u8,
    count: usize,
}

impl StringView {
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
    fn sv(cstr: *const u8) -> Self {
        Self {
            data: cstr,
            count: Self::c_strlen(cstr),
        }
    }

    // void sv_chop_left(String_View *sv, size_t n)
    fn sv_chop_left(sv: *mut StringView, mut n: usize) {
        unsafe {
            if n > (*sv).count {
                n = (*sv).count;
            }

            (*sv).count -= n;
            (*sv).data = (*sv).data.add(n);
        }
    }

    // void sv_chop_right(String_View *sv, size_t n)
    fn sv_chop_right(sv: *mut StringView, mut n: usize) {
        unsafe {
            if n > (*sv).count {
                n = (*sv).count;
            }

            (*sv).count -= n;
        }
    }
}

fn main() {
    unsafe {
        // C string literal
        let cstr = b"Hello, World\0";

        let mut s = StringView::sv(cstr.as_ptr());

        StringView::sv_chop_right(&mut s, 3);
        StringView::sv_chop_left(&mut s, 2);

        // printf("%.*s\n", (int)s.count, s.data);
        let slice = std::slice::from_raw_parts(s.data, s.count);

        let text = std::str::from_utf8_unchecked(slice);

        println!("{}", text);
    }
}
