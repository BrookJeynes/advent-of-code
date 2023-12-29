const std = @import("std");

const util = @import("util.zig");
const gpa = util.gpa;

const data = @embedFile("data/day02.txt");
const stdout = std.io.getStdOut().writer();

const Marbles = struct {
    red: u32,
    blue: u32,
    green: u32,

    pub fn default() Marbles {
        return Marbles{
            .red = 0,
            .blue = 0,
            .green = 0,
        };
    }
};

pub fn partA() !usize {
    var total: usize = 0;

    var lines = std.mem.tokenize(u8, data, "\n");
    while (lines.next()) |line| {
        var line_split = std.mem.tokenize(u8, line, ":");
        var id = line_split.next().?[5..];
        var marbles = Marbles.default();
        var possible = true;

        var games = std.mem.tokenize(u8, line_split.next().?, ";");
        while (games.next()) |game| {
            var marble = std.mem.tokenize(u8, game, ",");
            while (marble.next()) |m| {
                var next_marble = std.mem.tokenize(u8, m, " ");
                var next_marble_count = try std.fmt.parseInt(u32, next_marble.next().?, 10);
                var next_marble_colour = next_marble.next().?;

                if (std.mem.eql(u8, next_marble_colour, "red")) {
                    marbles.red += next_marble_count;
                } else if (std.mem.eql(u8, next_marble_colour, "green")) {
                    marbles.green += next_marble_count;
                } else if (std.mem.eql(u8, next_marble_colour, "blue")) {
                    marbles.blue += next_marble_count;
                }
            }

            if (marbles.red > 12 or marbles.green > 13 or marbles.blue > 14) {
                possible = false;
            }

            marbles = Marbles.default();
        }

        if (possible) {
            total += try std.fmt.parseInt(u32, id, 10);
        }
    }

    return total;
}

pub fn partB() !usize {
    var total: usize = 0;

    var lines = std.mem.tokenize(u8, data, "\n");
    while (lines.next()) |line| {
        var line_split = std.mem.tokenize(u8, line, ":");
        _ = line_split.next(); // Skip over "Game" text
        var marbles = Marbles.default();

        var games = std.mem.tokenize(u8, line_split.next().?, ";");
        while (games.next()) |game| {
            var marble = std.mem.tokenize(u8, game, ",");
            while (marble.next()) |m| {
                var next_marble = std.mem.tokenize(u8, m, " ");
                var next_marble_count = try std.fmt.parseInt(u32, next_marble.next().?, 10);
                var next_marble_colour = next_marble.next().?;

                if (std.mem.eql(u8, next_marble_colour, "red")) {
                    if (marbles.red < next_marble_count) {
                        marbles.red = next_marble_count;
                    }
                } else if (std.mem.eql(u8, next_marble_colour, "green")) {
                    if (marbles.green < next_marble_count) {
                        marbles.green = next_marble_count;
                    }
                } else if (std.mem.eql(u8, next_marble_colour, "blue")) {
                    if (marbles.blue < next_marble_count) {
                        marbles.blue = next_marble_count;
                    }
                }
            }
        }

        total += marbles.red * marbles.green * marbles.blue;
    }

    return total;
}

test "day 02 - part a" {
    const total = try partA();
    try std.testing.expectEqual(@as(usize, 2771), total);
}

test "day 02 - part b" {
    const total = try partB();
    try std.testing.expectEqual(@as(usize, 70924), total);
}
