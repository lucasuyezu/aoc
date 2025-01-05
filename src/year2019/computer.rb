#!/usr/bin/env ruby

class Computer
  def self.execute!(memory, ip=0)
    while memory[ip] != 99
      memory[memory[ip+3]] = case memory[ip]
                             when 1
                               memory[memory[ip+1]] + memory[memory[ip+2]]
                             when 2
                               memory[memory[ip+1]] * memory[memory[ip+2]]
                             else
                               raise "Invalid opcode #{memory[ip].inspect} at address #{ip}"
                             end

      ip += 4
    end

    memory
  end
end
