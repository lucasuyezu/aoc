#!/usr/bin/env ruby

def pah(operands, concat)
  result = []

  if operands.size == 2
    result << operands[0] + operands[1]
    result << operands[0] * operands[1]
    result << "#{operands[0]}#{operands[1]}".to_i if concat

    return result
  end

  rest = operands[2,operands.size]

  result += pah([ operands[0] + operands[1] ] + rest, concat)
  result += pah([ operands[0] * operands[1] ] + rest, concat)
  result += pah(["#{operands[0]}#{operands[1]}".to_i] + rest, concat) if concat

  result
end

def solve_part_1 input
  input
    .lines
    .filter_map do |line|
      line = line.split(":")
      result = line.first.to_i
      operands = line.last.split.map(&:to_i)

      result if pah(operands, false).any? { it == result }
    end
    .sum
end

def solve_part_2 input
  input
    .lines
    .filter_map do |line|
      line = line.split(":")
      result = line.first.to_i
      operands = line.last.split.map(&:to_i)

      result if pah(operands, true).any? { it == result }
    end
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
