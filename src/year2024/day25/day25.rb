#!/usr/bin/env ruby

def solve_part_1(input)
  locks, keys = input.split("\n\n").partition { it.start_with?("#####") }

  ans = 0

  locks.each do |lock|
    keys.each do |key|
      ans += 1 unless key.chars.each_with_index.any? { |c, i| c == "#" && lock[i] == "#" }
    end
  end

  ans
end

def solve_part_2(input)
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample
    assert_equal(3, solve_part_1(File.read("test_input")))
  end

  def test_part_1_real
    assert_equal(2885, solve_part_1(File.read("input")))
  end
end
