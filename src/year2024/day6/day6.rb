#!/usr/bin/env ruby

def solve_part_1 input
  grid = input.split

  row_count = grid.size
  col_count = grid.first.size # last char is \n

  start_idx = input.index('^')
  direction = '^'

  cur_tile = {
    x: start_idx / (row_count + 1),
    y: start_idx % (row_count + 1),
  }

  tiles_visited = [cur_tile].to_set

  while true
    next_tile = case direction
                when "^"
                  { x: cur_tile[:x] - 1, y: cur_tile[:y] }
                when ">"
                  { x: cur_tile[:x], y: cur_tile[:y] + 1 }
                when "v"
                  { x: cur_tile[:x] + 1, y: cur_tile[:y] }
                when "<"
                  { x: cur_tile[:x], y: cur_tile[:y] - 1 }
                else
                  raise "wrong direction"
                end

    if next_tile[:x] < 0 || next_tile[:x] >= row_count || next_tile[:y] < 0 || next_tile[:y] >= col_count
      return tiles_visited
    elsif grid[next_tile[:x]][next_tile[:y]] == "#"
      direction = case direction
                  when "^"
                    ">"
                  when ">"
                    "v"
                  when "v"
                    "<"
                  when "<"
                    "^"
                  else
                    raise "invalid direction"
                  end
    else
      cur_tile = next_tile
      tiles_visited << cur_tile
    end
  end
end

def solve_part_2 input, tiles
  grid = input.split

  row_count = grid.size
  col_count = grid.first.size # last char is \n

  start_idx = input.index('^')
  direction = '^'

  start_tile = {
    x: start_idx / (row_count + 1),
    y: start_idx % (row_count + 1),
  }

  result = 0

  row_count.times do |row|
    col_count.times do |col|
      next unless tiles.include?({ x: row, y: col })

      cur_grid = Marshal.load(Marshal.dump(grid)) # deep_dup
      cur_grid[row][col] = '#' unless row == start_tile[:x] && col == start_tile[:y]

      # prepare to walk grid with cycle detection
      direction = "^"
      keep_walking = true
      cur_tile = start_tile.dup
      tiles_visited = ["#{cur_tile[:x]}-#{cur_tile[:y]}-#{direction}"].to_set

      while keep_walking
        next_tile = case direction
                    when "^"
                      { x: cur_tile[:x] - 1, y: cur_tile[:y] }
                    when ">"
                      { x: cur_tile[:x], y: cur_tile[:y] + 1 }
                    when "v"
                      { x: cur_tile[:x] + 1, y: cur_tile[:y] }
                    when "<"
                      { x: cur_tile[:x], y: cur_tile[:y] - 1 }
                    else
                      raise "wrong direction"
                    end

        if tiles_visited.include? "#{next_tile[:x]}-#{next_tile[:y]}-#{direction}"
          result += 1
          keep_walking = false
        elsif next_tile[:x] < 0 || next_tile[:x] >= row_count || next_tile[:y] < 0 || next_tile[:y] >= col_count
          keep_walking = false
        elsif cur_grid[next_tile[:x]][next_tile[:y]] == "#"
          direction = case direction
                      when "^"
                        ">"
                      when ">"
                        "v"
                      when "v"
                        "<"
                      when "<"
                        "^"
                      else
                        raise "invalid direction"
                      end
        else
          cur_tile = next_tile
          tiles_visited << "#{cur_tile[:x]}-#{cur_tile[:y]}-#{direction}"
        end
      end
    end
  end

  result
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample_input
    assert_equal(41, solve_part_1(File.read("test_input")).size)
  end

  def test_part_1_real_input
    assert_equal(5131, solve_part_1(File.read("input")).size)
  end

  def test_part_2_sample_input
    test_input = File.read("test_input")
    tiles = solve_part_1(test_input)
    assert_equal(6, solve_part_2(test_input, tiles))
  end

  def test_part_2_real_input
    input = File.read("input")
    tiles = solve_part_1(input)
    assert_equal(1784, solve_part_2(input, tiles))
  end
end
