const std = @import("std");

const Direction = enum { left, right };

fn get_direction(c: u8) !Direction {
    if (c == 'L') return Direction.left;
    if (c == 'R') return Direction.right;
    return error.InvalidDirection;
}

fn part1(input: []const u8) !i32 {
    var it = std.mem.splitScalar(u8, input, '\n');
    var num_zeros: i32 = 0;
    var dial_number: i32 = 50;
    while (it.next()) |x| {
        if (x.len == 0) continue;
        const direction = try get_direction(x[0]);
        const count = try std.fmt.parseInt(i32, x[1..], 10);

        const signed_count = switch (direction) {
            .left => -count,
            .right => count,
        };
        dial_number += signed_count;
        dial_number = @mod(dial_number, 100);

        if (dial_number == 0) num_zeros += 1;
    }
    std.debug.print("num_zeros = {d}\n", .{num_zeros});
    return num_zeros;
}

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    const f = try std.fs.cwd().readFileAlloc(
        allocator,
        "input_01.txt",
        1 << 15, // HACK: read all of the bytes
    );
    std.debug.print("Part 1: {d}\n", .{try part1(f)});
}

test "example_1" {
    const input =
        \\L68
        \\L30
        \\R48
        \\L5
        \\R60
        \\L55
        \\L1
        \\L99
        \\R14
        \\L82
    ;
    try std.testing.expect((try part1(input)) == 3);
}
