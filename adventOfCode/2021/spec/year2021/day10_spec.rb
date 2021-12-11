require 'spec_helper'

RSpec.describe Year2021::Day10 do
  input ='[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]'
  it "solves part1" do
    d = Year2021::Day10.new
    expect(d.part1(input)).to eq(26397)
  end

  it "solves part2" do
    d = Year2021::Day10.new
    expect(d.part2('some_input')).to eq(nil)
  end
end
