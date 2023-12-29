def solve_part_1 input_lines
  input_lines
    .map { _1.scan(/\d/) }
    .map { _1.first.to_i * 10  + _1.last.to_i }
    .sum
end

def solve_part_2 input_lines
  re = /1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine/
  tokens = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight",
    "nine",
  ]

  digit_to_i = {
    "1" => 1,
    "one" => 1,
    "2" => 2,
    "two" => 2,
    "3" => 3,
    "three" => 3,
    "4" => 4,
    "four" => 4,
    "5" => 5,
    "five" => 5,
    "6" => 6,
    "six" => 6,
    "7" => 7,
    "seven" => 7,
    "8" => 8,
    "eight" => 8,
    "9" => 9,
    "nine" => 9
  }

  input_lines
    .map do |line|
      first_match = line.match(re)[0]
      second_match = tokens.max_by { |token| line.rindex(token) || -1 }
      digit_to_i[first_match] * 10 + digit_to_i[second_match]
    end
    .sum
end


require 'minitest/autorun'

class Day1Test < Minitest::Test
  def test_part_1_sample_input
    assert_equal(142, solve_part_1(File.readlines("test_input_part_1")))
  end

  def test_part_1_real_input
    assert_equal(56_506, solve_part_1(File.readlines("input")))
  end

  def test_part_2_sample_input
    assert_equal(292, solve_part_2(File.readlines("test_input_part_2")))
  end

  def test_part_2_real_input
    assert_equal(56_017, solve_part_2(File.readlines("input")))
  end
end



