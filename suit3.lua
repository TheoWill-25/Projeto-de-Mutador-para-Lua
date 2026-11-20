local Operators = require("codigo3")

local passed = 0
local failed = 0

local function test(name, callback)
    local ok, err = pcall(callback)

    if ok then
        passed = passed + 1
        print("[PASS] " .. name)
    else
        failed = failed + 1
        print("[FAIL] " .. name)
        print("       " .. tostring(err))
    end
end

local function assert_equal(actual, expected)
    assert(
        actual == expected,
        "Expected " .. tostring(expected) .. ", got " .. tostring(actual)
    )
end

test("Floor division rounds down for positive values", function()
    assert_equal(Operators.floor_divide(7, 2), 3)
end)

test("Floor division rounds down for negative values", function()
    assert_equal(Operators.floor_divide(-7, 2), -4)
end)

test("Concatenation preserves the order of the strings", function()
    assert_equal(Operators.concatenate("Lua", " 5.3"), "Lua 5.3")
end)

test("Length counts the bytes in a string", function()
    assert_equal(Operators.length("Lua"), 3)
end)

test("Length counts elements in a consecutive list", function()
    assert_equal(Operators.length({10, 20}), 2)
    assert_equal(Operators.length({}), 0)
end)

test("Unary minus reverses the sign", function()
    assert_equal(Operators.negate(7), -7)
    assert_equal(Operators.negate(-7), 7)
end)

test("Bitwise AND keeps the common bits", function()
    assert_equal(Operators.bit_and(6, 3), 2)
end)

test("Bitwise OR keeps bits set in either operand", function()
    assert_equal(Operators.bit_or(6, 3), 7)
end)

test("Bitwise XOR keeps bits that differ", function()
    assert_equal(Operators.bit_xor(6, 3), 5)
end)

test("Left shift moves bits to the left", function()
    assert_equal(Operators.shift_left(6, 3), 48)
end)

test("Right shift moves bits to the right", function()
    assert_equal(Operators.shift_right(6, 3), 0)
    assert_equal(Operators.shift_right(13, 2), 3)
end)

print("\n==============================")
print("Tests passed: " .. passed)
print("Tests failed: " .. failed)
print("==============================")

if failed > 0 then
    os.exit(1)
end
