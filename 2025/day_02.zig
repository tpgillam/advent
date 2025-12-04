const std = @import("std");

const Range = struct { begin: u32, end: u32 };

fn parseRange(x: []const u8) !Range {
    var token_it = std.mem.splitScalar(u8, x, '-');
    const begin = try std.fmt.parseInt(u32, token_it.next() orelse return error.Moo, 10);
    const end = try std.fmt.parseInt(u32, token_it.next() orelse return error.Moo, 10);
    return Range{ .begin = begin, .end = end };
}

fn isInvalid(x: u32) bool {
    var buf: [10]u8 = undefined;
    const string = std.fmt.bufPrint(&buf, "{d}", .{x}) catch {
        // All u32 can fit into a 10 byte string.
        unreachable;
    };
    // If the string isn't an even length, it can't be invalid.
    if (@mod(string.len, 2) != 0) return false;

    // It's invalid if the first half matches the second half.
    return std.mem.eql(
        u8,
        string[0 .. string.len / 2],
        string[string.len / 2 .. string.len],
    );
}

fn part1(input: []const u8) !u32 {
    var pair_it = std.mem.splitScalar(u8, input, ',');
    var result: u32 = 0;
    while (pair_it.next()) |x| {
        const range = try parseRange(x);
        var i = range.begin;
        while (i <= range.end) {
            if (isInvalid(i)) result += i;
            i += 1;
        }
    }
    return result;
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const f = try std.fs.cwd().readFileAlloc(
        allocator,
        "input_01.txt",
        1 << 15, // HACK: read all of the bytes
    );

    std.debug.print("Part 1: {d}\n", .{try part1(f)});
}

test "example1" {
    const input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    try std.testing.expect(try part1(input) == 1227775554);
}
