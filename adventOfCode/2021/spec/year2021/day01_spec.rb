require 'spec_helper'

day = Year2021::Day01.new
input3 = <<~DIGITS
  2
  3
  2
  4
  5
DIGITS

input2 = <<~DIGITS
  2
  3
  2
  2
  5
DIGITS

input6 = <<~DIGITS
  2
  3
  2
  4
  5
  8
  11
  13
DIGITS

example2 = <<~DIGITS
  199
  200
  208
  210
  200
  207
  240
  269
  260
  263
DIGITS

RSpec.describe Year2021::Day01 do
  it 'count the increases' do
    expect(day.part1(input3)).to eq(3)
    expect(day.part1(input2)).to eq(2)
    expect(day.part1(input6)).to eq(6)
  end

  it 'count the increases of 3 windows' do
    expect(day.part2(example2)).to eq(5)
  end
end
