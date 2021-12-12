require 'spec_helper'

RSpec.describe Year2021::Day12 do
  input = 'start-A
start-b
A-c
A-b
b-d
A-end
b-end'
  it 'solves part1' do
    d = Year2021::Day12.new
    expect(d.part1(input)).to eq(10)
  end

  it 'solves part2' do
    d = Year2021::Day12.new
    expect(d.part2(input)).to eq(36)
  end
end
