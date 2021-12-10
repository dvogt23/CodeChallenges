module Year2021
  class Day09
    def part1(input)
      lowpoints = []
      map = input.lines.map { |l| l.chomp.chars.map(&:to_i) }
      map.map.with_index do |line, i|
        line.each.with_index do |n, j|
          left = line[j - 1] if j.positive?
          right = line[j + 1] if j < line.size
          top = map[i - 1][j] if i.positive?
          bottom = map[i + 1][j] if i < map.size - 1
          lowpoints << [i, j] if n < [left, right, top, bottom].compact.min
        end
      end
      lowpoints.map { |p| map[p.first][p.last] + 1 }.sum
    end

    def part2(input)
      input = input.lines.map(&:strip).map { |l| l.chars.map(&:to_i) }

      row_limit = input.size - 1
      col_limit = input[0].size - 1

      coord = []

      (0..row_limit).each do |i|
        (0..col_limit).each do |j|
          p = input[i][j]
          next unless [[0, i - 1].max, [i + 1, row_limit].min].all? do |x|
            p < input[x][j] || x == i
          end && [[0, j - 1].max, [j + 1, col_limit].min].all? do |y|
            p < input[i][y] || y == j
          end

          coord << [i, j]
        end
      end

      coord.map { basins _1, input }.sort.reverse.first(3).reduce(&:*)
    end

    private

    def basins(coord, input)
      basin = Set.new([coord])
      loop do
        n = []
        basin.each { n << possible_neighbours(*_1, input) }
        n.flatten!(1)

        break if n.all? { basin.include? _1 }

        n.each { basin << _1 }
      end

      basin.size
    end

    def possible_neighbours(i, j, input)
      n = []
      n << [i, j - 1] if j.positive? && input[i][j - 1] < 9
      n << [i, j + 1] if j < input[i].size - 1 && input[i][j + 1] < 9
      n << [i - 1, j] if i.positive? && input[i - 1][j] < 9
      n << [i + 1, j] if i < input.size - 1 && input[i + 1][j] < 9

      n
    end
  end
end
