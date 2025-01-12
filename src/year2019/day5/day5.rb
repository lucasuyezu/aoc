#!/usr/bin/env ruby

require_relative '../computer'

require 'minitest/autorun'

class Day5Test < Minitest::Test
  def test_part_1_sample
    assert_equal([1002, 4, 3, 4, 99], Computer.new(memory: [1002,4,3,4,33]).execute_sync.memory)
  end

  def test_part_1_real
    memory = File.read("input").chomp.split(",").map(&:to_i)

    input_queue  = Thread::Queue.new
    output_queue = Thread::Queue.new

    input_queue << 1

    computer = Computer.new(memory:, input_queue:, output_queue:).execute_sync

    result = nil
    output_queue.size.times { result = output_queue.pop }

    assert_equal(15508323, result)
  end

  def test_part_2_real
    memory = File.read("input").chomp.split(",").map(&:to_i)

    input_queue  = Thread::Queue.new
    output_queue = Thread::Queue.new

    input_queue << 5

    computer = Computer.new(memory:, input_queue:, output_queue:).execute_sync

    result = nil
    output_queue.size.times { result = output_queue.pop }

    assert_equal(9006327, result)
  end
end
