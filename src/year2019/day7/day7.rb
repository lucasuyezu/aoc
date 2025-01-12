#!/usr/bin/env ruby

require_relative '../computer'

def solve_part_1(memory)
  [0,1,2,3,4].permutation.map do |phase_setting|
    in_a = Thread::Queue.new
    in_a << phase_setting[0]
    in_a << 0

    out_a_in_b = Thread::Queue.new
    out_a_in_b << phase_setting[1]

    out_b_in_c = Thread::Queue.new
    out_b_in_c << phase_setting[2]

    out_c_in_d = Thread::Queue.new
    out_c_in_d << phase_setting[3]

    out_d_in_e = Thread::Queue.new
    out_d_in_e << phase_setting[4]

    out_e = Thread::Queue.new

    Computer.execute!(memory.dup, in_a,       out_a_in_b)
    Computer.execute!(memory.dup, out_a_in_b, out_b_in_c)
    Computer.execute!(memory.dup, out_b_in_c, out_c_in_d)
    Computer.execute!(memory.dup, out_c_in_d, out_d_in_e)
    Computer.execute!(memory.dup, out_d_in_e, out_e)

    out_e.pop
  end.max
end

def solve_part_2(memory)
  [5,6,7,8,9].permutation.map do |phase_setting|
    out_e_in_a = Thread::Queue.new
    out_e_in_a << phase_setting[0]
    out_e_in_a << 0

    out_a_in_b = Thread::Queue.new
    out_a_in_b << phase_setting[1]

    out_b_in_c = Thread::Queue.new
    out_b_in_c << phase_setting[2]

    out_c_in_d = Thread::Queue.new
    out_c_in_d << phase_setting[3]

    out_d_in_e = Thread::Queue.new
    out_d_in_e << phase_setting[4]

    Computer.execute!(memory.dup, out_e_in_a, out_a_in_b)
    Computer.execute!(memory.dup, out_a_in_b, out_b_in_c)
    Computer.execute!(memory.dup, out_b_in_c, out_c_in_d)
    Computer.execute!(memory.dup, out_c_in_d, out_d_in_e)
    Computer.execute!(memory.dup, out_d_in_e, out_e_in_a).join

    out_e_in_a.pop
  end.max
end

require 'minitest/autorun'

class Day5Test < Minitest::Test
  def test_part_1_sample
    assert_equal(43210, solve_part_1([3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]))
    assert_equal(54321, solve_part_1([3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0]))
    assert_equal(65210, solve_part_1([3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0]))
  end

  def test_part_1_real
    memory = File.read("input").chomp.split(",").map(&:to_i)
    assert_equal(437860, solve_part_1(memory))
  end

  def test_part_2_sample
    assert_equal(139629729, solve_part_2([3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5]))
    assert_equal(18216, solve_part_2([3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10]))
  end

  def test_part_2_real
    memory = File.read("input").chomp.split(",").map(&:to_i)
    assert_equal(49810599, solve_part_2(memory))
  end
end
