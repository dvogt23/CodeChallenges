require 'spec_helper'

RSpec.describe Year2021::Day07 do
  input = '16,1,2,0,4,2,7,1,2,14'
  it 'solves part1' do
    d = Year2021::Day07.new
    expect(d.part1(input)).to eq(37)
  end

  it 'solves part2' do
    d = Year2021::Day07.new
    expect(d.part2(input)).to eq(168)
  end
end
