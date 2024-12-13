#!/usr/bin/env ruby

Point = Struct.new(:x, :y)

def min_tokens(machine, acc)
  lines = machine.lines

  lines[0] =~ /X\+(\d+), Y\+(\d+)/
  button_a = Point.new($1.to_f, $2.to_f)

  lines[1] =~ /X\+(\d+), Y\+(\d+)/
  button_b = Point.new($1.to_f, $2.to_f)

  lines[2] =~ /X=(\d+), Y=(\d+)/
  prize = Point.new(acc + $1.to_f, acc + $2.to_f)

  a = (prize.x*button_b.y - button_b.x*prize.y) / (button_a.x*button_b.y - button_b.x*button_a.y)
  return 0 unless a % 1 == 0

  b = (button_a.x*prize.y - prize.x*button_a.y) / (button_a.x*button_b.y - button_b.x*button_a.y)
  return 0 unless b % 1 == 0

  (3 * a + b).to_i
end

def solve_part_1(input)
  input.split("\n\n").sum { min_tokens(it, 0) }
end

def solve_part_2(input)
  input.split("\n\n").sum { min_tokens(it, 10000000000000) }
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample_input
    assert_equal(480, solve_part_1(File.read("test_input")))
  end

  def test_part_1_real_input
    assert_equal(29598, solve_part_1(File.read("input")))
  end

  def test_part_2_sample_input
    assert_equal(875318608908, solve_part_2(File.read("test_input")))
  end

  def test_part_2_real_input
    assert_equal(93217456941970, solve_part_2(File.read("input")))
  end
end
