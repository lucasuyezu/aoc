#!/usr/bin/env ruby

def solve_part_1(input)
  input.lines.sum { (it.to_i / 3) - 2 }
end

def solve_part_2(input)
  input.lines.map(&:to_i).sum do |line|
    ans = 0
    while line > 0
      line = (line / 3) - 2
      ans += line if line > 0
    end
    ans
  end
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample
    assert_equal(34241, solve_part_1("12\n14\n1969\n100756"))
  end

  def test_part_1_real
    assert_equal(3395944, solve_part_1(File.read("input")))
  end

  def test_part_2_sample
    assert_equal(51314, solve_part_2("14\n1969\n100756"))
  end

  def test_part_2_real
    assert_equal(5091036, solve_part_2(File.read("input")))
  end
end

