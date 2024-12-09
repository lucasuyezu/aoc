#!/usr/bin/env ruby

def solve_part_1 input
  disk = []

  input.chars.each_with_index do |char, i|
    char.to_i.times do
      disk << if i.even?
        i / 2
      else
        "."
      end
    end
  end

  puts disk.inspect

  puts "expanded string has #{disk.size} blocks"

  dot_idx = 0
  blk_idx = disk.size - 1
  keep_going = true

  while keep_going
    dot_idx += 1 while disk[dot_idx] != "."
    blk_idx -= 1 while disk[blk_idx] == "."

    if dot_idx < blk_idx
      puts "swaping blocks #{dot_idx} and #{blk_idx}"

      disk[dot_idx] = disk[blk_idx]
      disk[blk_idx] = "."
    else
      keep_going = false
    end
  end

  puts disk.inspect

  checksum = 0

  disk.each_with_index do |blk, i|
    return checksum if blk == "."

    checksum += i * blk
  end

  raise "wtf"
end

def solve_part_2 input
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample_input
    assert_equal(1928, solve_part_1(File.read("test_input")))
  end

  def test_part_1_real_input
    assert_equal(6384282079460, solve_part_1(File.read("input")))
  end

  def test_part_2_sample_input
    assert_equal(0, solve_part_2(File.read("test_input")))
  end

  def test_part_2_real_input
    assert_equal(0, solve_part_2(File.read("input")))
  end
end
