#!/usr/bin/env ruby

Gate = Struct.new(:wire_a, :op, :wire_b, :ignore_me, :output_wire)

def solve_part_1(input)
  wires_str, gates_str = input.split("\n\n")

  wires = wires_str.lines.each_with_object({}) do |wire, hash|
    k, v = wire.chomp.split(": ")
    hash[k] = v == "1"
  end

  remaining_gates = gates_str.lines.map { Gate.new(*it.split(" ")) }

  while remaining_gates.any?
    current_gates, remaining_gates = remaining_gates.partition { wires.key?(it.wire_a) && wires.key?(it.wire_b ) }

    current_gates.each do |gate|
      wires[gate.output_wire] = case gate.op
                                when "AND"
                                  wires.fetch(gate.wire_a) && wires.fetch(gate.wire_b)
                                when "XOR"
                                  wires.fetch(gate.wire_a) ^ wires.fetch(gate.wire_b)
                                when "OR"
                                  wires.fetch(gate.wire_a) || wires.fetch(gate.wire_b)
                                else
                                  raise "Invalid operator #{gate.op}"
                                end
    end
  end

  z_wires = wires.keys.select { it.start_with?("z") }
  raise "There are no z wires" if z_wires.empty?

  z_wires.sort.reverse.reduce(0) do |acc, z_wire|
    acc <<= 1
    acc ^= (wires.fetch(z_wire) ? 1 : 0)
  end
end

def solve_part_2(input)
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample_small
    assert_equal(4, solve_part_1(File.read("test_input_small")))
  end

  def test_part_1_sample_large
    assert_equal(2024, solve_part_1(File.read("test_input_large")))
  end

  def test_part_1_real
    assert_equal(63168299811048, solve_part_1(File.read("input")))
  end

  def test_part_2_sample
    assert_equal("co,de,ka,ta", solve_part_2(File.read("test_input")))
  end

  def test_part_2_real
    assert_equal(1568, solve_part_2(File.read("input")))
  end
end
