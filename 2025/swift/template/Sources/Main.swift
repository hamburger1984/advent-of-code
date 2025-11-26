import Foundation

@main
struct Solution {
    static func main() {
        let input = try! String(contentsOfFile: "input.txt").trimmingCharacters(in: .whitespacesAndNewlines)

        print("Part 1: \\(part1(input))")
        print("Part 2: \\(part2(input))")
    }

    static func part1(_ input: String) -> String {
        let lines = input.split(separator: "\\n").map(String.init)

        // TODO: Implement part 1
        // Example:
        // let numbers = lines.compactMap { Int($0) }
        // let sum = numbers.reduce(0, +)

        return "not implemented"
    }

    static func part2(_ input: String) -> String {
        let lines = input.split(separator: "\\n").map(String.init)

        // TODO: Implement part 2

        return "not implemented"
    }
}
