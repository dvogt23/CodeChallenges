module Year2021
  class Day05
    def part1(input)
      lines = parse_input(input).filter(&:orthogonal?)

      rows = lines.flat_map(&:max_x).max + 1
      cols = lines.flat_map(&:max_y).max + 1

      # grid = Array.new(max_x + 1) { Array.new(max_y + 1, 0) }
      grid = Array.new(rows * cols, 0)

      lines.each do |line|
        line.covered_orthogonal_points.each do |x, y|
          # grid[x][y] += 1
          grid[x * cols + y] += 1
        end
      end

      grid.count { |n| n >= 2 }
    end

    def part2(input)
      lines = parse_input(input)

      rows = lines.flat_map(&:max_x).max + 1
      cols = lines.flat_map(&:max_y).max + 1

      grid = Array.new(rows * cols, 0)

      lines.each do |line|
        line.covered_points.each do |x, y|
          grid[x * cols + y] += 1
        end
      end

      grid.count { |n| n >= 2 }
    end

    def parse_input(input)
      input.lines
           .flat_map { |l| l.split('->').map(&:chomp) }
           .map { |pair| pair.split(',').map(&:to_i) }
           .each_slice(2)
           .map { |points| Line.new(*points) }
    end

    class Line
      def initialize(point1, point2)
        @p1 = point1
        @p2 = point2
      end

      def orthogonal?
        vertical? || horizontal?
      end

      def vertical?
        @p1[0] == @p2[0]
      end

      def horizontal?
        @p1[1] == @p2[1]
      end

      def max_x
        [@p1[0], @p2[0]].max
      end

      def max_y
        [@p1[1], @p2[1]].max
      end

      def covered_orthogonal_points
        if horizontal?
          range(@p1[0], @p2[0]).map { |x| [x, @p1[1]] }
        else
          range(@p1[1], @p2[1]).map { |y| [@p1[0], y] }
        end
      end

      def covered_points
        return covered_orthogonal_points if orthogonal?

        range(@p1[0], @p2[0])
          .zip(range(@p1[1], @p2[1]))
      end

      def range(num1, num2)
        if num1 > num2
          num1.downto(num2).to_a
        else
          (num1..num2).to_a
        end
      end
    end
  end
end
