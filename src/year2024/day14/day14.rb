#!/usr/bin/env ruby

Point = Struct.new(:x, :y)
Robot = Struct.new(:pos, :vel)

def robots(input)
  input
    .scan(/p=(\d+),(\d+) v=(-?\d+),(-?\d+)/)
    .map { Robot.new(Point.new(it[0].to_i, it[1].to_i), Point.new(it[2].to_i, it[3].to_i)) }
end

def move_robot(robot, steps, max_x, max_y)
  new_x = robot.pos.x + robot.vel.x * steps
  new_x %= max_x

  new_y = robot.pos.y + robot.vel.y * steps
  new_y %= max_y

  robot.pos = Point.new(new_x, new_y)
end

def quadrant(robot, max_x, max_y)
  if robot.pos.x <= (max_x / 2) - 1 && robot.pos.y <= (max_y / 2) - 1
    1
  elsif robot.pos.x > (max_x / 2) && robot.pos.y <= (max_y / 2) - 1
    2
  elsif robot.pos.x <= (max_x / 2) - 1 && robot.pos.y > (max_y / 2)
    3
  elsif robot.pos.x > (max_x / 2) && robot.pos.y > (max_y / 2)
    4
  end
end

def solve_part_1(input, max_x, max_y)
  robots(input)
    .each { move_robot(it, 100, max_x, max_y) }
    .map { quadrant(it, max_x, max_y) }
    .compact
    .tally
    .values
    .reduce(&:*)
end

PAH = /###############################/

def solve_part_2(input, max_x, max_y)
  robots = robots(input)

  line = "." * max_x
  line << "\n"

  template = []
  max_y.times { template << line.dup }

  (1..).each do |n|
    frame = Marshal.load(Marshal.dump(template))

    robots
      .each { move_robot(it, 1, max_x, max_y) }
      .each { frame[it.pos.y][it.pos.x] = "#" }

    return n if frame.any? { it =~ PAH }
  end
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample_input
    assert_equal(12, solve_part_1(File.read("test_input"), 11, 7))
  end

  def test_part_1_real_input
    assert_equal(228410028, solve_part_1(File.read("input"), 101, 103))
  end

  def test_part_2_real_input
    assert_equal(8258, solve_part_2(File.read("input"), 101, 103))
  end
end

