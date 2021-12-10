module Year2021
  class Day02
    def part1(input)
      position = {
        horizontal: 0,
        depth: 0
      }

      input.lines.map { |l| l.split(' ') }.each do |a, b|
        case a
        when 'forward'
          position[:horizontal] += b.to_i
        when 'down'
          position[:depth] += b.to_i
        when 'up'
          position[:depth] -= b.to_i
        end
      end

      position[:horizontal] * position[:depth]
    end

    def part2(input)
      position = {
        horizontal: 0,
        depth: 0,
        aim: 0
      }

      input.lines.map { |l| l.split(' ') }.each do |a, b|
        case a
        when 'forward'
          position[:horizontal] += b.to_i
          position[:depth] += position[:aim] * b.to_i
        when 'down'
          position[:aim] += b.to_i
        when 'up'
          position[:aim] -= b.to_i
        end
      end

      position[:horizontal] * position[:depth]
    end
  end
end
