#!/usr/bin/env ruby

def solve_part_1 min, max
  min.upto(max).map(&:to_s).count do
    next false unless it.chars.each_cons(2).any? { it[0] == it[1] }
    next false unless it.chars.each_cons(2).all? { it[0].to_i <= it[1].to_i }

    true
  end
end

def solve_part_2(min, max)
  min.upto(max).map(&:to_s).count do
    next false unless it.chars.each_cons(2).all? { it[0].to_i <= it[1].to_i }
    next false unless it.chars.tally.values.any? { it == 2 }

    true
  end
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample
    assert_equal(1, solve_part_1(111111, 111111))
    assert_equal(0, solve_part_1(223450, 223450))
    assert_equal(0, solve_part_1(123789, 123789))
  end

  def test_part_1_real
    assert_equal(1063, solve_part_1(246540, 787419))
  end

  def test_part_2_sample
    assert_equal(0, solve_part_2(123456, 123456))
    assert_equal(1, solve_part_2(112233, 112233))
    assert_equal(0, solve_part_2(123444, 123444))
    assert_equal(1, solve_part_2(111122, 111122))
  end

  def test_part_2_real
    assert_equal(686, solve_part_2(246540, 787419))
  end
end
