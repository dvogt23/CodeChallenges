module Year2021
  class Day12
    def part1(input)
      connections = {}
      input
        .lines
        .map { _1.scan(/\w+/) }
        .each do |s, e|
        connections[s] ||= []
        connections[s] << e unless e == 'start'
        connections[e] ||= []
        connections[e] << s unless s == 'start' || e == 'end'
      end

      combinations = []
      incomplete = [['start']]

      until incomplete.empty?
        path = incomplete.pop
        connections[path.last].each do |n|
          next if n.downcase == n && path.include?(n)

          if n == 'end'
            combinations << (path + [n])
          else
            incomplete << (path + [n])
          end
        end
      end

      combinations.size
    end

    def part2(input)
      connections = {}
      input
        .lines
        .map { _1.scan(/\w+/) }
        .each do |s, e|
        connections[s] ||= []
        connections[s] << e unless e == 'start'
        connections[e] ||= []
        connections[e] << s unless s == 'start' || e == 'end'
      end

      combinations = []
      incomplete = [['start']]

      until incomplete.empty?
        path = incomplete.pop
        connections[path.last].each do |n|
          next if n.downcase == n && path.include?(n) && path.tally.any? { |k, v| k.downcase == k && v >= 2 }

          if n == 'end'
            combinations << (path + [n])
          else
            incomplete << (path + [n])
          end
        end
      end

      combinations.size
    end
  end
end
