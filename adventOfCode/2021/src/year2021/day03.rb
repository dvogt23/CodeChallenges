require 'matrix'

module Year2021
  class Day03
    def part1(input)
      eps = []
      gam = []

      input.lines.map { |l| l.strip.chars.map(&:to_i) }.transpose.map do |c|
        eps << if c.count(0) > c.count(1)
                  0
                else
                  1
                end
        gam << if c.count(0) < c.count(1)
                  0
                else
                  1
                end
      end

      eps.join.to_i(2) * gam.join.to_i(2)
    end

    def part2(_input)
      nil
    end
  end
end
