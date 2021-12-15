module Year2021
  class Day14
    def part1(input)
      template = input.lines.first.chomp
      insertion = input.scan(/(\w+) -> (\w)/).to_h

      10.times do
        template = template.chars.each_cons(2).map { |a, b| a + insertion[a + b] }.join + template.chars[-1]
      end

      chars = template.chars.tally.sort_by { |_, n| n }
      chars[-1][1] - chars[0][1]
    end

    def part2(input)
      template = input.lines.first.chomp
      insertions = input.scan(/(\w+) -> (\w+)/).to_h

      pairs = template.chars.each_cons(2).map(&:join).tally
      chars = template.chars.tally

      40.times do
        new_pairs = {}
        pairs.each do |pair, n|
          chars[insertions[pair]] = (chars[insertions[pair]] || 0) + n
          c = insertions[pair]
          new_pairs[pair] = (new_pairs[pair] || 0) - n
          new_pairs[pair[0] + c] = (new_pairs[pair[0] + c] || 0) + n
          new_pairs[c + pair[1]] = (new_pairs[c + pair[1]] || 0) + n
        end
        pairs.merge!(new_pairs) { |_, o, n| o + n }
      end

      chars.values.minmax.reduce(:-).abs
    end
  end
end
