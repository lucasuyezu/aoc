#!/usr/bin/env ruby

Node = Struct.new(:node, :count)

def solve_part_1(input)
  orbits = input.lines.map(&:chomp).each_with_object({}) do |line, hash|
    from, to = line.split(")")
    hash[from] ||= []
    hash[from] << to
  end

  ans = 0

  queue = [Node.new("COM", 0)]

  while queue.any?
    node = queue.shift

    ans += node.count

    orbits[node.node].to_a.each do |orbit|
      queue << Node.new(orbit, node.count + 1)
    end
  end

  ans
end

def solve_part_2(input)
  orbits = input.lines.map(&:chomp).each_with_object({}) do |line, hash|
    from, to = line.split(")")
    hash[to] = from
  end

  node = "YOU"
  count = 0
  you_orbits = {}

  while orbits[node] != "COM"
    node = orbits[node]
    count += 1
    you_orbits[orbits[node]] = count
  end

  node = "SAN"
  count = 0
  san_orbits = {}

  you_orbits_set = you_orbits.keys.to_set

  while !you_orbits_set.include?(orbits[node])
    node = orbits[node]
    count += 1
    san_orbits[orbits[node]] = count
  end

  key, _ = you_orbits.keys & san_orbits.keys

  you_orbits[key] + san_orbits[key]
end

require 'minitest/autorun'

class Day5Test < Minitest::Test
  def test_part_1_sample
    assert_equal(42, solve_part_1(File.read("test_input")))
  end

  def test_part_1_real
    assert_equal(621125, solve_part_1(File.read("input")))
  end

  def test_part_2_sample
    assert_equal(4, solve_part_2(File.read("test_input_p2")))
  end

  def test_part_2_real
    assert_equal(550, solve_part_2(File.read("input")))
  end
end
