#!/usr/bin/env ruby

def is_safe?(tokens)
  inc = false
  dec = false
  tokens.each_cons(2).all? do |tuple|
    delta = tuple[1] - tuple[0]
    inc ||= delta > 0
    dec ||= delta < 0
    large_gap = delta > 3 || delta < -3

    delta != 0 && (inc ^ dec) && !large_gap
  end
end

def solve_part_1(lines)
  lines.count { |line| is_safe?(line.split.map(&:to_i)) }
end

def solve_part_2(lines)
  lines.count do |line|
    tokens = line.split.map(&:to_i)

    next true if is_safe?(tokens)

    (0...tokens.size).any? do |i|
      tokens.permutation
      list = tokens.dup
      list.delete_at(i)
      is_safe?(list)
    end
  end
end


require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample_input
    assert_equal(2, solve_part_1(File.readlines("test_input")))
  end

  def test_part_1_real_input
    assert_equal(549, solve_part_1(File.readlines("input")))
  end

  def test_part_2_sample_input
    assert_equal(4, solve_part_2(File.readlines("test_input")))
  end

  def test_part_2_real_input
    assert_equal(589, solve_part_2(File.readlines("input")))
  end
end
