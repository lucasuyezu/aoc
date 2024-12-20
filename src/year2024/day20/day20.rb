#!/usr/bin/env ruby

Point = Struct.new(:x, :y)
Node = Struct.new(:pos, :path)

DIRECTIONS = [
  Point.new(-1, 0),
  Point.new(0, 1),
  Point.new(1, 0),
  Point.new(0, -1),
]

def solve(input, max_cheat_length)
  grid = input.lines.map(&:chomp)

  start_idx = input.index("S")
  pos = Point.new(start_idx / (grid.first.size + 1), start_idx % (grid.first.size + 1))

  queue = [pos]
  steps = {}

  counter = 0

  while queue.any?
    node = queue.shift

    steps[node] = counter
    counter += 1

    break if grid[pos.x][pos.y] == "E"

    DIRECTIONS.each do |dir|
      n_pos = Point.new(node.x + dir.x, node.y + dir.y)

      next if grid[n_pos.x][n_pos.y] == "#"
      next if steps.key? n_pos

      queue << n_pos
    end
  end

  steps
    .keys
    .combination(2)
    .filter_map do |from, to|
      manhattan_distance = (from.x - to.x).abs + (from.y - to.y).abs
      next unless manhattan_distance <= max_cheat_length

      savings = steps[to] - steps[from] - manhattan_distance
      savings if savings >= 100
    end
    .tally
    .values
    .sum
end

def solve_part_1(input)
  solve(input, 2)
end

def solve_part_2(input)
  solve(input, 20)
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample
    assert_equal(0, solve_part_1(File.read("test_input")))
  end

  def test_part_1_real
    assert_equal(1327, solve_part_1(File.read("input")))
  end

  def test_part_2_sample
    assert_equal(0, solve_part_2(File.read("test_input")))
  end

  def test_part_2_real
    assert_equal(985737, solve_part_2(File.read("input")))
  end
end
