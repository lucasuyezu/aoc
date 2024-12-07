#!/usr/bin/env ruby

def pah(target, operands, concat)
  return false if operands[0] > target

  sum = operands[0] + operands[1]
  mul = operands[0] * operands[1]
  cct = "#{operands[0]}#{operands[1]}".to_i if concat

  if operands.size == 2
    return sum == target || mul == target || (concat && cct == target)
  end

  rest = operands[2..]

  pah(target, [ sum ] + rest, concat) ||
  pah(target, [ mul ] + rest, concat) ||
  (concat && pah(target, [ cct ] + rest, concat))
end

def solve_part_1 input
  input
    .lines
    .map { it.gsub(":", " ").split.map(&:to_i) }
    .filter_map { |target, *operands| target if pah(target, operands, false) }
    .sum
end

def solve_part_2 input
  input
    .lines
    .map { it.gsub(":", " ").split.map(&:to_i) }
    .filter_map { |target, *operands| target if pah(target, operands, true) }
    .sum
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample_input
    assert_equal(3749, solve_part_1(File.read("test_input")))
  end

  def test_part_1_real_input
    assert_equal(4555081946288, solve_part_1(File.read("input")))
  end

  def test_part_2_sample_input
    assert_equal(11387, solve_part_2(File.read("test_input")))
  end

  def test_part_2_real_input
    assert_equal(227921760109726, solve_part_2(File.read("input")))
  end
end
