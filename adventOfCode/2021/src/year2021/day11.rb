module Year2021
  class Day11
    def part1(input)
      input = input.lines.map(&:chomp).map{ _1.scan(/\d/).map(&:to_i) }

      flashes = 0
      100.times do
        input.map! { |l| l.map! { _1 + 1 } }

        flashing_points = []
        (0..9).each do |x|
          (0..9).each do |y|
            flashing_points << [x, y] if input[x][y] == 10
          end
        end

        flashed = []

        until flashing_points.empty?
          x, y = flashing_points.pop

          input[x][y] = 0
          flashed << [x, y]

          neighbours(x, y).each do |xx, yy|
            input[xx][yy] += 1 unless flashed.include? [xx, yy]
            flashing_points << [xx, yy] if input[xx][yy] > 9
          end

          flashing_points.uniq!
        end

        flashes += flashed.size
      end

      flashes
    end

    def part2(input)
      input = input.lines.map(&:chomp).map{ _1.scan(/\d/).map(&:to_i) }

      (1..).each do |i|
        input.map! { |l| l.map! { _1 + 1 } }

        flashing_points = []
        (0..9).each do |x|
          (0..9).each do |y|
            flashing_points << [x, y] if input[x][y] == 10
          end
        end

        flashed = []

        until flashing_points.empty?
          x, y = flashing_points.pop

          input[x][y] = 0
          flashed << [x, y]

          neighbours(x, y).each do |xx, yy|
            input[xx][yy] += 1 unless flashed.include? [xx, yy]
            flashing_points << [xx, yy] if input[xx][yy] > 9
          end

          flashing_points.uniq!
        end

        return i if input.flatten.all?(0)
      end
    end

    private

    def neighbours(i, j)
      n = []

      (-1..1).each do |xx|
        next if (i + xx).negative? || i + xx > 9

        (-1..1).each do |yy|
          next if (j + yy).negative? || j + yy > 9 || (xx.zero? && yy.zero?)

          n << [i + xx, j + yy]
        end
      end

      n
    end
  end
end
