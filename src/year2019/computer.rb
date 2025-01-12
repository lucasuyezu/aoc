#!/usr/bin/env ruby

class OpCode1
  def self.execute(computer, param_modes)
    param1 = computer.get_read_param(1, param_modes)
    param2 = computer.get_read_param(2, param_modes)
    param3 = computer.get_write_param(3, param_modes)

    computer.memory[param3] = param1 + param2

    [computer.ip + 4]
  end
end

class OpCode2
  def self.execute(computer, param_modes)
    param1 = computer.get_read_param(1, param_modes)
    param2 = computer.get_read_param(2, param_modes)
    param3 = computer.get_write_param(3, param_modes)

    computer.memory[param3] = param1 * param2

    [computer.ip + 4]
 end
end

class OpCode3
  def self.execute(computer, param_modes)
    dest_addr = computer.memory[computer.ip+1]
    dest_addr += computer.relative_base if param_modes[0] == 2

    computer.memory[dest_addr] = computer.input_queue.pop

    [computer.ip + 2]
  end
end

class OpCode4
  def self.execute(computer, param_modes)
    computer.output_queue << computer.get_read_param(1, param_modes)

    [computer.ip + 2]
  end
end

class OpCode5
  def self.execute(computer, param_modes)
    param1 = computer.get_read_param(1, param_modes)
    param2 = computer.get_read_param(2, param_modes)

    param1.zero? ? [computer.ip + 3] : [param2]
  end
end

class OpCode6
  def self.execute(computer, param_modes)
    param1 = computer.get_read_param(1, param_modes)
    param2 = computer.get_read_param(2, param_modes)

    param1.zero? ? [param2] : [computer.ip + 3]
  end
end

class OpCode7
  def self.execute(computer, param_modes)
    param1 = computer.get_read_param(1, param_modes)
    param2 = computer.get_read_param(2, param_modes)
    param3 = computer.get_write_param(3, param_modes)

    computer.memory[param3] = param1 < param2 ? 1 : 0

    [computer.ip + 4]
  end
end

class OpCode8
  def self.execute(computer, param_modes)
    param1 = computer.get_read_param(1, param_modes)
    param2 = computer.get_read_param(2, param_modes)
    param3 = computer.get_write_param(3, param_modes)

    computer.memory[param3] = param1 == param2 ? 1 : 0

    [computer.ip + 4]
  end
end

class OpCode9
  def self.execute(computer, param_modes)
    computer.relative_base += computer.get_read_param(1, param_modes)

    [computer.ip + 2]
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
    9 => OpCode9,
  }

  attr_accessor :input_queue, :output_queue, :memory, :ip, :relative_base

  def initialize(memory:, input_queue: Thread::Queue.new, output_queue: Thread::Queue.new)
    @memory = memory
    @ip = 0
    @input_queue = input_queue
    @output_queue = output_queue
    @relative_base = 0
    @thread = nil
  end

  def execute_sync
    execute_async.wait
    self
  end

  def execute_async
    @thread = Thread.new do
      while memory[ip] != 99
        ins = memory[ip]
        opcode = ins % 100

        param_modes = []

        param_modes << (ins / 100) % 10
        param_modes << (ins / 1000) % 10
        param_modes << (ins / 10000) % 10

        result = OPCODES[opcode].execute(self, param_modes)

        @ip = result[0]
      end
    end
    self
  end

  def wait
    @thread.join
  end

  def get_read_param(idx, param_modes)
    case param_modes[idx - 1]
    when 0
      memory[memory[ip+idx]]
    when 1
      memory[ip+idx]
    when 2
      memory[relative_base + memory[ip+idx]]
    else
      raise "Invalid param mode"
    end
  end

  def get_write_param(idx, param_modes)
    dest_addr = memory[ip+idx]
    dest_addr += param_modes[2] == 2 ? relative_base : 0
  end
end

