module Year2021
  class Day08
    def part1(input)
      input.lines.map do |l|
        l.split('|').last.split(' ').map(&:length).filter do |w|
          [2, 3, 4, 7].include?(w)
        end.count
      end.sum
    end

    def part2(input)
      input = input
              .lines
              .map(&:chomp)
              .map { |l| l.scan(/\w+/).map(&:chars).map(&:sort) }

      input.reduce(0) do |acc, line|
        s = {}
        n = {}

        n[1] = line.find { _1.size == 2 }
        n[4] = line.find { _1.size == 4 }
        n[7] = line.find { _1.size == 3 }
        n[8] = line.find { _1.size == 7 }

        s[:a] = n[7] - n[1]

        # 6 digits: 0, 6, 9
        six_digits = line.filter { _1.size == 6 }.uniq
        n[9] = six_digits.find { (_1 - n[4] - s[:a]).size == 1 }

        n[0] = six_digits.reject { _1 == n[9] }.find { |seg| n[1].all? { |o| seg.include? o } }
        n[6] = six_digits.find { _1 != n[9] && _1 != n[0] }

        s[:d] = (n[8] - n[0]).flatten
        s[:c] = n[8] - n[6]
        s[:f] = n[1] - s[:c]

        s[:b] = n[4] - s[:c] - s[:d] - s[:f]

        n[2] = n[8] - s[:b] - s[:f]
        n[3] = n[9] - s[:b]
        n[5] = n[9] - s[:c]

        acc + line.drop(10).map { |ary| n.key(ary) }.join.to_i
      end
    end
  end
end
