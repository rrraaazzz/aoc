defmodule Day1 do
  def numbers() do
    for line <- IO.stream(:stdio, :line), into: MapSet.new() do
      line |> String.trim() |> String.to_integer()
    end
  end

  def part1() do
    set = numbers()

    for n <- set, MapSet.member?(set, 2020 - n) do
      IO.puts(:stdio, n * (2020 - n))
    end
  end

  def part2() do
    set = numbers()
    for a <- set,
        b <- set,
        b != a,
        MapSet.member?(set, 2020 - a - b) do
      IO.inspect(a * b * (2020 - a - b))
    end
  end
end

if "--part2" in System.argv() do
  Day1.part2()
else
  Day1.part1()
end
