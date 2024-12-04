#!/usr/bin/env ruby

DIRECTIONS = [
  [{i: -1, j:  0, c: "M"},{i: -2, j:  0, c: "A"},{i: -3, j:  0, c: "S"}], # go up
  [{i: -1, j:  1, c: "M"},{i: -2, j:  2, c: "A"},{i: -3, j:  3, c: "S"}], # go up-right
  [{i:  0, j:  1, c: "M"},{i:  0, j:  2, c: "A"},{i:  0, j:  3, c: "S"}], # go right
  [{i:  1, j:  1, c: "M"},{i:  2, j:  2, c: "A"},{i:  3, j:  3, c: "S"}], # go right-down
  [{i:  1, j:  0, c: "M"},{i:  2, j:  0, c: "A"},{i:  3, j:  0, c: "S"}], # go down
  [{i:  1, j: -1, c: "M"},{i:  2, j: -2, c: "A"},{i:  3, j: -3, c: "S"}], # go left-down
  [{i:  0, j: -1, c: "M"},{i:  0, j: -2, c: "A"},{i:  0, j: -3, c: "S"}], # go left
  [{i: -1, j: -1, c: "M"},{i: -2, j: -2, c: "A"},{i: -3, j: -3, c: "S"}], # go left-up
]

def solve_part_1 lines
  result = 0
  max_i = lines.size
  max_j = lines.first.size - 1 # last char is \n

  lines.map(&:strip).each_with_index do |line, i|
    line.chars.each_with_index do |char, j|
      if char == "X"
        DIRECTIONS.each do |direction|
          if direction.all? do |x|
            ii = i + x[:i]
            jj = j + x[:j]

            ii >= 0 && ii < max_i &&
            jj >= 0 && jj < max_j &&
            lines[ii][jj] == x[:c]
          end
            result += 1
          end
        end
      end
    end
  end

  result
end

def solve_part_2 lines
  result = 0
  max_i = lines.size
  max_j = lines.first.size - 1 # last char is \n

  lines.map(&:strip).each_with_index do |line, i|
    line.chars.each_with_index do |char, j|
      if char == "A" && i > 0 && j > 0 && i < max_i - 1 && j < max_j - 1
        x = [
          lines[i - 1][j - 1],
          lines[i + 1][j + 1],
        ]

        y = [
          lines[i - 1][j + 1],
          lines[i + 1][j - 1],
        ]

        if x.sort.join == "MS" && y.sort.join == "MS"
          result += 1
        end
      end
    end
  end

  result
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample_input
    assert_equal(18, solve_part_1(File.readlines("test_input")))
  end

  def test_part_1_real_input
    assert_equal(2464, solve_part_1(File.readlines("input")))
  end

  def test_part_2_sample_input
    assert_equal(9, solve_part_2(File.readlines("test_input")))
  end

  def test_part_2_real_input
    assert_equal(1982, solve_part_2(File.readlines("input")))
  end
end
