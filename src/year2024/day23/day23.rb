#!/usr/bin/env ruby

def solve_part_1(input)
  nodes = Set.new
  edges = Set.new

  input.lines.each do
    from, to = it.chomp.split("-")
    nodes << from
    nodes << to
    edges << "#{from}-#{to}"
    edges << "#{to}-#{from}"
  end

  nodes.to_a.combination(3).count do |combination|
    next unless combination.any? { it.start_with?("t") }

    puts "Combination #{combination.inspect} is a candidate"

    combination.combination(2).all? do |from, to|
      edges.include?("#{from}-#{to}")
    end
  end
end

def solve_part_2(input)
  raise NotImplementedError
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample
    assert_equal(7, solve_part_1(File.read("test_input")))
  end

  def test_part_1_real
    assert_equal(1077, solve_part_1(File.read("input")))
  end

  def test_part_2_sample
    assert_equal("co,de,ka,ta", solve_part_2(File.read("test_input")))
  end

  def test_part_2_real
    assert_equal(1568, solve_part_2(File.read("input")))
  end
end
