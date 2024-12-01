def solve_part_1 lines
  left = []
  right = []

  lines.each do
    left_ele, right_ele = it.split
    left << left_ele.to_i
    right << right_ele.to_i
  end

  left.sort!
  right.sort!

  left
    .each_with_index
    .map { |ele, idx| (ele - right[idx]).abs }
    .sum
end

def solve_part_2 lines
  left = []
  right = []

  lines.each do
    left_ele, right_ele = it.split
    left << left_ele.to_i
    right << right_ele.to_i
  end

  left
    .map { |left_ele| left_ele * right.select { |right_ele| right_ele == left_ele }.size }
    .sum
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample_input
    assert_equal(11, solve_part_1(File.readlines("test_input")))
  end

  def test_part_1_real_input
    assert_equal(1320851, solve_part_1(File.readlines("input")))
  end

  def test_part_2_sample_input
    assert_equal(31, solve_part_2(File.readlines("test_input")))
  end

  def test_part_2_real_input
    assert_equal(26859182, solve_part_2(File.readlines("input")))
  end
end
