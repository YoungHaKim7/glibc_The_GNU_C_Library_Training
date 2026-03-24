#[derive(Debug, Clone, Copy)]
struct StringView<'a> {
    data: &'a str,
    count: usize,
}

impl<'a> StringView<'a> {
    fn sv(s: &'a str) -> Self {
        Self {
            data: s,
            count: s.len(),
        }
    }

    fn sv_chop_left(sv: &mut StringView, n: usize) {
        let n = n.min(sv.count);

        sv.data = &sv.data[n..];
        sv.count -= n;
    }
    fn sv_chop_right(sv: &mut StringView, n: usize) {
        let n = n.min(sv.count);

        sv.count -= n;
        sv.data = &sv.data[..sv.count];
    }
}

fn main() {
    let mut s = StringView::sv("Hello, World");

    StringView::sv_chop_right(&mut s, 3);
    StringView::sv_chop_left(&mut s, 2);

    println!("{}", s.data);
}
