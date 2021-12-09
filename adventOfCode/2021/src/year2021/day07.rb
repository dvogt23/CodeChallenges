module Year2021
  class Day07
    def part1(input)
      crabs = input.split(',').map(&:to_i)
      med = crabs.sort[crabs.length / 2]
      crabs.reduce(0) { |acc, c| acc + (c - med).abs }
    end

    def part2(input)
      positions = input.scan(/\d+/).map(&:to_i)

      (positions.min..positions.max).map do |opt|
        positions.map do |p|
          n = (p - opt).abs
          n * (n + 1) / 2
        end.sum
      end.min
    end
  end
end
