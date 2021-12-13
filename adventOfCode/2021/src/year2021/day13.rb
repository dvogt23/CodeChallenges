module Year2021
  class Day13
    def part1(input)
      dots = input.scan(/(\d+),(\d+)/).map { |a| a.map(&:to_i) }
      folds = input.scan(/(x|y)=(\d+)/).map { |a| [a[0], a[1].to_i] }

      fold(folds[0][0], folds[0][1], dots).length
    end

    def part2(input)
      dots = input.scan(/(\d+),(\d+)/).map { |a| a.map(&:to_i) }
      folds = input.scan(/(x|y)=(\d+)/).map { |a| [a[0], a[1].to_i] }
      folds.each do |dir, i|
        dots = fold(dir, i, dots)
      end

      xmax, ymax = dots.transpose.map(&:max)
      (0..ymax).each do |y|
        puts (0..xmax).map { |x|
          dots.include?([x, y]) ? '█' : ' '
        }.join
      end
    end

    private

    def fold(dir, i, dots)
      dots.map do |x, y|
        if dir == 'y'
          y = y > i ? 2 * i - y : y
        else
          x = x > i ? 2 * i - x : x
        end
        [x, y]
      end.uniq
    end
  end
end
