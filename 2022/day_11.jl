include("common.jl")

abstract type Operand end
struct Old <: Operand end
struct Const <: Operand
    value::Int
end

value(::Old, old::Int) = old
value(operand::Const, ::Int) = operand.value

function Base.parse(::Type{Operand}, str::AbstractString)
    return str == "old" ? Old() : Const(parse(Int, str))
end

struct Expression
    left::Operand
    right::Operand
    op::Function
end

function value(expression::Expression, old::Int)
    return expression.op(value(expression.left, old), value(expression.right, old))
end

function Base.parse(::Type{Expression}, str::AbstractString)
    parts = split(str, ' ')
    length(parts) == 3 || throw(ArgumentError("Invalid expression: $str"))
    op = getfield(Main, Symbol(parts[2]))
    return Expression(parse(Operand, parts[1]), parse(Operand, parts[3]), op)
end

struct Monkey
    id::Int
    items::Vector{Int}
    expression::Expression
    test_divisor::Int
    id_target_true::Int
    id_target_false::Int
end

function Base.parse(::Type{Monkey}, str::AbstractString)
    lines = filter(!isempty, map(strip, split(str, '\n')))
    length(lines) == 6 || throw(ArgumentError("Can't read monkey from: $str"))

    id = parse(Int, only(match(r"Monkey (\d+):", lines[1]).captures))
    items_str = only(match(r"Starting items: (.+)", lines[2]).captures)
    items = map(x -> parse(Int, x), split(items_str, ", "))
    expression_str = only(match(r"Operation: new = (.+)", lines[3]).captures)
    expression = parse(Expression, expression_str)
    test_divisor = parse(Int, only(match(r"Test: divisible by (\d+)", lines[4]).captures))
    id_target_true = parse(
        Int, only(match(r"If true: throw to monkey (\d+)", lines[5]).captures)
    )
    @assert id_target_true != id
    id_target_false = parse(
        Int, only(match(r"If false: throw to monkey (\d+)", lines[6]).captures)
    )
    @assert id_target_false != id
    return Monkey(id, items, expression, test_divisor, id_target_true, id_target_false)
end

function Base.parse(::Type{Vector{Monkey}}, str::AbstractString)
    return map(x -> parse(Monkey, x), split(str, "\n\n"))
end

struct Transfer
    id_target::Int
    worry::Int
end

function compute_new_worry(monkey::Monkey, old_worry::Int, ::Part1)
    return div(value(monkey.expression, old_worry), 3)
end

function compute_new_worry(monkey::Monkey, old_worry::Int, ::Part2)
    return value(monkey.expression, old_worry)
end

function compute_transfers(monkey::Monkey, part::Part)::Vector{Transfer}
    return map(monkey.items) do old_worry
        new_worry = compute_new_worry(monkey, old_worry, part)
        test_passed = (new_worry % monkey.test_divisor) == 0
        target = test_passed ? monkey.id_target_true : monkey.id_target_false
        Transfer(target, new_worry)
    end
end

struct Problem
    monkeys::Vector{Monkey}
    num_inspections::Vector{Int64}
    Problem(monkeys::Vector{Monkey}) = new(monkeys, zeros(Int64, length(monkeys)))
end

function Base.parse(::Type{Problem}, str::AbstractString)
    monkeys = parse(Vector{Monkey}, str)
    return Problem(monkeys)
end

function round!(problem::Problem, part::Part)
    # We can work in a reduced set of integers that allows numbers up to the product of all
    # the divisors (each of which happen to be prime in the example & my input).
    # This means that when we wrap around to zero, we do not change the result of any of the
    # remainder checks that we do.
    #
    # Without doing this, we quickly run into integer overflow in part 2.
    factor = mapreduce(*, problem.monkeys) do monkey
        monkey.test_divisor
    end

    for monkey in problem.monkeys
        problem.num_inspections[monkey.id + 1] += length(monkey.items)
        transfers = compute_transfers(monkey, part)
        # Remove all items from the current monkey.
        empty!(monkey.items)
        @assert length(monkey.items) == 0
        # Move each item to the appropriate target monkey.
        for transfer in transfers
            target_monkey = problem.monkeys[transfer.id_target + 1]
            push!(target_monkey.items, transfer.worry % factor)
        end
    end
end

function get_ans(input::AbstractString)
    problem = parse(Problem, input)
    for _ in 1:20
        round!(problem, Part1())
    end
    return reduce(*, partialsort(problem.num_inspections, 1:2; rev=true))
end

example = """
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"""

@assert get_ans(example) == 10605
get_ans(load_input(11))

# Part 2

function get_ans2(input::AbstractString)
    problem = parse(Problem, input)
    for i in 1:10000
        round!(problem, Part2())
    end
    return reduce(*, partialsort(problem.num_inspections, 1:2; rev=true))
end

@assert get_ans2(example) == 2713310158
get_ans2(load_input(11))
