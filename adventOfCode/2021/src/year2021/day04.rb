module Year2021
  class Day04
    def part1(input)
      random_numbers = input.lines.first.split(',').map(&:to_i)

      boards = input
               .lines
               .drop(2)
               .map(&:chomp)
               .filter { |l| !l.empty? }
               .each_slice(5)
               .map { |arr| arr.map { |line| line.split(' ').map(&:to_i) } }
               .map { |board| Bingo.new board }

      random_numbers.each do |num|
        boards.each do |board|
          board.mark num
          return board.calculate_answer if board.won?
        end
      end
    end

    def part2(input)
      random_numbers = input.lines.first.split(',').map(&:to_i)

      boards = input
               .lines
               .drop(2)
               .map(&:chomp)
               .filter { |l| !l.empty? }
               .each_slice(5)
               .map { |arr| arr.map { |line| line.split(' ').map(&:to_i) } }
               .map { |board| Bingo.new board }

      # find last board to win
      last_board = find_last_board random_numbers, boards

      # resolve last board
      random_numbers.each do |num|
        last_board.mark num
        return last_board.calculate_answer if last_board.won?
      end
    end

    private

    def find_last_board(nums, boards)
      nums.each do |num|
        boards.each do |board|
          board.mark num
        end

        boards.delete_if(&:won?)

        return boards.first if boards.size == 1
      end
    end
  end

  class Bingo
    def initialize(board)
      @board = board
      @values = board.flatten
      @marked = []
      @won = false
    end

    def mark(num)
      return if @won

      if @values.include? num
        @marked << num
        @board = @board.map { |col| col.map { |val| val == num ? '#' : val } }
      end
    end

    def won?
      return true if @won

      valid = ->(line) { line.count { |n| n == '#' } == line.size }
      @won = @board.any?(&valid) || @board.transpose.any?(&valid)
    end

    def calculate_answer
      (@values - @marked).sum * @marked.last
    end
  end
end
