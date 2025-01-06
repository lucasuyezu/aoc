#!/usr/bin/env ruby

Point = Struct.new(:x, :y)

def solve(input)
  input.chomp.split("\n").map do |cable|
    point = Point.new(0, 0)

    dists = {}
    cur_dist = 0

    cable.split(",").each do |ins|
      ins[1..].to_i.times.map do |n|
        point = point.dup
        cur_dist += 1

        case ins[0]
        when "R"
          point.x += 1
        when "U"
          point.y += 1
        when "L"
          point.x -= 1
        when "D"
          point.y -= 1
        end

        dists[point] = cur_dist
      end
    end

    dists
  end
end

def solve_part_1(input)
  solve(input).map(&:keys).reduce(&:&).map { it.x.abs + it.y.abs }.min
end

def solve_part_2(input)
  dists = solve(input)
  dists.map(&:keys).reduce(&:&).map { dists[0][it] + dists[1][it] }.min
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample
    assert_equal(6, solve_part_1("R8,U5,L5,D3\nU7,R6,D4,L4"))
    assert_equal(159, solve_part_1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"))
    assert_equal(135, solve_part_1("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"))
  end

  def test_part_1_real
    assert_equal(1674, solve_part_1(File.read("input")))
  end

  def test_part_2_sample
    assert_equal(30, solve_part_2("R8,U5,L5,D3\nU7,R6,D4,L4"))
    assert_equal(610, solve_part_2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"))
    assert_equal(410, solve_part_2("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"))
  end

  def test_part_2_real
    assert_equal(14012, solve_part_2(File.read("input")))
  end
end
