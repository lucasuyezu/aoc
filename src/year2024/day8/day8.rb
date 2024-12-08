#!/usr/bin/env ruby

Point = Struct.new(:x, :y)

def antennas input
  result = {}

  input.lines.each_with_index do |line, x|
    line.strip.chars.each_with_index do |char, y|
      next if char == "."

      result[char] ||= []
      result[char] << Point.new(x, y)
    end
  end

  result.values
end

def solve_part_1 input
  x_range = 0...input.lines.size
  y_range = 0...(input.lines.first.size - 1) # last char is \n

  antinodes_set = [].to_set
  antennas(input).each do
    it.permutation(2) do |a, b|
      v = Point.new(b.x - a.x, b.y - a.y)

      n = Point.new(a.x - v.x, a.y - v.y)
      antinodes_set << n if x_range === n.x && y_range === n.y

      n = Point.new(b.x + v.x, b.y + v.y)
      antinodes_set << n if x_range === n.x && y_range === n.y
    end
  end

  antinodes_set.size
end

def solve_part_2 input
  x_range = 0...input.lines.size
  y_range = 0...(input.lines.first.size - 1) # last char is \n

  antinodes_set = [].to_set
  antennas(input).each do
    it.permutation(2) do |a, b|
      antinodes_set << a
      antinodes_set << b

      v = Point.new(b.x - a.x, b.y - a.y)

      n = Point.new(a.x - v.x, a.y - v.y)
      while x_range === n.x && y_range === n.y
        antinodes_set << n
        n = Point.new(n.x - v.x, n.y - v.y)
      end

      n = Point.new(b.x + v.x, b.y + v.y)
      while x_range === n.x && y_range === n.y
        antinodes_set << n
        n = Point.new(n.x + v.x, n.y + v.y)
      end
    end
  end

  antinodes_set.size
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample_input
    assert_equal(14, solve_part_1(File.read("test_input")))
  end

  def test_part_1_real_input
    assert_equal(285, solve_part_1(File.read("input")))
  end

  def test_part_2_sample_input
    assert_equal(34, solve_part_2(File.read("test_input")))
  end

  def test_part_2_real_input
    assert_equal(944, solve_part_2(File.read("input")))
  end
end
