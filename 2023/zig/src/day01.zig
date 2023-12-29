const std = @import("std");

const util = @import("util.zig");
const gpa = util.gpa;

const data = @embedFile("data/day01.txt");
const stdout = std.io.getStdOut().writer();

pub fn partA() !usize {
    var total: usize = 0;
    var num: u32 = 0;

    var lines = std.mem.tokenize(u8, data, "\n");
    while (lines.next()) |line| {
        var occurance: u32 = 0;

        for (line) |c| {
            if (std.ascii.isDigit(c)) {
                num = try std.fmt.charToDigit(c, 10);

                if (occurance == 0) {
                    total += num * 10;
                    occurance += 1;
                }
            }
        }

        if (occurance == 1) {
            total += num;
        }
    }

    return total;
}

pub fn partB() !usize {
    var total: usize = 0;
    var num: u32 = 0;

    const numbers = [_][:0]const u8{ "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" };

    var lines = std.mem.tokenize(u8, data, "\n");
    while (lines.next()) |line| {
        var occurrence: u32 = 0;

        for (line, 0..) |c, i| {
            if (std.ascii.isDigit(c)) {
                num = try std.fmt.charToDigit(c, 10);
                if (occurrence == 0) {
                    total += num * 10;
                    occurrence += 1;
                }
            } else {
                for (numbers, 0..) |n, j| {
                    if (std.mem.startsWith(u8, line[i..], n)) {
                        num = @intCast(j);
                        if (occurrence == 0) {
                            total += num * 10;
                            occurrence += 1;
                        }
                    }
                }
            }
        }

        if (occurrence == 1) {
            total += num;
        }
    }

    return total;
}

test "day 01 - part a" {
    const total = try partA();
    try std.testing.expectEqual(@as(usize, 55090), total);
}

test "day 01 - part b" {
    const total = try partB();
    try std.testing.expectEqual(@as(usize, 54845), total);
}
