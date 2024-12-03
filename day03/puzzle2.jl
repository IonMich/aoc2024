function sum_muls()
    remove_pattern = r"don't\(\).*?(?:do\(\)|$)"
    mul_pattern = r"mul\((\d+),(\d+)\)"
    total = 0
    open("./inputs/input.txt", "r") do file
        full_text = join(readlines(file), "")
        full_text = replace(full_text, remove_pattern => "-")
        all_matches = eachmatch(mul_pattern, full_text)
        for match in all_matches
            num1 = parse(Int, match.captures[1])
            num2 = parse(Int, match.captures[2])
            total += num1 * num2
        end
    end
    return total
end

println(sum_muls())