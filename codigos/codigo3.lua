local Operators = {}

function Operators.floor_divide(value, divisor)
    return value // divisor
end

function Operators.concatenate(first, second)
    return first .. second
end

function Operators.length(value)
    return #value
end

function Operators.negate(value)
    return -value
end

function Operators.bit_and(left, right)
    return left & right
end

function Operators.bit_or(left, right)
    return left | right
end

function Operators.bit_xor(left, right)
    return left ~ right
end

function Operators.shift_left(value, bits)
    return value << bits
end

function Operators.shift_right(value, bits)
    return value >> bits
end

function Operators.bit_not(value)
    return ~value
end

return Operators
