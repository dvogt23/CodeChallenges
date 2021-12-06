require 'matrix'

module Year2021
  class Day03
    def part1(input)
      gamma = []

      input.lines
           .map { |l| l.strip.chars.map(&:to_i) }
           .transpose
           .map { |c| gamma << c.max_by { |i| c.count(i) } }

      epsilon = gamma.map { |i| i ^ 1 }

      gamma.join.to_i(2) * epsilon.join.to_i(2)
    end

    def part2(input)
      rows = input.lines.map { |l| l.strip.chars.map(&:to_i) }

      oxigen = find_oxygen_generator rows, 0
      co2 = find_co2_scrubber_rating rows, 0

      oxigen * co2
    end

    private

    def find_oxygen_generator(report, col)
      return report.first.join.to_i(2) if report.size == 1

      bit_line = report.transpose[col]

      bit_count = Hash.new(0)
      bit_line.each { |b| bit_count[b] += 1 }

      most_common = if bit_count[0] == bit_count[1]
                      1
                    else
                      bit_count.max_by { |_bit, number| number }.first
                    end

      new_report = report.filter { |b| b[col] == most_common }

      find_oxygen_generator new_report, col + 1
    end

    def find_co2_scrubber_rating(report, col)
      return report.first.join.to_i(2) if report.size == 1

      bit_line = report.transpose[col]

      bit_count = Hash.new(0)
      bit_line.each { |b| bit_count[b] += 1 }

      most_common = if bit_count[0] == bit_count[1]
                      0
                    else
                      bit_count.min_by { |_bit, number| number }.first
                    end

      new_report = report.filter { |b| b[col] == most_common }

      find_co2_scrubber_rating new_report, col + 1
    end
  end
end
