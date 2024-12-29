#!/usr/bin/env ruby
require 'lazy_priority_queue'

Point = Struct.new(:x, :y)
Node = Struct.new(:pos, :dir, :dist, :path)

DIRECTIONS = [
  Point.new(-1, 0),
  Point.new(0, 1),
  Point.new(1, 0),
  Point.new(0, -1),
]

def build_grid(input)
  grid = input.lines

  start_idx = input.index("S")
  start_pos = Point.new(start_idx / grid.first.size, start_idx % grid.first.size)

  end_idx = input.index("E")
  end_pos = Point.new(end_idx / grid.first.size, end_idx % grid.first.size)

  [start_pos, end_pos, grid]
end

def dijkstra(grid, start, dir, goal)
  queue = MinPriorityQueue.new
  queue.enqueue(Node.new(start, dir, 0, [start]), 0)

  dists = Hash.new(1e10)
  dists[Node.new(start, dir)] = 0

  visited = Set.new

  min_dist = 1e10

  while !queue.empty?
    node = queue.pop

    next if node.dist > dists[Node.new(node.pos, node.dir)]

    if node.pos == goal
      if node.dist <= min_dist
        min_dist = node.dist
        visited += node.path
      end
      next
    end

    [
      {dir: node.dir, dist: 1},
      {dir: (node.dir + 1) % DIRECTIONS.size, dist: 1001},
      {dir: (node.dir - 1) % DIRECTIONS.size, dist: 1001},
    ].each do |n|
      nn = Node.new(
        nil,
        n[:dir],
        node.dist + n[:dist],
        node.path.dup
      )
      nn.pos = Point.new(node.pos.x + DIRECTIONS[nn.dir].x, node.pos.y + DIRECTIONS[nn.dir].y)
      nn.path << nn.pos

      n_char = grid[nn.pos.x][nn.pos.y]

      dist_key = Node.new(nn.pos, nn.dir)
      if n_char != "#" && nn.dist <= dists[dist_key]
        dists[dist_key] = nn.dist
        queue.enqueue(nn, nn.dist)
      end
    end
  end

  [min_dist, visited]
end

def solve(input)
  start, goal, grid = build_grid(input)
  dijkstra(grid, start, 1, goal)
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_sample_input_small
    min_dist, visited = solve(File.read("test_input_small"))
    assert_equal(7036, min_dist)
    assert_equal(45, visited.size)
  end

  def test_sample_input_large
    min_dist, visited = solve(File.read("test_input"))
    assert_equal(11048, min_dist)
    assert_equal(64, visited.size)
  end

  def test_real_input
    min_dist, visited = solve(File.read("input"))
    assert_equal(75416, min_dist)
    assert_equal(476, visited.size)
  end
end
