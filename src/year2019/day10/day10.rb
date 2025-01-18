#!/usr/bin/env ruby
# frozen_string_literal: true

Point = Struct.new(:x, :y)
Asteroid = Struct.new(:pos, :dist, :angle)

def solve_part_1 input
  grid = input.lines.map(&:chomp)
  grid_y_range = 0...grid.size
  grid_x_range = 0...grid[0].size

  asteroids = Set.new

  grid.each_with_index do |line, y|
    line.chars.each_with_index do |c, x|
      next unless c == "#"

      asteroids << Point.new(x, y)
    end
  end

  asteroids.map do |asteroid|
    asteroids.map { Math.atan2(it.y - asteroid.y, it.x - asteroid.x) }.uniq.size
  end.max
end

def solve_part_2 input, x, y
  grid = input.lines.map(&:chomp)
  grid_y_range = 0...grid.size
  grid_x_range = 0...grid[0].size

  origin = Point.new(x, y)
  asteroids = Set.new

  grid.each_with_index do |line, y|
    line.chars.each_with_index do |c, x|
      next unless c == "#"

      next if x == origin.x && y == origin.y

      dist = Math.sqrt((x - origin.x)**2 + (y - origin.y)**2)

      angle_radians = Math.atan2(y - origin.y, x - origin.x) * -1
      angle_radians = 0 if angle_radians.zero?

      angle_degrees = angle_radians * (180.0 / Math::PI)
      angle_degrees += 360 if angle_degrees < 0

      angle_degrees = (angle_degrees - 90) % 360

      asteroids << Asteroid.new(Point.new(x, y), dist, angle_degrees)
    end
  end

  asteroids_by_angle = asteroids
    .sort_by(&:dist)
    .group_by(&:angle)

  count = 1
  wave = asteroids_by_angle.keys.map { asteroids_by_angle[it].shift }.compact

  while wave.any?

    clockwise_wave = wave.sort_by(&:angle).reverse
    clockwise_wave.unshift clockwise_wave.pop

    clockwise_wave.each do
      return it.pos.x * 100 + it.pos.y if count == 200

      count += 1
    end
    wave = asteroids_by_angle.keys.map { x[it].shift }.compact
  end

  raise "There are fewer than 200 asteroids"
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_debug
    assert_equal(2, solve_part_1("##########"))
    assert_equal(2, solve_part_1("#...######"))
    assert_equal(3, solve_part_1("#\n#\n#\n#\n#\n"))
    assert_equal(3, solve_part_1("#\n#\n.\n.\n#\n"))

    assert_equal(3, solve_part_1("#....\n.#...\n..#..\n...#.\n....#\n"))
    assert_equal(3, solve_part_1("#....\n.#...\n.....\n...#.\n....#\n"))
    assert_equal(3, solve_part_1("....#\n...#.\n..#..\n.#...\n#....\n"))
    assert_equal(3, solve_part_1("....#\n...#.\n.....\n.#...\n#....\n"))

    # assert_equal(7, solve_part_1(File.read("test_debug")))
  end

  def test_part_1_sample_input
    assert_equal(8, solve_part_1(File.read("test_input_1")))
    assert_equal(33, solve_part_1(File.read("test_input_2")))
    assert_equal(35, solve_part_1(File.read("test_input_3")))
    assert_equal(41, solve_part_1(File.read("test_input_4")))
    assert_equal(210, solve_part_1(File.read("test_input_5")))
  end

  def test_part_1_real_input
    assert_equal(299, solve_part_1(File.read("input")))
  end

  def test_part_2_sample_input
    assert_equal(802, solve_part_2(File.read("test_input_5"), 11, 13))
  end

  def test_part_2_real_input
    assert_equal(1419, solve_part_2(File.read("input"), 26, 29))
  end
end
