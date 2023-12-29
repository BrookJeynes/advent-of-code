const std = @import("std");

const util = @import("util.zig");
const gpa = util.gpa;

const data = @embedFile("data/day04.txt");
const stdout = std.io.getStdOut().writer();

pub fn partA() !usize {
    var total: usize = 0;

    var lines = std.mem.tokenize(u8, data, "\n");
    while (lines.next()) |line| {
        var match_total: usize = 1;
        var matched = false;

        var card_nums = std.mem.tokenize(u8, line, ":");
        _ = card_nums.next(); // Skip "Game x: "

        var winning_nums = std.ArrayList(u32).init(gpa);
        defer winning_nums.deinit();

        var numbers = std.mem.tokenize(u8, card_nums.next().?, "|");
        var winning_nums_parser = std.mem.tokenize(u8, numbers.next().?, " ");
        var scrap_nums_parser = std.mem.tokenize(u8, numbers.next().?, " ");

        while (winning_nums_parser.next()) |num| {
            try winning_nums.append(try std.fmt.parseInt(u32, num, 10));
        }

        while (scrap_nums_parser.next()) |num| {
            for (winning_nums.items) |winning_num| {
                if (try std.fmt.parseInt(u32, num, 10) == winning_num) {
                    match_total += match_total;
                    matched = true;
                }
            }
        }

        if (matched) {
            total += match_total - (match_total / 2);
        }
    }

    return total;
}

pub fn partB() !usize {
    return 0;
}

test "day 04 - part a" {
    const total = try partA();
    try std.testing.expectEqual(@as(usize, 21568), total);
}

test "day 04 - part b" {
    const total = try partB();
    try std.testing.expectEqual(@as(usize, 70924), total);
}
