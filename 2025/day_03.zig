const std = @import("std");

// Maximum length of any line; we uw
const MAX_LINE_LENGTH = 512;

fn maxJoltage(line: []const u8) u32 {
    // The 'tens' place is the largest digit found in [0:len(line) - 2].
    // The 'ones' place is the largest digit found after the 'tens' place.
    //
    // Initialise to zero; this is a useful placeholder as we know that we
    // will not read a zero.
    var tens: u32 = 0;
    var ones: u32 = 0;

    for (line, 0..) |x, i| {
        const val = x - '0';
        if (i < (line.len - 1)) {
            // We can look for a tens.
            if (val > tens) {
                // We want to update our tens place; this means we have
                // invalidated our previous choice for `ones`.
                tens = val;
                ones = 0;
                continue;
            }
        }

        if (val > ones) ones = val;
    }

    return 10 * tens + ones;
}

fn part1(reader: *std.io.Reader) !u32 {
    var total: u32 = 0;
    while (try reader.takeDelimiter('\n')) |line| {
        if (line.len == 0) break;
        // std.debug.print("{s}\n", .{line});
        total += maxJoltage(line);
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
