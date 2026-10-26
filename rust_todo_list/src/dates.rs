use chrono::{Datelike, Days, Local, NaiveDate};

// Turns what the user typed into an optional due date.
// "none" → Ok(None), "today" / "tmr" / "30-10" / "30-10-2026" → Ok(Some(date)),
// anything invalid or in the past → Err.
pub fn parse_due_date(value: &str) -> Result<Option<NaiveDate>, String> {
    let today = Local::now().date_naive();

    let date = match value.to_lowercase().as_str() {
        "none" => return Ok(None),
        "today" => today,
        "tmr" | "tomorrow" => today
            .checked_add_days(Days::new(1))
            .ok_or_else(|| "Date out of range".to_string())?,
        _ => parse_date_text(value, today)?,
    };

    if date < today {
        return Err(format!(
            "Due date {} is in the past",
            date.format("%d-%m-%Y")
        ));
    }

    Ok(Some(date))
}

// Parses "30-10-2026" (dd-mm-yyyy) or "30-10" (dd-mm, year filled in).
// Returns a plain NaiveDate; "none" etc. were handled by the caller.
fn parse_date_text(value: &str, today: NaiveDate) -> Result<NaiveDate, String> {
    // Attempt 1: a full date, dd-mm-yyyy.
    NaiveDate::parse_from_str(value, "%d-%m-%Y")
        // Attempt 2: only runs if attempt 1 failed.
        // `|_|` ignores attempt 1's error, because we're trying something else.
        .or_else(|_| {
            // Add the current year to the end: "30-10" → "30-10-2026",
            // then parse it with the same pattern as attempt 1.
            // The ? works inside a closure too: if this parse fails,
            // the closure returns that error immediately.
            let this_year =
                NaiveDate::parse_from_str(&format!("{value}-{}", today.year()), "%d-%m-%Y")?;

            // Already passed this year? The user almost certainly means next year,
            // e.g. typing "01-01" in October means next January.
            if this_year < today {
                NaiveDate::parse_from_str(&format!("{value}-{}", today.year() + 1), "%d-%m-%Y")
            } else {
                Ok(this_year)
            }
        })
        // If both attempts failed, replace chrono's error with a helpful message.
        .map_err(|_| format!("Invalid date '{value}'. Use dd-mm-yyyy, dd-mm, today, tmr or none"))
}
