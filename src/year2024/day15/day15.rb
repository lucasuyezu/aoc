#!/usr/bin/env ruby

Point = Struct.new(:x, :y)

DIRECTIONS = {
  "^" => Point.new(-1, 0),
  ">" => Point.new(0, 1),
  "v" => Point.new(1, 0),
  "<" => Point.new(0, -1),
}

def parse(input, expand)
  map, moves = input.split("\n\n")

  grid = nil

  if expand
    grid = []
    line = String.new
    map.chars.each do |c|
      case c
      when "#"
        line << "##"
      when "O"
        line << "[]"
      when "."
        line << ".."
      when "@"
        line << "@."
      when "\n"
        grid << line
        line = String.new
      end
    end
    grid << line
  else
    grid = map.lines.map(&:strip)
  end

  grid.each_with_index do |line, x|
    line.chars.each_with_index do |char, y|
      next unless grid[x][y] == "@"

      return [grid, moves.gsub("\n", ""), Point.new(x, y)]
    end
  end

  raise "WTF"
end

def execute_move(move, grid, robot)
  dir = DIRECTIONS[move]
  n_pos = Point.new(robot.x + dir.x, robot.y + dir.y)
  n_char = grid[n_pos.x][n_pos.y]

  case n_char
  when "."
    grid[n_pos.x][n_pos.y] = "@"
    grid[robot.x][robot.y] = "."
    robot.x = n_pos.x
    robot.y = n_pos.y
  when "O", "[", "]"
    yield n_pos, dir
  end
end

def simple_push(grid, robot, n_pos, dir)
  # figure out how many boxes to push
  # n_pos is O
  # keep going into dir until you find a . or a #
  box_count = 0
  n_char = grid[n_pos.x][n_pos.y]
  while n_char == "O" || n_char == "[" || n_char == "]"
    box_count += 1
    n_pos.x += dir.x
    n_pos.y += dir.y
    n_char = grid[n_pos.x][n_pos.y]
  end

  if grid[n_pos.x][n_pos.y] == "."
    # move all the boxes
    box_count.times do
      grid[n_pos.x][n_pos.y] = grid[n_pos.x - dir.x][n_pos.y - dir.y]
      n_pos.x -= dir.x
      n_pos.y -= dir.y
    end
    grid[n_pos.x][n_pos.y] = "@"
    grid[robot.x][robot.y] = "."
    robot.x = n_pos.x
    robot.y = n_pos.y
  end
end

def gps(grid)
  gps = 0
  grid.each_with_index do |line, x|
    line.chars.each_with_index do |c, y|
      gps += 100 * x + y if c == "O" || c == "["
    end
  end
  gps
end

def solve_part_1(input)
  grid, moves, robot = parse(input, false)

  moves.chars.each do
    execute_move(it, grid, robot) do |n_pos, dir|
      simple_push(grid, robot, n_pos, dir)
    end
  end

  gps(grid)
end

def solve_part_2(input)
  grid, moves, robot = parse(input, true)

  moves.chars.each do |move|
    execute_move(move, grid, robot) do |n_pos, dir|
      if move == ">" || move == "<"
        simple_push(grid, robot, n_pos, dir)
      else
        tree_nodes = Set.new
        if can_move_tree(grid, n_pos, dir, tree_nodes)
          tree_nodes.each do |to_move|
            grid[to_move[:from].x][to_move[:from].y] = "."
          end
          tree_nodes.each do |to_move|
            grid[to_move[:to].x][to_move[:to].y] = to_move[:char]
          end

          grid[n_pos.x][n_pos.y] = "@"
          grid[robot.x][robot.y] = "."
          robot.x = n_pos.x
          robot.y = n_pos.y
        end
      end
    end
  end

  gps(grid)
end

def can_move_tree(grid, node, dir, tree_nodes)
  char = grid[node.x][node.y]

  return false if char == "#"
  return true  if char == "."

  if char == "["
    l_pos = node.dup
    r_pos = Point.new(node.x, node.y + 1)
  elsif char == "]"
    l_pos = Point.new(node.x, node.y - 1)
    r_pos = node.dup
  end

  left_node  = Point.new(l_pos.x + dir.x, l_pos.y + dir.y)
  right_node = Point.new(r_pos.x + dir.x, r_pos.y + dir.y)

  tree_nodes << { from: l_pos, to: left_node,  char: "[" }
  tree_nodes << { from: r_pos, to: right_node, char: "]" }

  can_move_tree(grid, left_node, dir, tree_nodes) && can_move_tree(grid, right_node, dir, tree_nodes)
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample_small
    assert_equal(2028, solve_part_1(File.read("test_input_small")))
  end

  def test_part_1_sample_large
    assert_equal(10092, solve_part_1(File.read("test_input")))
  end

  def test_part_1_real
    assert_equal(1429911, solve_part_1(File.read("input")))
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
