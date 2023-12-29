const std = @import("std");

const util = @import("util.zig");
const gpa = util.gpa;

const data = @embedFile("data/day05.txt");
const stdout = std.io.getStdOut().writer();

pub fn partA() !isize {
    var lowestA: isize = std.math.maxInt(isize);
    var lines = std.mem.tokenize(u8, data, "\n");

    // Get seeds
    var seeds_iter = std.mem.tokenize(u8, lines.next().?, " ");
    _ = seeds_iter.next(); // Ignore "seeds: "
    while (seeds_iter.next()) |seed| {
        var new_seed = try std.fmt.parseInt(isize, seed, 10);

        // Parse map
        while (lines.next()) |line| {
            if (!std.ascii.isDigit(line[0])) continue; // Skip headers

            var line_iter = std.mem.tokenize(u8, line, " ");
            const dest = try std.fmt.parseInt(isize, line_iter.next().?, 10);
            const src = try std.fmt.parseInt(isize, line_iter.next().?, 10);
            const range = try std.fmt.parseInt(isize, line_iter.next().?, 10);

            if (new_seed >= src and new_seed <= (src + (range - 1))) {
                new_seed += (dest - src);

                // Skip to next section
                while (true) {
                    var next_line = lines.next() orelse break;

                    // Find next line that starts with a character
                    if (!std.ascii.isDigit(next_line[0])) {
                        break;
                    }
                }
            }
        }

        if (new_seed < lowestA) lowestA = new_seed;

        lines.reset();
    }

    return lowestA;
}

pub fn findLowest(seed_range: struct { usize, usize }) !void {
    var lines = std.mem.tokenize(u8, data, "\n");

    for (seed_range[0]..seed_range[1]) |seed| {
        var new_seed = @as(isize, @intCast(seed));

        // Parse map
        while (lines.next()) |line| {
            if (!std.ascii.isDigit(line[0])) continue; // Skip headers

            var line_iter = std.mem.tokenize(u8, line, " ");
            const dest = try std.fmt.parseInt(isize, line_iter.next().?, 10);
            const src = try std.fmt.parseInt(isize, line_iter.next().?, 10);
            const range = try std.fmt.parseInt(isize, line_iter.next().?, 10);

            if (new_seed >= src and new_seed <= (src + (range - 1))) {
                new_seed += (dest - src);

                // Skip to next section
                while (true) {
                    var next_line = lines.next() orelse break;

                    // Find next line that starts with a character
                    if (!std.ascii.isDigit(next_line[0])) {
                        break;
                    }
                }
            }
        }

        mutex.lock();
        if (new_seed < lowest) {
            lowest = new_seed;
            try stdout.print("== Updating lowest {d} ==\n", .{lowest});
        }
        mutex.unlock();

        lines.reset();
    }

    try stdout.print("== Thread Finishing ==\n", .{});
}

var lowest: isize = std.math.maxInt(isize);
var mutex = std.Thread.Mutex{};

pub fn partB() !isize {
    var lines = std.mem.tokenize(u8, data, "\n");

    // Get seeds
    var seed_ranges = std.ArrayList(struct { usize, usize }).init(gpa);
    defer seed_ranges.deinit();

    var seeds_iter = std.mem.tokenize(u8, lines.next().?, " ");
    _ = seeds_iter.next(); // Ignore "seeds: "
    while (seeds_iter.next()) |seed_data| {
        const parsed_seed = try std.fmt.parseInt(u32, seed_data, 10);
        const seed_range = try std.fmt.parseInt(u32, seeds_iter.next().?, 10);

        try seed_ranges.append(.{ parsed_seed, parsed_seed + seed_range });
    }

    var threads = std.ArrayList(std.Thread).init(gpa);
    defer threads.deinit();

    for (seed_ranges.items) |range| {
        try stdout.print("== Range {d}-{d} ==\n", .{ range[0], range[1] });
        try threads.append(try std.Thread.spawn(.{}, findLowest, .{range}));
    }

    for (threads.items) |thread| {
        thread.join();
    }

    return lowest;
}

test "day 05 - part a" {
    const total = try partA();
    try std.testing.expectEqual(@as(isize, 621354867), total);
}

test "day 05 - part b" {
    const total = try partB();
    try std.testing.expectEqual(@as(isize, 0), total);
}
