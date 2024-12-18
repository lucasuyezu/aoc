#!/usr/bin/env ruby

Point = Struct.new(:x, :y)
Node = Struct.new(:pos, :dist, :path)

DIRECTIONS = [
  Point.new(-1, 0),
  Point.new(0, 1),
  Point.new(1, 0),
  Point.new(0, -1),
]

def build_grid(input, size)
  grid = []

  (size + 1).times { grid << "." * (size + 1) }

  grid
end

def solve(grid)
  start_pos = Point.new(0, 0)

  queue = [Node.new(start_pos, 0, Set.new)]

  goals = []

  distances = {}
  distances[start_pos] = 0

  size_range = 0...grid.size

  while queue.any?
    node = queue.shift

    if node.pos.x == size_range.max && node.pos.y == size_range.max
      goals << node
      next
    end

    queue += DIRECTIONS.filter_map do |dir|
      nn = Node.new(
        Point.new(node.pos.x + dir.x, node.pos.y + dir.y),
        node.dist + 1,
        node.path.dup,
      )
      nn.path << nn.pos

      next unless size_range.include?(nn.pos.x) && size_range.include?(nn.pos.y)

      min_dist = nn.dist < distances.fetch(nn.pos, 1e10)
      next unless min_dist

      distances[nn.pos] = nn.dist

      nn if grid[nn.pos.x][nn.pos.y] == "."
    end
  end

  goals.min_by { it.dist }
end

def solve_part_1(input, size, bytes)
  grid = build_grid(input, size)

  input.lines.take(bytes).each do
    y, x = it.split(",").map(&:to_i)
    grid[x][y] = "#"
  end

  solve(grid).dist
end

def solve_part_2(input, size, bytes)
  grid = build_grid(input, size)

  best_path = nil

  input.lines.map(&:chomp).each_with_index.find do |line, n|
    y, x = line.split(",").map(&:to_i)
    grid[x][y] = "#"

    next if n < bytes
    next unless best_path.nil? || best_path.path.include?(Point.new(x, y))

    best_path = solve(grid)
    best_path.nil?
  end.first
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample
    assert_equal(22, solve_part_1(File.read("test_input"), 6, 12))
  end

  def test_part_1_real
    assert_equal(268, solve_part_1(File.read("input"), 70, 1024))
  end

  def test_part_2_sample
    assert_equal("6,1", solve_part_2(File.read("test_input"), 6, 12))
  end

  def test_part_2_real
    assert_equal("64,11", solve_part_2(File.read("input"), 70, 1024))
  end
end
