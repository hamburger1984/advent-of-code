const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Add all day modules
    const days = [_]u8{ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25 };

    for (days) |day| {
        const day_str = b.fmt("day-{d}", .{day});
        const day_path = b.fmt("{s}/src/lib.zig", .{day_str});

        // Check if day directory exists
        var day_dir = std.fs.cwd().openDir(day_str, .{}) catch continue;
        day_dir.close();

        // Part 1 executable
        const part1_exe = b.addExecutable(.{
            .name = b.fmt("{s}-part1", .{day_str}),
            .root_module = b.createModule(.{
                .root_source_file = b.path(b.fmt("{s}/src/part1.zig", .{day_str})),
                .target = target,
                .optimize = optimize,
            }),
        });
        b.installArtifact(part1_exe);

        const run_part1 = b.addRunArtifact(part1_exe);
        const run_part1_step = b.step(b.fmt("run-{s}-part1", .{day_str}), b.fmt("Run {s} part 1", .{day_str}));
        run_part1_step.dependOn(&run_part1.step);

        // Part 2 executable
        const part2_exe = b.addExecutable(.{
            .name = b.fmt("{s}-part2", .{day_str}),
            .root_module = b.createModule(.{
                .root_source_file = b.path(b.fmt("{s}/src/part2.zig", .{day_str})),
                .target = target,
                .optimize = optimize,
            }),
        });
        b.installArtifact(part2_exe);

        const run_part2 = b.addRunArtifact(part2_exe);
        const run_part2_step = b.step(b.fmt("run-{s}-part2", .{day_str}), b.fmt("Run {s} part 2", .{day_str}));
        run_part2_step.dependOn(&run_part2.step);

        // Tests
        const lib_tests = b.addTest(.{
            .root_module = b.createModule(.{
                .root_source_file = b.path(day_path),
                .target = target,
                .optimize = optimize,
            }),
        });

        const test_step = b.step(b.fmt("test-{s}", .{day_str}), b.fmt("Run {s} tests", .{day_str}));
        test_step.dependOn(&b.addRunArtifact(lib_tests).step);
    }

    // Run all tests
    const all_tests_step = b.step("test", "Run all tests");
    for (days) |day| {
        const day_str = b.fmt("day-{d}", .{day});
        const day_path = b.fmt("{s}/src/lib.zig", .{day_str});

        var day_dir_2 = std.fs.cwd().openDir(day_str, .{}) catch continue;
        day_dir_2.close();

        const lib_tests = b.addTest(.{
            .root_module = b.createModule(.{
                .root_source_file = b.path(day_path),
                .target = target,
                .optimize = optimize,
            }),
        });
        all_tests_step.dependOn(&b.addRunArtifact(lib_tests).step);
    }
}
