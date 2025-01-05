#!/usr/bin/env ruby

require_relative '../computer'

def solve_part_1(input)
  Computer.execute!(input.chomp.split(",").map(&:to_i))
end

def solve_part_2(input)
  program = input.chomp.split(",").map(&:to_i)

  0.upto(99) do |noun|
    0.upto(99) do |verb|
      memory = program.dup
      memory[1] = noun
      memory[2] = verb

      Computer.execute!(memory)

      return 100 * noun + verb if memory[0] == 19690720
    end
  end

  raise "WTF"
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample
    assert_equal("3500,9,10,70,2,3,11,0,99,30,40,50", solve_part_1("1,9,10,3,2,3,11,0,99,30,40,50").join(","))
    assert_equal("2,0,0,0,99", solve_part_1("1,0,0,0,99").join(","))
    assert_equal("2,3,0,6,99", solve_part_1("2,3,0,3,99").join(","))
    assert_equal("30,1,1,4,2,5,6,0,99", solve_part_1("1,1,1,4,99,5,6,0,99").join(","))
  end

  def test_part_1_real
    input = File.read("input").chomp.split(",")
    input[1] = 12
    input[2] = 2
    assert_equal(10566835, solve_part_1(input.join(","))[0])
  end

  def test_part_2_real
    assert_equal(2347, solve_part_2(File.read("input")))
  end
end
