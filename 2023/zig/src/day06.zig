const std = @import("std");

const util = @import("util.zig");
const gpa = util.gpa;

const data = @embedFile("data/day06.txt");
const stdout = std.io.getStdOut().writer();

pub fn partA() !usize {
    return 0;
}

pub fn partB() !usize {
    return 0;
}

test "day 06 - part a" {
    const total = try partA();
    try std.testing.expectEqual(@as(usize, 0), total);
}

test "day 06 - part b" {
    const total = try partB();
    try std.testing.expectEqual(@as(usize, 0), total);
}
