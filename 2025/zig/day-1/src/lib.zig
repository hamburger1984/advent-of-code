const std = @import("std");
const testing = std.testing;

pub fn part1(allocator: std.mem.Allocator, input: []const u8) ![]const u8 {
    _ = allocator;
    _ = input;
    return error.NotImplemented;
}

pub fn part2(allocator: std.mem.Allocator, input: []const u8) ![]const u8 {
    _ = allocator;
    _ = input;
    return error.NotImplemented;
}

test "part1 example" {
    const input =
        \\TODO: Add example input
    ;
    const result = try part1(testing.allocator, input);
    defer testing.allocator.free(result);
    try testing.expectEqualStrings("TODO", result);
}

test "part2 example" {
    const input =
        \\TODO: Add example input
    ;
    const result = try part2(testing.allocator, input);
    defer testing.allocator.free(result);
    try testing.expectEqualStrings("TODO", result);
}
