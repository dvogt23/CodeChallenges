module Year2021
  class Day01
    def part1(input)
      input.lines.map(&:to_i).each_cons(2).reduce(0) { |a, r| r[0] < r[1] ? a + 1 : a }
    end

    def part2(_input)
      nil
    end
  end
end
