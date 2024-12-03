#!/usr/bin/env ruby

def solve_part_1 lines
  lines
    .join
    .scan(/mul\((?<n1>\d{1,3}),(?<n2>\d{1,3})\)/)
    .map { |tuple| tuple[0].to_i * tuple[1].to_i }
    .sum
end

def solve_part_2 lines
  toggle = true

  lines
    .join
    .scan(/mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)/)
    .filter_map do |tuple|
      case tuple
      when "don't()"
        toggle = false
        nil
      when "do()"
        toggle = true
        nil
      else
        tuple.scan(/mul\((?<n1>\d{1,3}),(?<n2>\d{1,3})\)/).first.map(&:to_i).reduce(&:*) if toggle
      end
    end.sum
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample_input
    assert_equal(161, solve_part_1(File.readlines("test_input_p1")))
  end

  def test_part_1_real_input
    assert_equal(164730528, solve_part_1(File.readlines("input")))
  end

  def test_part_2_sample_input
    assert_equal(48, solve_part_2(File.readlines("test_input_p2")))
  end

  def test_part_2_real_input
    assert_equal(70478672, solve_part_2(File.readlines("input")))
  end
end
