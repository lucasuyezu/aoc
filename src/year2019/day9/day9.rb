#!/usr/bin/env ruby

require_relative '../computer'

require 'minitest/autorun'

class Day9Test < Minitest::Test
  def test_part_1_sample
    # computer = Computer.new(memory: [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99])
    # computer.execute_sync
    # assert_equal(1219070632396864, computer.output_queue.pop)

    computer = Computer.new(memory: [1102,34915192,34915192,7,4,7,99,0]).execute_async
    assert_equal(1219070632396864, computer.output_queue.pop)

    computer = Computer.new(memory: [104,1125899906842624,99]).execute_async
    assert_equal(1125899906842624, computer.output_queue.pop)
  end

  def test_part_1_real
    memory = File.read("input").chomp.split(",").map(&:to_i)

    input_queue = Thread::Queue.new
    input_queue << 1

    computer = Computer.new(memory:, input_queue:).execute_async
    assert_equal(2494485073, computer.output_queue.pop)
  end

  def test_part_2_real
    memory = File.read("input").chomp.split(",").map(&:to_i)

    input_queue = Thread::Queue.new
    input_queue << 2

    computer = Computer.new(memory:, input_queue:).execute_async
    assert_equal(44997, computer.output_queue.pop)
  end
end
