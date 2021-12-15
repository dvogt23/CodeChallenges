require 'spec_helper'

RSpec.describe Year2021::Day14 do
  input = 'NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C'
  it 'solves part1' do
    d = Year2021::Day14.new
    expect(d.part1(input)).to eq(1588)
  end

  it 'solves part2' do
    d = Year2021::Day14.new
    expect(d.part2(input)).to eq(2188189693529)
  end
end
