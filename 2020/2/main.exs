defmodule Day2 do
  def parse_line(line) do
    matches = Regex.run(~r/^(\d+)-(\d+) ([a-z]): ([a-z]*)/, line)

    case matches do
      nil -> nil
      [_, l, h, char, pass] ->
        [
          %{
            low: String.to_integer(l),
            high: String.to_integer(h),
            char: char,
            pass: pass
          }
        ]
    end
  end

  def passwords() do
    IO.stream(:stdio, :line) |> Stream.flat_map(&parse_line/1)
  end

  def is_valid_pass(%{ low: low, high: high, char: char, pass: pass }) do
    count = String.graphemes(pass) |> Enum.count(&(&1 == char))
    count >= low && count <= high
  end

  def part1() do
    count = Enum.count(passwords(), &(is_valid_pass(&1)))
    IO.puts(:stdio, "Valid items: #{count}")
  end

  def xor(a, b) do
    (a && !b) || (b && !a)
  end

  def is_valid_pass2(%{ low: low, high: high, char: char, pass: pass }) do
    xor(String.at(pass, low - 1) == char, String.at(pass, high - 1) == char)
  end

  def part2() do
    count = Enum.count(passwords(), &(is_valid_pass2(&1)))
    IO.puts(:stdio, "Valid items: #{count}")
  end
end

if "--part2" in System.argv() do
  Day2.part2()
else
  Day2.part1()
end
