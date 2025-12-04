const std = @import("std");

const Range = struct { begin: u64, end: u64 };

fn parseRange(x: []const u8) !Range {
    // std.debug.print("'{s}'\n", .{x});
    var token_it = std.mem.splitScalar(u8, x, '-');
    const begin_str = token_it.next() orelse return error.BadID;
    const end_str = token_it.next() orelse return error.BadID;
    const begin = try std.fmt.parseInt(u64, begin_str, 10);
    const end = try std.fmt.parseInt(u64, end_str, 10);
    return Range{ .begin = begin, .end = end };
}

// NOTE: this buffer must be global to ensure that it has static lifetime.
//  It's all rather dangerously exciting that idToSlice will return a slice whose
//  value will be invalidated when it is next called.
var buf20: [20]u8 = undefined;

fn idToSlice(x: u64) []const u8 {
    return std.fmt.bufPrint(&buf20, "{d}", .{x}) catch {
        // All u64 can fit into a 20 byte string.
        unreachable;
    };
}

fn isInvalidNParts(string: []const u8, n_parts: usize) bool {
    // If the string isn't an even length, it can't be invalid.
    if (@mod(string.len, n_parts) != 0) return false;
    const m = string.len / n_parts;
    // std.debug.print("  {s}\n", .{string});

    // It's invalid if all segments are equal.
    for (0..(n_parts - 1)) |i_part| {
        // std.debug.print("  {s}  {d}\n", .{string, i_part});
        if (!std.mem.eql(
            u8,
            string[i_part * m .. (i_part + 1) * m],
            string[(i_part + 1) * m .. (i_part + 2) * m],
        )) return false;
    }
    return true;
}

fn isInvalidPart1(x: u64) bool {
    const string = idToSlice(x);
    return isInvalidNParts(string, 2);
}

fn isInvalidPart2(x: u64) bool {
    const string = idToSlice(x);
    for (2..(string.len + 1)) |n| {
        if (isInvalidNParts(string, n)) return true;
    }
    return false;
}

/// Remove any trailing newline from x.
fn removeTrailingNewline(x: []const u8) []const u8 {
    if (x.len == 0) return x;
    if (x[x.len - 1] == '\n') return x[0 .. x.len - 1];
    return x;
}

const Part = enum { part1, part2 };

fn solve(comptime part: Part, input: []const u8) !u64 {
    var pair_it = std.mem.splitScalar(u8, removeTrailingNewline(input), ',');
    var result: u64 = 0;
    while (pair_it.next()) |x| {
        const range = try parseRange(x);
        var i = range.begin;
        while (i <= range.end) {
            const invalid = comptime switch (part) {
                .part1 => isInvalidPart1,
                .part2 => isInvalidPart2,
            };
            if (invalid(i)) result += i;
            // std.debug.print("{:>15}  {}\n", .{ i, result });
            i += 1;
        }
    }
    return result;
}

fn part1(input: []const u8) !u64 {
    return solve(.part1, input);
}

fn part2(input: []const u8) !u64 {
    return solve(.part2, input);
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const f = try std.fs.cwd().readFileAlloc(
        allocator,
        "input_02.txt",
        1 << 15, // HACK: read all of the bytes
    );

    std.debug.print("Part 1: {d}\n", .{try part1(f)});
    std.debug.print("Part 2: {d}\n", .{try part2(f)});
}

const example_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

test "example1" {
    try std.testing.expect(try part1(example_input) == 1227775554);
}
test "example2" {
    try std.testing.expect(try part2(example_input) == 4174379265);
}
