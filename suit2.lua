local Evaluator = require("codigo2")

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
        "Expected " .. expected .. ", got " .. actual
    )
end

test("Age 15 is rejected", function()
    assert_equal(
        Evaluator.check_age(15),
        "REJECTED"
    )
end)

test("Age 25 is approved", function()
    assert_equal(
        Evaluator.check_age(25),
        "APPROVED"
    )
end)

print("\n==============================")
print("Tests passed: " .. passed)
print("Tests failed: " .. failed)
print("==============================")

if failed > 0 then
    os.exit(1)
end