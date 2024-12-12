#!/usr/bin/env ruby

def solve(input, blinks)
  stones = Hash[input.split(" ").map { [it, 1] }]

  blinks.times do
    new_stones = Hash.new(0)
    stones.each do |k, v|
      if k == "0"
        new_stones["1"] += v
      elsif k.size.even?
        left  = k[0...k.size/2]
        left = left.to_i.to_s
        new_stones[left] += v

        right = k[(k.size/2)...]
        right = right.to_i.to_s
        new_stones[right] += v
      else
        new_key = (k.to_i * 2024).to_s
        new_stones[new_key] = v
      end
    end

    stones = new_stones
  end

  stones.values.sum
end

def solve_part_1(input)
  solve(input, 25)
end

def solve_part_2(input)
  solve(input, 75)
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample_input
    assert_equal(55312, solve_part_1(File.read("test_input")))
  end

  def test_part_1_real_input
    assert_equal(199753, solve_part_1(File.read("input")))
  end

  def test_part_2_sample_input
    assert_equal(65601038650482, solve_part_2(File.read("test_input")))
  end

  def test_part_2_real_input
    assert_equal(239413123020116, solve_part_2(File.read("input")))
  end
end
