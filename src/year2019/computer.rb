#!/usr/bin/env ruby

class OpCode1
  def self.execute(memory, ip, param_modes, input_queue, output_queue)
    param1 = param_modes[0] == 0 ? memory[memory[ip+1]] : memory[ip+1]
    param2 = param_modes[1] == 0 ? memory[memory[ip+2]] : memory[ip+2]

    memory[memory[ip+3]] = param1 + param2

    [ip + 4]
  end
end

class OpCode2
  def self.execute(memory, ip, param_modes, input_queue, output_queue)
    param1 = param_modes[0] == 0 ? memory[memory[ip+1]] : memory[ip+1]
    param2 = param_modes[1] == 0 ? memory[memory[ip+2]] : memory[ip+2]

    memory[memory[ip+3]] = param1 * param2

    [ip + 4]
  end
end

class OpCode3
  def self.execute(memory, ip, param_modes, input_queue, output_queue)
    memory[memory[ip+1]] = input_queue.pop

    [ip + 2]
  end
end

class OpCode4
  def self.execute(memory, ip, param_modes, input_queue, output_queue)
    param = param_modes[0] == 0 ? memory[memory[ip+1]] : memory[ip+1]

    output_queue << param

    [ip + 2]
  end
end

class OpCode5
  def self.execute(memory, ip, param_modes, input_queue, output_queue)
    param1 = param_modes[0] == 0 ? memory[memory[ip+1]] : memory[ip+1]
    param2 = param_modes[1] == 0 ? memory[memory[ip+2]] : memory[ip+2]

    param1.zero? ? [ip + 3] : [param2]
  end
end

class OpCode6
  def self.execute(memory, ip, param_modes, input_queue, output_queue)
    param1 = param_modes[0] == 0 ? memory[memory[ip+1]] : memory[ip+1]
    param2 = param_modes[1] == 0 ? memory[memory[ip+2]] : memory[ip+2]

    param1.zero? ? [param2] : [ip + 3]
  end
end

class OpCode7
  def self.execute(memory, ip, param_modes, input_queue, output_queue)
    param1 = param_modes[0] == 0 ? memory[memory[ip+1]] : memory[ip+1]
    param2 = param_modes[1] == 0 ? memory[memory[ip+2]] : memory[ip+2]

    memory[memory[ip+3]] = param1 < param2 ? 1 : 0

    [ip + 4]
  end
end

class OpCode8
  def self.execute(memory, ip, param_modes, input_queue, output_queue)
    param1 = param_modes[0] == 0 ? memory[memory[ip+1]] : memory[ip+1]
    param2 = param_modes[1] == 0 ? memory[memory[ip+2]] : memory[ip+2]

    memory[memory[ip+3]] = param1 == param2 ? 1 : 0

    [ip + 4]
  end
end


class Computer
  OPCODES = {
    1 => OpCode1,
    2 => OpCode2,
    3 => OpCode3,
    4 => OpCode4,
    5 => OpCode5,
    6 => OpCode6,
    7 => OpCode7,
    8 => OpCode8,
  }
  def self.execute!(memory, input_queue=nil, output_queue=nil, ip=0)
    Thread.new do
      while memory[ip] != 99
        ins = memory[ip]
        opcode = ins % 100

        param_modes = []

        param_modes << (ins / 100) % 10
        param_modes << (ins / 1000) % 10
        param_modes << (ins / 10000) % 10

        result = OPCODES[opcode].execute(memory, ip, param_modes, input_queue, output_queue)

        ip = result[0]
      end

      # input_queue.close  if input_queue
      # output_queue.close if output_queue
    end
  end
end

