// main.zig

const std = @import("std");
const Io = std.Io;

const string_view_zig = @import("string_view_zig");

const StringView = struct {
    data: [*]const u8,
    count: usize,

    // String_View sv(const char *cstr)
    pub fn sv(cstr: [*:0]const u8) StringView {
        return StringView{
            .data = cstr,
            .count = std.mem.len(cstr),
        };
    }

    pub fn chopLeft(self: *StringView, n: usize) void {
        var k = n;
        if (k > self.count) {
            k = self.count;
        }

        self.count -= k;
        self.data += k;
    }

    pub fn chopRight(self: *StringView, n: usize) void {
        var k = n;
        if (k > self.count) {
            k = self.count;
        }

        self.count -= k;
    }
};

pub fn main(init: std.process.Init) !void {
    try std.Io.File.stdout().writeStreamingAll(init.io, "StringView zig lang (ver0.16.0-dev.2984+cb7d2b056)\n\n\n");

    var s = StringView.sv("Hello, World");

    s.chopRight(3);
    s.chopLeft(2);

    std.debug.print("{s}\n", .{s.data[0..s.count]});
}

// test "simple test" {}

// test "fuzz example" {}

// fn testOne(context: void, smith: *std.testing.Smith) !void {}
