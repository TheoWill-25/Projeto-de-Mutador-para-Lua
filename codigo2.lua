local Evaluator = {}

local x = true
function Evaluator.check_age(age)

    if age < 18 then
        return "REJECTED"
    else
        return "APPROVED"
    end

end

return Evaluator