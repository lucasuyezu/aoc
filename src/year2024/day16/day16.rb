#!/usr/bin/env ruby

Point = Struct.new(:x, :y)
Node = Struct.new(:pos, :dir, :dist)

DIRECTIONS = [
  Point.new(-1, 0),
  Point.new(0, 1),
  Point.new(1, 0),
  Point.new(0, -1),
]

def solve_part_1(input)
  grid = input.lines

  start_idx = input.index("S")
  start_pos = Point.new(start_idx / grid.first.size, start_idx % grid.first.size)

  queue = [Node.new(start_pos, 1, 0)]

  goals = []

  distances = {}
  distances[start_pos] = 0

  while queue.any?
    node = queue.shift
    puts node.inspect

    neighbours = [
      {dir: node.dir, dist: 1},
      {dir: (node.dir + 1) % DIRECTIONS.size, dist: 1001},
      {dir: (node.dir - 1) % DIRECTIONS.size, dist: 1001},
    ].filter_map do |n|
      dir = DIRECTIONS[n[:dir]]
      n_pos = Point.new(node.pos.x + dir.x, node.pos.y + dir.y)

      n_dist = node.dist + n[:dist]

      goals << n_dist if grid[n_pos.x][n_pos.y] == "E"

      min_dist = n_dist < distances.fetch(n_pos, 10_000_000_000)

      if min_dist
        distances[n_pos] = n_dist
      end

      Node.new(
        n_pos,
        n[:dir],
        n_dist,
      ) if grid[n_pos.x][n_pos.y] == "." && min_dist
    end

    queue += neighbours
    # puts queue.inspect
    # puts visited.inspect
  end

  puts goals
  goals.min
end

def solve_part_2(input)
  raise NotImplementedError
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample_small
    assert_equal(7036, solve_part_1(File.read("test_input_small")))
  end

  def test_part_1_sample_large
    assert_equal(11048, solve_part_1(File.read("test_input")))
  end

  def test_part_1_real
    assert_equal(75416, solve_part_1(File.read("input")))
  end

  def test_part_2_sample_small
    assert_equal(618, solve_part_2(File.read("test_input_small_p2")))
  end

  def test_part_2_sample_large
    assert_equal(9021, solve_part_2(File.read("test_input")))
  end

  def test_part_2_real
    assert_equal(1453087, solve_part_2(File.read("input")))
  end
end
