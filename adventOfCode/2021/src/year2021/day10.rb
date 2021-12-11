module Year2021
  class Day10
    POINTS = { ')' => 3, ']' => 57, '}' => 1197, '>' => 25_137 }.freeze
    PAIRS = { '(' => ')', '[' => ']', '{' => '}', '<' => '>' }.freeze

    def part1(input)
      input.lines.map(&:chomp).map do |l|
        stack = []
        point = 0
        l.chars.each do |c|
          if PAIRS.keys.include? c
            stack << c
          elsif c == PAIRS[stack.last]
            stack.pop
          else
            point = POINTS[c]
            break
          end
        end
        point
      end
           .sum
    end

    POINTS2 = { ')' => 1, ']' => 2, '}' => 3, '>' => 4 }.freeze

    def part2(input)
      values = input.lines.map(&:chomp).map do |l|
        stack = []
        valid = true
        l.chars.each do |c|
          if PAIRS.keys.include? c
            stack << c
          elsif c == PAIRS[stack.last]
            stack.pop
          else
            valid = false
            break
          end
        end

        [valid, stack.reverse]
      end
                    .filter_map { |valid, stack| stack if valid }
                    .map { |l| l.map { PAIRS[_1] } }
                    .map { |l| l.reduce(0) { |acc, c| (5 * acc) + POINTS2[c] } }
                    .sort

      values[(values.size / 2).round]
    end
  end
end
