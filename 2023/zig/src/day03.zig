const std = @import("std");

const util = @import("util.zig");
const gpa = util.gpa;

const data = @embedFile("data/day03.txt");
const stdout = std.io.getStdOut().writer();

pub fn partA() !usize {
    var total: usize = 0;

    const line_len = std.mem.indexOf(u8, data, "\n").?;
    const line_count = std.mem.count(u8, data, "\n");

    for (0..line_count) |y| {
        var x: usize = 0;
        while (x < line_len) : (x += 1) {
            var parser = std.fmt.Parser{ .buf = data[(x + y * (line_len + 1))..] };

            const number = parser.number() orelse continue;
            const number_len = std.math.log10_int(number) + 1;
            defer x += number_len - 1;

            const left = if (x == 0) x else x - 1;
            const right = if (x + number_len == line_len) x + number_len else x + number_len + 1;
            const top = if (y == 0) y else y - 1;
            const bottom = if (y + 1 == line_count) y else y + 1;

            for (top..bottom + 1) |y_surround| {
                for (left..right) |x_surround| {
                    switch (data[(x_surround + y_surround * (line_len + 1))]) {
                        '0'...'9', '.' => {},
                        else => {
                            total += number;
                        },
                    }
                }
            }
        }
    }

    return total;
}

pub fn partB() !usize {
    const Point = struct { x: usize, y: usize };

    const Gears = struct {
        /// The product of all gears
        val: usize = 1,
        /// Amount of gears
        amount: usize = 0,
    };

    var map = std.AutoHashMap(Point, Gears).init(gpa);
    defer map.deinit();

    var total: usize = 0;

    const line_len = std.mem.indexOf(u8, data, "\n").?;
    const line_count = std.mem.count(u8, data, "\n");

    for (0..line_count) |y| {
        var x: usize = 0;
        while (x < line_len) : (x += 1) {
            var parser = std.fmt.Parser{ .buf = data[(x + y * (line_len + 1))..] };

            const number = parser.number() orelse continue;
            const number_len = std.math.log10_int(number) + 1;
            defer x += number_len - 1;

            const left = if (x == 0) x else x - 1;
            const right = if (x + number_len == line_len) x + number_len else x + number_len + 1;
            const top = if (y == 0) y else y - 1;
            const bottom = if (y + 1 == line_count) y else y + 1;

            for (top..bottom + 1) |y_surround| {
                for (left..right) |x_surround| {
                    if (data[(x_surround + y_surround * (line_len + 1))] == '*') {
                        const point = Point{
                            .x = x_surround,
                            .y = y_surround,
                        };

                        if (map.getPtr(point)) |gears| {
                            gears.val *= number;
                            gears.amount += 1;
                        } else {
                            const gears = Gears{
                                .val = number,
                                .amount = 1,
                            };

                            try map.put(point, gears);
                        }
                    }
                }
            }
        }
    }

    var iter = map.valueIterator();
    while (iter.next()) |val| {
        if (val.amount >= 2) {
            total += val.val;
        }
    }

    return total;
}

test "day 03 - part a" {
    const total = try partA();
    try std.testing.expectEqual(@as(usize, 556367), total);
}

test "day 03 - part b" {
    const total = try partB();
    try std.testing.expectEqual(@as(usize, 89471771), total);
}
