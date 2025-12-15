const std = @import("std");

// Maximum length of any line; we use this to ensure our reader's buffer
// is sufficiently large.
const MAX_LINE_LENGTH = 512;

fn maxJoltage2(line: []const u8) u32 {
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

fn maxJoltageN(line: []const u8, comptime n: usize) u64 {
    var digits: [n]u64 = .{0} ** n;

    outer: for (line, 0..) |x, i| {
        const val = x - '0';
        for (0..n) |i_digit| {
            // Compute the exponent of 10 that gives us the multiplier of this
            // digit.
            const exponent = (n - i_digit) - 1;
            if (i < (line.len - exponent)) {
                if (val > digits[i_digit]) {
                    // Set this exponent; this means we must invalidate the
                    // remaining digits, and the continue.
                    digits[i_digit] = val;

                    // NOTE: to perform the invalidation we need only
                    // invalidate the _next_ digit, if there is one.
                    if ((i_digit + 1) < n) digits[i_digit + 1] = 0;

                    // Abort the inner loop over exponents, and move onto the
                    // next character.
                    continue :outer;
                }
            }

            // std.debug.print("{d}\n", .{i_digit});
            // std.debug.print("{any}\n", .{digits});
        }
    }

    var total: u64 = 0;
    var multiplier: u64 = 1;
    for (0..n) |i| {
        total += multiplier * digits[n - 1 - i];
        multiplier *= 10;
    }
    return total;
}

fn part1(reader: *std.io.Reader) !u32 {
    var total: u32 = 0;
    while (try reader.takeDelimiter('\n')) |line| {
        if (line.len == 0) break;
        // std.debug.print("{s}\n", .{line});
        total += maxJoltage2(line);
    }
    return total;
}

fn part2(reader: *std.io.Reader) !u64 {
    var total: u64 = 0;
    while (try reader.takeDelimiter('\n')) |line| {
        if (line.len == 0) break;
        total += maxJoltageN(line, 12);
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
    std.debug.print("Part 2: {d}\n", .{try part2(&reader_wrapper.interface)});
}

const example_input =
    \\987654321111111
    \\811111111111119
    \\234234234234278
    \\818181911112111
;
test "example1" {
    var reader = std.io.Reader.fixed(example_input);
    try std.testing.expect(try part1(&reader) == 357);
}
test "example2" {
    var reader = std.io.Reader.fixed(example_input);
    try std.testing.expect(try part2(&reader) == 3121910778619);
}
