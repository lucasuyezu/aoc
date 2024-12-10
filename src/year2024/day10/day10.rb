#!/usr/bin/env ruby

Point = Struct.new(:x, :y)

def trailheads(input)
  grid = input.lines
  x_range = 0...grid.size
  y_range = 0...(grid.first.size - 1) # last char is \n

  trailheads = []

  grid.each_with_index do |line, x|
    line.chars.each_with_index do |char, y|
      next unless char == "0"
      trailheads << Point.new(x, y)
    end
  end

  [grid, x_range, y_range, trailheads]
end

def peaks(trailhead, grid, x_range, y_range, peaks)
  val = grid[trailhead.x][trailhead.y]

  if val == "9"
    peaks << trailhead
    return peaks
  end

  n_val = (val.to_i + 1).to_s

  [
    Point.new(trailhead.x - 1, trailhead.y),
    Point.new(trailhead.x, trailhead.y + 1),
    Point.new(trailhead.x + 1, trailhead.y),
    Point.new(trailhead.x, trailhead.y - 1),
  ].select { x_range.include?(it.x) && y_range.include?(it.y) && grid[it.x][it.y] == n_val }.each do |n|
    peaks(n, grid, x_range, y_range, peaks)
  end

  peaks
end

def solve_part_1 input
  grid, x_range, y_range, trailheads = trailheads(input)

  trailheads
    .map { peaks(it, grid, x_range, y_range, [].to_set) }
    .sum { it.size }
end

def solve_part_2 input
  grid, x_range, y_range, trailheads = trailheads(input)

  trailheads
    .map { peaks(it, grid, x_range, y_range, []) }
    .sum { it.size }
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample_input
    assert_equal(36, solve_part_1(File.read("test_input")))
  end

  def test_part_1_real_input
    assert_equal(682, solve_part_1(File.read("input")))
  end

  def test_part_2_sample_input
    assert_equal(81, solve_part_2(File.read("test_input")))
  end

  def test_part_2_real_input
    assert_equal(1511, solve_part_2(File.read("input")))
  end
end
