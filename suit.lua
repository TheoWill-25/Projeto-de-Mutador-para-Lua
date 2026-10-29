local Evaluator = require("codigo")
local Status = Evaluator.Status

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

local function contains(list, value)
    for _, item in ipairs(list) do
        if item == value then
            return true
        end
    end
    return false
end

local function evaluate(age, gpa, attendance, courses, discipline)
    return Evaluator.evaluate_scholarship(
        age, gpa, attendance, courses, discipline
    )
end

local function assert_status(result, expected)
    assert(
        result.status == expected,
        "Expected " .. expected .. ", got " .. tostring(result.status)
    )
end

local function assert_reason(result, reason)
    assert(
        contains(result.reasons, reason),
        "Expected reason not found: " .. reason
    )
end

-- Caso ideal: aprovado
test("Ideal applicant is approved", function()
    local result = evaluate(18, 8, 90, true, false)
    assert_status(result, Status.APPROVED)
    assert_reason(result,
        "Applicant meets all scholarship requirements.")
end)

-- Idade: limites
test("Age 15 is rejected", function()
    assert_status(evaluate(15, 8, 90, true, false),
        Status.REJECTED)
end)

test("Age 16 requires manual review", function()
    assert_status(evaluate(16, 8, 90, true, false),
        Status.MANUAL_REVIEW)
end)

test("Age 17 requires manual review", function()
    assert_status(evaluate(17, 8, 90, true, false),
        Status.MANUAL_REVIEW)
end)

test("Age 18 is approved", function()
    assert_status(evaluate(18, 8, 90, true, false),
        Status.APPROVED)
end)

-- GPA: limites
test("GPA 0 is rejected", function()
    assert_status(evaluate(18, 0, 90, true, false),
        Status.REJECTED)
end)

test("GPA 5.99 is rejected", function()
    assert_status(evaluate(18, 5.99, 90, true, false),
        Status.REJECTED)
end)

test("GPA 6 requires manual review", function()
    assert_status(evaluate(18, 6, 90, true, false),
        Status.MANUAL_REVIEW)
end)

test("GPA 6.99 requires manual review", function()
    assert_status(evaluate(18, 6.99, 90, true, false),
        Status.MANUAL_REVIEW)
end)

test("GPA 7 is approved", function()
    assert_status(evaluate(18, 7, 90, true, false),
        Status.APPROVED)
end)

test("GPA 10 is approved", function()
    assert_status(evaluate(18, 10, 90, true, false),
        Status.APPROVED)
end)

-- Frequência: limites
test("Attendance 0 is rejected", function()
    assert_status(evaluate(18, 8, 0, true, false),
        Status.REJECTED)
end)

test("Attendance 74.99 is rejected", function()
    assert_status(evaluate(18, 8, 74.99, true, false),
        Status.REJECTED)
end)

test("Attendance 75 requires manual review", function()
    assert_status(evaluate(18, 8, 75, true, false),
        Status.MANUAL_REVIEW)
end)

test("Attendance 79.99 requires manual review", function()
    assert_status(evaluate(18, 8, 79.99, true, false),
        Status.MANUAL_REVIEW)
end)

test("Attendance 80 is approved", function()
    assert_status(evaluate(18, 8, 80, true, false),
        Status.APPROVED)
end)

test("Attendance 100 is approved", function()
    assert_status(evaluate(18, 8, 100, true, false),
        Status.APPROVED)
end)

-- Outros requisitos
test("Missing required courses cause rejection", function()
    assert_status(evaluate(18, 8, 90, false, false),
        Status.REJECTED)
end)

test("Disciplinary record causes rejection", function()
    assert_status(evaluate(18, 8, 90, true, true),
        Status.REJECTED)
end)

-- Múltiplas razões
test("Multiple rejection reasons are returned", function()
    local result = evaluate(15, 5.5, 90, true, false)

    assert_status(result, Status.REJECTED)
    assert_reason(result,
        "Applicant is younger than the minimum age.")
    assert_reason(result,
        "GPA is below the minimum required.")
end)

test("Multiple manual review reasons are returned", function()
    local result = evaluate(17, 6.5, 78, true, false)

    assert_status(result, Status.MANUAL_REVIEW)
    assert_reason(result,
        "Applicant is under 18 and requires manual review.")
    assert_reason(result,
        "GPA is in the manual review range.")
    assert_reason(result,
        "Attendance rate is in the manual review range.")
end)

-- Valores inválidos
test("GPA -0.01 raises an error", function()
    local ok, err = pcall(evaluate, 18, -0.01, 90, true, false)

    assert(not ok, "Expected an error")
    assert(tostring(err):find(
        "GPA must be between 0 and 10.", 1, true))
end)

test("GPA 10.01 raises an error", function()
    local ok, err = pcall(evaluate, 18, 10.01, 90, true, false)

    assert(not ok, "Expected an error")
    assert(tostring(err):find(
        "GPA must be between 0 and 10.", 1, true))
end)

test("Attendance -0.01 raises an error", function()
    local ok, err = pcall(evaluate, 18, 8, -0.01, true, false)

    assert(not ok, "Expected an error")
    assert(tostring(err):find(
        "Attendance rate must be between 0 and 100.", 1, true))
end)

test("Attendance 100.01 raises an error", function()
    local ok, err = pcall(evaluate, 18, 8, 100.01, true, false)

    assert(not ok, "Expected an error")
    assert(tostring(err):find(
        "Attendance rate must be between 0 and 100.", 1, true))
end)

-- Resultado final
print("\n==============================")
print("Tests passed: " .. passed)
print("Tests failed: " .. failed)
print("==============================")

if failed > 0 then
    os.exit(1)
end