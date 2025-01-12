#!/usr/bin/env ruby

require_relative '../computer'

require 'minitest/autorun'

class Day5Test < Minitest::Test
  def test_part_1_sample
    memory = [1002,4,3,4,33]
    Computer.execute!(memory).join
    assert_equal([1002, 4, 3, 4, 99], memory)
  end

  def test_part_1_real
    memory = File.read("input").chomp.split(",").map(&:to_i)

    input_queue  = Thread::Queue.new
    output_queue = Thread::Queue.new

    input_queue << 1

    Computer.execute!(memory, input_queue, output_queue).join

    result = nil
    output_queue.size.times { result = output_queue.pop }

    assert_equal(15508323, result)
  end

  def test_part_2_real
    memory = File.read("input").chomp.split(",").map(&:to_i)

    input_queue  = Thread::Queue.new
    output_queue = Thread::Queue.new

    input_queue << 5

    Computer.execute!(memory, input_queue, output_queue).join

    result = nil
    output_queue.size.times { result = output_queue.pop }

    assert_equal(9006327, result)
  end
end
