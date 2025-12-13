const std = @import("std");

// Maximum length of any line; we uw
const MAX_LINE_LENGTH = 512;

fn part1(reader: *std.io.Reader) !i32 {
    var total: i32 = 0;
    while (try reader.takeDelimiter('\n')) |line| {
        if (line.len == 0) break;
        std.debug.print("{s}\n", .{line});
        total += 1;
    }
    return total;
}

pub fn main() !void {
    const file = try std.fs.cwd().openFile("input_03.txt", .{});
    defer file.close();

    var buffer: [MAX_LINE_LENGTH]u8 = undefined;
    var reader_wrapper = file.reader(&buffer);
    std.debug.print("Part 1: {d}\n", .{try part1(&reader_wrapper.interface)});
    try reader_wrapper.seekTo(0);
    std.debug.print("Part 1: {d}\n", .{try part1(&reader_wrapper.interface)});

}

test "example1" {
    const example_input =
        \\987654321111111
        \\811111111111119
        \\234234234234278
        \\818181911112111
    ;
    var reader = std.io.Reader.fixed(example_input);
    try std.testing.expect(try part1(&reader) == 357);
}
