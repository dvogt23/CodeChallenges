input = File.read("input/day01.txt").split("\n\n").map { |elph| elph.split("\n").map(&:to_i).sum }

pp input.max
pp input.sort[-3..-1].sum
