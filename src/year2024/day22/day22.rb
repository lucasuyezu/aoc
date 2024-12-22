#!/usr/bin/env ruby

Price = Struct.new(:price, :digit, :delta)
Sequence = Struct.new(:price_list, :cache)

def new_secret(secret)
  secret ^= (secret * 64)
  secret %= 16777216

  secret ^= (secret / 32)
  secret %= 16777216

  secret ^= (secret * 2048)
  secret %= 16777216
end

def build_sequences(input, build_cache=false)
  lines = input.lines
  lines.map(&:to_i).each_with_index.map do |secret, i|
    puts "Building sequence #{i+1}/#{lines.size}"
    prev = Price.new(secret, secret % 10)
    sequence = Sequence.new([prev], Hash.new)

    1.upto(2000) do |n|
      new_secret = new_secret(prev.price)
      cur = Price.new(new_secret, new_secret % 10, new_secret % 10 - prev.digit)
      sequence.price_list << cur

      if build_cache && n >= 4
        range = n-3..n
        cache_key = sequence.price_list[range].map(&:delta).join(",")
        value = sequence.price_list[n].digit
        sequence.cache[cache_key] = value unless sequence.cache.key? cache_key
      end

      prev = cur
    end

    sequence
  end
end

def solve_part_1(input)
  build_sequences(input)
    .map { it.price_list.last.price }
    .sum
end

def solve_part_2(input)
  sequences = build_sequences(input, true)

  max_result = 0
  cache = Set.new
  cache_hits = 0

  sequences.each_with_index do |outer_sequence, i|
    puts "Testing sequence #{i + 1}/#{sequences.size} (#{cache_hits} cache hits)"
    outer_list = outer_sequence.price_list
    1.upto(outer_list.size - 4) do |n|
      outer_range = n...n+4
      outer_seq = outer_list[outer_range].map(&:delta).join(",")

      if cache.include? outer_seq
        cache_hits += 1
      else
        total_price = sequences.each_with_index.filter_map do |inner_sequence, j|
          inner_sequence.cache[outer_seq]
        end.sum

        cache << outer_seq
        max_result = total_price if total_price > max_result
      end
    end
  end

  max_result
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample
    assert_equal(37327623, solve_part_1(File.read("test_input")))
  end

  def test_part_1_real
    assert_equal(14082561342, solve_part_1(File.read("input")))
  end

  def test_part_2_sample
    assert_equal(23, solve_part_2(File.read("test_input_p2")))
  end

  def test_part_2_real
    assert_equal(1568, solve_part_2(File.read("input")))
  end
end
