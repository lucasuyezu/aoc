#!/usr/bin/env ruby

def solve_part_1(input)
  registers, program = input.split("\n\n")
  # puts registers.inspect
  a = registers.lines[0].split(": ")[1].to_i
  b = registers.lines[1].split(": ")[1].to_i
  c = registers.lines[2].split(": ")[1].to_i

  # puts a.inspect
  # puts b.inspect
  # puts c.inspect

  instructions = program.strip.split(": ")[1].split(",").map(&:to_i)

  # puts instructions.inspect

  ip = 0
  output = []

  while ip >= 0 && ip < instructions.size
    literal_op = instructions[ip + 1]
    combo_op   = combo_op(instructions, ip + 1, a, b, c)

    puts
    puts "Register A: #{a}"
    puts "Register B: #{b}"
    puts "Register C: #{c}"
    puts "Instructions: #{instructions.inspect}"
    puts "ip: instructions[#{ip}] = #{instructions[ip]}"
    puts "literal_op: #{literal_op}"
    puts "combo_op:   #{combo_op}"

    case instructions[ip]
    when 0 #adv
      a = dv(a, combo_op)
    when 1 #bxl
      result = b ^ literal_op
      puts "#{result} = #{b} ^ #{literal_op}"
      b = result
    when 2 #bst
      b = combo_op % 8
    when 4 #bxc
      b = b ^ c
    when 5 #out
      output << combo_op % 8
    when 6 #bdv
      b = dv(a, combo_op)
    when 7 #cdv
      c = dv(a, combo_op)
    end

    ip = instructions[ip] == 3 && a != 0 ? literal_op : ip + 2
  end

  output.join(",")
end

def combo_op(instructions, ip, a, b, c)
  op = instructions[ip]

  result = case op
  when 0..3
    op
  when 4
    a
  when 5
    b
  when 6
    c
  else
    raise "WTF"
  end


  #  puts "combo_op: instructions[#{ip}]=#{op} produces #{result}"
  result
end

def dv(a, combo_op)
  denominator = 2 ** combo_op
  result = a / denominator
  puts "#{result} = #{a} / #{denominator}"
  result
end

def solve_part_2(input)
  raise NotImplementedError
end

require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample
    assert_equal("4,6,3,5,6,3,5,2,1,0", solve_part_1(File.read("test_input")))
  end

  def test_part_1_real
    assert_equal("7,0,7,3,4,1,3,0,1", solve_part_1(File.read("input")))
  end

  def test_part_2_sample
    assert_equal(64, solve_part_2(File.read("test_input")))
  end

  def test_part_2_real
    assert_equal(1453087, solve_part_2(File.read("input")))
  end
end
