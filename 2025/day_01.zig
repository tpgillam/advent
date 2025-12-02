const std = @import("std");

const Direction = enum { left, right };

fn get_direction(c: u8) !Direction {
    if (c == 'L') return Direction.left;
    if (c == 'R') return Direction.right;
    return error.InvalidDirection;
}

const N = 100;

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
        dial_number = @mod(dial_number, N);

        if (dial_number == 0) num_zeros += 1;
    }
    return num_zeros;
}

fn part2(input: []const u8) !i32 {
    var it = std.mem.splitScalar(u8, input, '\n');
    var num_zeros: i32 = 0;
    var dial_number: i32 = 50;
    while (it.next()) |x| {
        if (x.len == 0) continue;
        const direction = try get_direction(x[0]);
        const count = try std.fmt.parseInt(i32, x[1..], 10);
        // std.debug.print("{} - {s}  (num_zeros={})\n", .{dial_number, x, num_zeros});

        switch (direction) {
            .left => {
                std.debug.assert(count > 0);
                // Ensure we don't double-count if we are starting on zero.
                if (dial_number == 0) dial_number += N;
                dial_number -= count;
                while (dial_number < 0) {
                    // PERF: sigh too tired
                    dial_number += N;
                    num_zeros += 1;
                }
                // Handle the case that we finished on a zero.
                if (dial_number == 0) num_zeros += 1;
            },
            .right => {
                dial_number += count;
                while (dial_number > (N - 1)) {
                    // PERF: sigh too tired
                    dial_number -= N;
                    num_zeros += 1;
                }
                // The finishing-on-zero case is already handled in the loop
            },
        }
    }
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
    std.debug.print("Part 2: {d}\n", .{try part2(f)});
}

const example_input =
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

test "example_1" {
    try std.testing.expect((try part1(example_input)) == 3);
}

test "example_2" {
    try std.testing.expect((try part2(example_input)) == 6);
}
