const std = @import("std");
const lib = @import("lib.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const input = @embedFile("../input.txt");
    const result = try lib.part1(allocator, input);
    defer allocator.free(result);

    const stdout = std.io.getStdOut().writer();
    try stdout.print("Part 1: {s}\n", .{result});
}
