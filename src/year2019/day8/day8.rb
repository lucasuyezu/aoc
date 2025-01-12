#!/usr/bin/env ruby

def solve_part_1(input, width, length)
  min_layer = input
    .chomp
    .chars
    .each_slice(width * length)
    .map(&:tally)
    .min_by { it["0"].to_i }

  min_layer["1"] * min_layer["2"]
end

def solve_part_2(input, width, length)
  layer_length = width * length

  input
    .chomp
    .chars
    .each_slice(layer_length)
    .map { it }
    .reverse
    .reduce(["."] * layer_length) do |acc, layer|
      layer_length.times do |i|
        acc[i] = layer[i] if layer[i] != "2"
      end
      acc
    end
end

require 'minitest/autorun'

class Day5Test < Minitest::Test
  def test_part_1_sample
    assert_equal(1, solve_part_1(File.read("test_input"), 3, 2))
  end

  def test_part_1_real
    assert_equal(2193, solve_part_1(File.read("input"), 25, 6))
  end

  def test_part_2_sample
    assert_equal("0110", solve_part_2(File.read("test_input_p2"), 2, 2).join)
  end

  def test_part_2_real
    # puts solve_part_2(File.read("input"), 25, 6).each_slice(25).map { it.join.gsub("0", ".").gsub("1", "#") }
    assert_equal("100011111010010111101111010001100001001010000100000101011100111101110011100001001000010010100001000000100100001001010000100000010011110100101111010000", solve_part_2(File.read("input"), 25, 6).join)
  end
end
