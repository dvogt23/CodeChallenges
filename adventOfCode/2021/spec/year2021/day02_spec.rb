require 'spec_helper'

day = Year2021::Day02.new

example1 = <<~DIGITS
  forward 5
  down 5
  forward 8
  up 3
  down 8
  forward 2
DIGITS

example2 = <<~DIGITS
  forward 5
  down 5
  forward 8
  up 3
  down 8
  forward 2
DIGITS

RSpec.describe Year2021::Day02 do
  it 'count the increases' do
    expect(day.part1(example1)).to eq(150)
  end

  it 'count the increases of 3 windows' do
    expect(day.part2(example2)).to eq(900)
  end
end
