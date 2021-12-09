module Year2021
  class Day09
    def part1(input)
      lowpoints = []
      map = input.lines.map { |l| l.chomp.chars.map(&:to_i) }
      map.map.with_index do |line, i|
        line.each.with_index do |n, j|
          left = line[j - 1] if j > 0
          right = line[j + 1] if j < line.size
          top = map[i - 1][j] if i > 0
          bottom = map[i + 1][j] if i < map.size - 1
          lowpoints << [i, j] if n < [left, right, top, bottom].compact.min
        end
      end
      lowpoints.map { |p| map[p.first][p.last] + 1 }.sum
    end

    def part2(_input)
      nil
    end
  end
end
