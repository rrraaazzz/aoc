defmodule Pass do
  @keys [:byr, :iyr, :eyr, :hgt, :hcl, :ecl, :pid, :cid]
  defstruct @keys

  @rex %{
    byr: ~r/\d{4}/,
    iyr: ~r/\d{4}/,
    eyr: ~r/\d{4}/,
    hgt: ~r/(\d+)(in|cm)/,
    hcl: ~r/#[0-9a-f]{6}/,
    ecl: ~r/amb|blu|brn|gry|grn|hzl|oth/,
    pid: ~r/\d{9}/
  }

  @valid %{
    byr: &Pass.is_valid_byr/1,
    iyr: &Pass.is_valid_iyr/1,
    eyr: &Pass.is_valid_eyr/1,
    hgt: &Pass.is_valid_hgt/1
  }

  def is_valid_byr([y_str]) do
    y = String.to_integer(y_str)
    y >= 1920 && y <= 2002
  end

  def is_valid_iyr([y_str]) do
    y = String.to_integer(y_str)
    y >= 2010 && y <= 2020
  end

  def is_valid_eyr([y_str]) do
    y = String.to_integer(y_str)
    y >= 2020 && y <= 2030
  end

  def is_valid_hgt([_, h_str, unit]) do
    h = String.to_integer(h_str)
    (unit == "in" && h >= 59 && h <= 76) || (unit == "cm" && h >= 150 && h <= 193)
  end

  def is_valid_part1(pass) do
    nils = Enum.filter(@keys, &(Map.get(pass, &1, nil) == nil))
    valid = nils == [] || nils == [:cid]
    valid
  end

  def is_valid_part2(pass) do
    is_valid_part1(pass) && are_keys_valid(pass)
  end

  def is_valid_kv({k, v}) do
    is_ok = fn (r, valid) ->
      matches = Regex.run(r, v)
      matches && valid.(matches)
    end

    case {@rex[k], @valid[k]} do
      {nil, nil} -> true
      {r, nil} -> Regex.match?(r, v)
      {r, valid} -> is_ok.(r, valid)
    end
  end

  def are_keys_valid(pass) do
    Map.to_list(pass) |> Enum.map(&is_valid_kv/1) |> Enum.all?()
  end
end

defmodule Day4 do
  def line_fields(line) do
    Map.new(
      String.split(line)
      |> Enum.map(&String.split(&1, ":"))
      |> Enum.map(fn [k, v] -> {String.to_atom(k), v} end)
    )
  end

  def transform_line(line, pass) do
    fields = line_fields(line)

    if map_size(fields) > 0 do
      new_pass = Map.merge(pass, fields)
      {[], new_pass}
    else
      {[pass], %Pass{}}
    end
  end

  def passports(lines) do
    lines |> Stream.transform(%Pass{}, &transform_line/2)
  end

  def part1(lines) do
    passes = passports(lines)
    count_all = Enum.count(passes)
    count_valid = passes |> Enum.filter(&Pass.is_valid_part1/1) |> Enum.count()
    IO.puts("Have valid items: #{count_valid}/#{count_all}")
  end

  def part2(lines) do
    passes = passports(lines)
    count_all = Enum.count(passes)
    count_valid = passes |> Enum.filter(&Pass.is_valid_part2/1) |> Enum.count()
    IO.puts("Have valid items: #{count_valid}/#{count_all}")
  end
end
