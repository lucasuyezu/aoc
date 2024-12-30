#!/usr/bin/env ruby

def is_valid?(rule, update)
  first_idx = update.find_index(rule.first)
  last_idx = update.find_index(rule.last)

  first_idx.nil? || last_idx.nil? || first_idx < last_idx
end

def solve_part_1 input
  rules, updates = input.split("\n\n")

  rules = rules.split.map { it.split("|").map(&:to_i) }

  x = updates
    .split
    .map { it.split(",").map(&:to_i) }
    .select { |update| rules.all? { |rule| is_valid?(rule, update) } }
    .map { it[it.size/2] }
    .sum
end

def solve_part_2 input
  rules, updates = input.split("\n\n")

  rules = rules.split.map { it.split("|").map(&:to_i) }

  updates
    .split
    .map { it.split(",").map(&:to_i) }
    .reject { |update| rules.all? { |rule| is_valid?(rule, update) } }
    .map do |update|
      update.sort do |a, b|
        if rules.any? { |rule| rule.first == a && rule.last == b }
          -1
        elsif rules.any? { |rule| rule.first == b && rule.last == a }
          1
        else
          0
        end
      end
    end
    .map { it[it.size/2] }
    .sum
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample_input
    assert_equal(143, solve_part_1(File.read("test_input")))
  end

  def test_part_1_real_input
    assert_equal(5275, solve_part_1(File.read("input")))
  end

  def test_part_2_sample_input
    assert_equal(123, solve_part_2(File.read("test_input")))
  end

  def test_part_2_real_input
    assert_equal(6191, solve_part_2(File.read("input")))
  end
end
