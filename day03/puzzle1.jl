function sum_muls()
    pattern = r"mul\(\d+,\d+\)"
    total = 0
    open("./inputs/input.txt", "r") do file
        for line in eachline(file)
            line_total = 0
            all_matches = eachmatch(pattern, line)
            # strip the "mul(" and ")" from the match
            # and split the numbers by ","
            for match in all_matches
                nums = split(match.match, ",")
                num1 = parse(Int, nums[1][5:end])
                num2 = parse(Int, nums[2][1:end-1])
                line_total += num1 * num2
            end
            total += line_total
        end
    end
    return total
end

println(sum_muls())