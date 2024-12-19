#!/usr/bin/env ruby

def check(design, towels, cache)
  return cache[design] if cache.key? design
  return 1 if design == ""

  ans = 1.upto(design.size)
    .select { towels.include?(design[0...it]) }
    .sum { check(design[it..], towels, cache) }

  cache[design] = ans
  ans
end

def solve_part_1(input)
  towels, designs = input.split("\n\n")
  towels = towels.split(", ")

  cache = {}
  designs.lines.map(&:chomp).count { check(it, towels, cache) > 0 }
end

def solve_part_2(input)
  towels, designs = input.split("\n\n")
  towels = towels.split(", ")

  cache = {}
  designs.lines.map(&:chomp).sum { check(it, towels, cache) }
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample
    assert_equal(6, solve_part_1(File.read("test_input")))
  end

  def test_part_1_real
    assert_equal(251, solve_part_1(File.read("input")))
  end

  def test_part_2_sample
    assert_equal(16, solve_part_2(File.read("test_input")))
  end

  def test_part_2_real
    assert_equal(616957151871345, solve_part_2(File.read("input")))
  end
end
