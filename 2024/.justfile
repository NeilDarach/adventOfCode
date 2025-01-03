_default:
    @just --list

# Use `just work day-01 part1` to work on the specific binary for a specific day's problems
work day part:
    bacon {{day}}/src libtest -- {{part}}

lint day:
    cargo clippy -p {{day}}
create day:
    cargo generate --path ./daily-template --name {{day}}
    just get-input {{day}}

# You can find SESSION by using Chrome tools:
# 1) Go to https://adventofcode.com/2024/day/1/input
# 2) right-click -> inspect -> click the "Application" tab.
# 3) Refresh
# 5) Click https://adventofcode.com under "Cookies"
# 6) Grab the value for session. Fill it into your .env file
# 
# example .env:
#
# ```
# SESSION=PASTE_COOKIE_VALUE_HERE
# ```
#
# get the input for a day's puzzle
get-input day:
    ./scripts/get-aoc-input.rs --day {{day}} --current-working-directory {{justfile_directory()}}
