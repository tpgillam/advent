const std = @import("std");

pub fn main() void {
    std.debug.print(
        "Hello {[a]s} {[b]s}.\n",
        .{ .a = "moo", .b = "foo" },
    );
}

test "moo" {
    try std.testing.expect(1 == (2 - 1));
    try std.testing.expect(2 == (2 + 1));
    try std.testing.expect(3 == (2 + 1));
}
