#!/usr/bin/env ruby

EmptyBlock = Struct.new(:size, :pos)
FileBlock = Struct.new(:id, :size, :pos)

def expand_disk(input)
  disk = []
  empty_blocks = []
  file_blocks = []
  input.chars.each_with_index do |char, i|
    blk_size = char.to_i
    next if blk_size.zero?

    if i.even?
      file_blocks << FileBlock.new(i / 2, blk_size, disk.size)
      blk_size.times { disk << i / 2 }
    else
      empty_blocks << EmptyBlock.new(blk_size, disk.size)
      blk_size.times { disk << "." }
    end
  end
  [disk, empty_blocks, file_blocks]
end

def solve_part_1 input
  disk, _ = expand_disk(input)
  dot_idx = 0
  blk_idx = disk.size - 1

  while dot_idx < blk_idx
    dot_idx += 1 while disk[dot_idx] != "."
    blk_idx -= 1 while disk[blk_idx] == "."

    if dot_idx < blk_idx
      disk[dot_idx] = disk[blk_idx]
      disk[blk_idx] = "."
    end
  end

  checksum = 0

  disk.each_with_index do |blk, i|
    return checksum if blk == "."

    checksum += i * blk
  end

  raise "wtf"
end

def solve_part_2 input
  disk, empty_blocks, file_blocks = expand_disk(input)

  file_blocks.reverse.each do |file_block|
    empty_block_idx = empty_blocks.index { it.size >= file_block.size && it.pos < file_block.pos }

    if empty_block_idx
      empty_block = empty_blocks[empty_block_idx]

      empty_block_range = empty_block.pos...(empty_block.pos + file_block.size)
      file_block_range = file_block.pos...(file_block.pos + file_block.size)

      # swap blocks
      disk[empty_block_range] = disk[file_block_range]
      disk[file_block_range] = ["."] * file_block.size

      if empty_block.size == file_block.size
        empty_blocks.delete_at(empty_block_idx)
      else
        empty_block.size -= file_block.size
        empty_block.pos += file_block.size
      end
    end
  end

  checksum = 0

  disk.each_with_index do |blk, i|
    checksum += i * blk if blk != "."
  end

  checksum
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
    assert_equal(2858, solve_part_2(File.read("test_input")))
  end

  def test_part_2_real_input
    assert_equal(6408966547049, solve_part_2(File.read("input")))
  end
end
