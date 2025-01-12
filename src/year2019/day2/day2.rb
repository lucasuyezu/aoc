#!/usr/bin/env ruby

require_relative '../computer'

def solve_part_2(input)
  program = input.chomp.split(",").map(&:to_i)

  0.upto(99) do |noun|
    0.upto(99) do |verb|
      memory = program.dup
      memory[1] = noun
      memory[2] = verb

      Computer.execute!(memory).join

      return 100 * noun + verb if memory[0] == 19690720
    end
  end

  raise "WTF"
end

require 'minitest/autorun'

class Day2Test < Minitest::Test
  def test_part_1_sample
    memory = "1,9,10,3,2,3,11,0,99,30,40,50".split(",").map(&:to_i)
    Computer.execute!(memory).join
    assert_equal("3500,9,10,70,2,3,11,0,99,30,40,50", memory.join(","))

    memory = "1,0,0,0,99".split(",").map(&:to_i)
    Computer.execute!(memory).join
    assert_equal("2,0,0,0,99", memory.join(","))

    memory = "2,3,0,3,99".split(",").map(&:to_i)
    Computer.execute!(memory).join
    assert_equal("2,3,0,6,99", memory.join(","))

    memory = "1,1,1,4,99,5,6,0,99".split(",").map(&:to_i)
    Computer.execute!(memory).join
    assert_equal("30,1,1,4,2,5,6,0,99", memory.join(","))
  end

  def test_part_1_real
    memory = File.read("input").chomp.split(",").map(&:to_i)
    memory[1] = 12
    memory[2] = 2

    Computer.execute!(memory).join

    assert_equal(10566835, memory[0])
  end

  def test_part_2_real
    assert_equal(2347, solve_part_2(File.read("input")))
  end
end
