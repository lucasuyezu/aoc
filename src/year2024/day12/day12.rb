#!/usr/bin/env ruby

Point = Struct.new(:x, :y)

DIRECTIONS = [
  Point.new(-1, 0),
  Point.new(0, 1),
  Point.new(1, 0),
  Point.new(0, -1)
]

def map_region(grid, x_range, y_range, pos, char, visited, region)
  visited << pos
  region << pos

  DIRECTIONS.each do |dir|
    n = Point.new(pos.x + dir.x, pos.y + dir.y)

    next if visited.include?(n)

    next unless x_range.include?(n.x) && y_range.include?(n.y)
    next unless grid[n.x][n.y] == char

    puts "Adding plot #{n} to region #{char}"
    map_region(grid, x_range, y_range, n, char, visited, region)
  end
end

def perimeter(region)
  puts "Calculating perimeter for region #{region}"

  region.sum do |plot|
    4 - DIRECTIONS.count { region.include?(Point.new(plot.x + it.x, plot.y + it.y)) }
  end
end

def solve_part_1(input)
  grid = input.lines
  x_range = 0...grid.size
  y_range = 0...(grid.first.size - 1) # last char is \n

  regions = []
  visited = Set.new

  grid.each_with_index do |line, x|
    line.strip.chars.each_with_index do |char, y|
      pos = Point.new(x, y)
      next if visited.include?(pos)

      puts
      puts "Starting region at plot #{pos} with value #{char}"
      region = Set.new
      map_region(grid, x_range, y_range, pos, char, visited, region)
      puts "Region has #{region.size} plots"
      regions << region
    end
  end

  # puts visited.inspect
  # puts regions.inspect

  regions.sum { it.size  * perimeter(it) }
end

def solve_part_2(input)
  raise NotImplementedError
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample_input
    assert_equal(1930, solve_part_1(File.read("test_input")))
  end

  def test_part_1_real_input
    assert_equal(1363682, solve_part_1(File.read("input")))
  end

  def test_part_2_sample_input
    assert_equal(0, solve_part_2(File.read("test_input")))
  end

  def test_part_2_real_input
    assert_equal(0, solve_part_2(File.read("input")))
  end
end
