#!/usr/bin/env ruby

Point = Struct.new(:x, :y)

DIRECTIONS = {
  "^" => Point.new(-1, 0),
  ">" => Point.new(0, 1),
  "v" => Point.new(1, 0),
  "<" => Point.new(0, -1),
}

def map_region(grid, x_range, y_range, pos, char, visited, region)
  visited << pos
  region << pos

  DIRECTIONS.values.each do |dir|
    n = Point.new(pos.x + dir.x, pos.y + dir.y)

    next if visited.include?(n)
    next unless x_range.include?(n.x) && y_range.include?(n.y)
    next unless grid[n.x][n.y] == char

    map_region(grid, x_range, y_range, n, char, visited, region)
  end
end

def perimeter(region)
  region.sum do |plot|
    4 - DIRECTIONS.values.count { region.include?(Point.new(plot.x + it.x, plot.y + it.y)) }
  end
end

def sides(region)
  region.sum do |plot|
    corners = 0

    south = Point.new(plot.x + DIRECTIONS["v"].x, plot.y + DIRECTIONS["v"].y)
    west  = Point.new(plot.x + DIRECTIONS["<"].x, plot.y + DIRECTIONS["<"].y)
    east  = Point.new(plot.x + DIRECTIONS[">"].x, plot.y + DIRECTIONS[">"].y)
    north = Point.new(plot.x + DIRECTIONS["^"].x, plot.y + DIRECTIONS["^"].y)

    if !region.include?(south) && !region.include?(west)
      corners += 1
    end

    if !region.include?(west) && !region.include?(north)
      corners += 1
    end

    if !region.include?(north) && !region.include?(east)
      corners += 1
    end

    if !region.include?(east) && !region.include?(south)
      corners += 1
    end

    northeast = Point.new(plot.x + DIRECTIONS["^"].x + DIRECTIONS[">"].x, plot.y + DIRECTIONS["^"].y + DIRECTIONS[">"].y)
    if region.include?(north) && region.include?(east) && !region.include?(northeast)
      corners += 1
    end

    northwest = Point.new(plot.x + DIRECTIONS["^"].x + DIRECTIONS["<"].x, plot.y + DIRECTIONS["^"].y + DIRECTIONS["<"].y)
    if region.include?(north) && region.include?(west) && !region.include?(northwest)
      corners += 1
    end

    southeast = Point.new(plot.x + DIRECTIONS["v"].x + DIRECTIONS[">"].x, plot.y + DIRECTIONS["v"].y + DIRECTIONS[">"].y)
    if region.include?(south) && region.include?(east) && !region.include?(southeast)
      corners += 1
    end

    southwest = Point.new(plot.x + DIRECTIONS["v"].x + DIRECTIONS["<"].x, plot.y + DIRECTIONS["v"].y + DIRECTIONS["<"].y)
    if region.include?(south) && region.include?(west) && !region.include?(southwest)
      corners += 1
    end

    corners
  end
end

def regions(input)
  grid = input.lines
  x_range = 0...grid.size
  y_range = 0...(grid.first.size - 1) # last char is \n

  regions = []
  visited = Set.new

  grid.each_with_index do |line, x|
    line.strip.chars.each_with_index do |char, y|
      pos = Point.new(x, y)
      next if visited.include?(pos)

      region = Set.new
      map_region(grid, x_range, y_range, pos, char, visited, region)
      regions << region
    end
  end

  regions
end

def solve_part_1(input)
  regions(input).sum { it.size  * perimeter(it) }
end

def solve_part_2(input)
  regions(input).sum { it.size * sides(it) }
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
    assert_equal(1206, solve_part_2(File.read("test_input")))
  end

  def test_part_2_real_input
    assert_equal(787680, solve_part_2(File.read("input")))
  end
end
