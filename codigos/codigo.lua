local ScholarshipEligibilityEvaluator = {}

local Status = {
    APPROVED = "APPROVED",
    REJECTED = "REJECTED",
    MANUAL_REVIEW = "MANUAL_REVIEW"
}

ScholarshipEligibilityEvaluator.Status = Status

function ScholarshipEligibilityEvaluator.evaluate_scholarship(
    age,
    gpa,
    attendance_rate,
    has_required_courses,
    disciplinary_record
)
    --start mut
    assert(gpa >= 0 and gpa <= 10,
        "GPA must be between 0 and 10.")

    assert(attendance_rate >= 0 and attendance_rate <= 100,
        "Attendance rate must be between 0 and 100.")

    local reasons = {}
    local rejected = false
    local manual_review = false

    if age < 16 then
        rejected = true
        table.insert(reasons,
            "Applicant is younger than the minimum age.")
    elseif age < 18 then
        manual_review = true
        table.insert(reasons,
            "Applicant is under 18 and requires manual review.")
    end

    if gpa < 6 then
        rejected = true
        table.insert(reasons,
            "GPA is below the minimum required.")
    elseif gpa < 7 then
        manual_review = true
        table.insert(reasons,
            "GPA is in the manual review range.")
    end

    if attendance_rate < 75 then
        rejected = true
        table.insert(reasons,
            "Attendance rate is below the minimum required.")
    elseif attendance_rate < 80 then
        manual_review = true
        table.insert(reasons,
            "Attendance rate is in the manual review range.")
    end

    if not has_required_courses then
        rejected = true
        table.insert(reasons,
            "Required courses have not been completed.")
    end

    if disciplinary_record then
        rejected = true
        table.insert(reasons,
            "Applicant has a disciplinary record.")
    end

    local status

    if rejected then
        status = Status.REJECTED
    elseif manual_review then
        status = Status.MANUAL_REVIEW
    else
        status = Status.APPROVED
        table.insert(reasons,
            "Applicant meets all scholarship requirements.")
    end

    --end mut
    return {
        status = status,
        reasons = reasons
    }
end

return ScholarshipEligibilityEvaluator