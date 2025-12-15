const std = @import("std");

const MAX_LINE_LENGTH = 256;

fn part1(reader: *std.io.Reader) !u32 {
    var total: u32 = 0;
    total = 1;
    while (try reader.takeDelimiter('\n')) |line| {
        if (line.len == 0) break;
        std.debug.print("{s}\n", .{line});

        // FIXME: Thinking:
        //  - we need to consider three adjacent lines simultaneously
        //  - what we are computing is:
        //      1. a convolution with a 3x3 kernel, assuming zero padding around the
        //          entire grid. Kernel:
        //              111 
        //              101 
        //              111
        //      2. reduction by sum
    }
    return total;
}


pub fn main() !void {
    const file = try std.fs.cwd().openFile("input_04.txt", .{});
    defer file.close();

    var buffer: [MAX_LINE_LENGTH]u8 = undefined;
    var reader_wrapper = file.reader(&buffer);
    std.debug.print("Part 1: {d}\n", .{try part1(&reader_wrapper.interface)});
    try reader_wrapper.seekTo(0);
    std.debug.print("Part 1: {d}\n", .{try part1(&reader_wrapper.interface)});
    // std.debug.print("Part 2: {d}\n", .{try part2(&reader_wrapper.interface)});
}

const example_input =
    \\..@@.@@@@.
    \\@@@.@.@.@@
    \\@@@@@.@.@@
    \\@.@@@@..@.
    \\@@.@@@@.@@
    \\.@@@@@@@.@
    \\.@.@.@.@@@
    \\@.@@@.@@@@
    \\.@@@@@@@@.
    \\@.@.@@@.@.
;

test "example1" {
    var reader = std.io.Reader.fixed(example_input);
    try std.testing.expect(try part1(&reader) == 13);
}
