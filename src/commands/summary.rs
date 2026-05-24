use crate::{
    cli::SummaryArgs,
    db::entries::{EntryFilter, list_entries},
    errors::CliError,
};
use chrono::{Datelike, Duration, Local};
use serde::Serialize;
use std::collections::HashMap;
use tabled::{Table, Tabled};

#[derive(Serialize)]
struct TaskSummary {
    task_key: String,
    task_description: String,
    impact: String,
    tags: Vec<String>,
    projects: Vec<String>,
    activity_types: Vec<String>,
    messages: Vec<String>,
    entry_count: usize,
    #[serde(skip)]
    total_time_minutes: u32,
}

#[derive(Tabled)]
struct SummaryRow {
    task: String,
    #[tabled(rename = "Description")]
    desc: String,
    impact: String,
    #[tabled(rename = "Logs")]
    count: usize,
}

/// Converts HHMM integer (e.g., 1330) to minutes from midnight (810)
fn to_minutes(hhmm: u32) -> u32 {
    let hours = hhmm / 100;
    let minutes = hhmm % 100;
    (hours * 60) + minutes
}

/// Categorizes task effort based on relative time spent
fn get_impact_level(minutes: u32, total_minutes: u32) -> String {
    if total_minutes == 0 {
        return "Minor".to_string();
    }
    let ratio = minutes as f32 / total_minutes as f32;
    match ratio {
        r if r > 0.3 => "Primary Focus".to_string(),
        r if r > 0.1 => "Significant Effort".to_string(),
        _ => "Routine Maintenance".to_string(),
    }
}

fn get_week_range(start: Option<String>, end: Option<String>) -> (Option<String>, Option<String>) {
    if start.is_some() || end.is_some() {
        return (start, end);
    }

    let now = Local::now().date_naive();

    // Saturday=6, Sunday=0, ... Friday=5
    // Logic:
    // If today is Saturday (6), back 0.
    // If today is Sunday (0), back 1.
    // ... If today is Friday (5), back 6.
    let wd = now.weekday().num_days_from_sunday() as i64;
    let days_back = if wd >= 6 { wd - 6 } else { wd + 1 };

    let start_of_week = now - Duration::days(days_back);
    let end_of_week = start_of_week + Duration::days(6);

    (
        Some(start_of_week.format("%Y-%m-%d").to_string()),
        Some(end_of_week.format("%Y-%m-%d").to_string()),
    )
}

pub fn handle(args: SummaryArgs) -> Result<(), CliError> {
    let (start, end) = get_week_range(args.start_date, args.end_date);
    let filter = EntryFilter {
        start_date: start.as_deref(),
        end_date: end.as_deref(),
        ..Default::default()
    };

    let entries = list_entries(&filter)?;
    let mut summary_map: HashMap<String, TaskSummary> = HashMap::new();

    for entry in entries {
        let entry_ref = summary_map
            .entry(entry.task_key.clone())
            .or_insert(TaskSummary {
                task_key: entry.task_key.clone(),
                task_description: entry.task_description.clone(),
                impact: String::new(),
                total_time_minutes: 0,
                tags: Vec::new(),
                projects: Vec::new(),
                activity_types: Vec::new(),
                messages: Vec::new(),
                entry_count: 0,
            });

        entry_ref.entry_count += 1;

        if let (Some(start), Some(end)) = (entry.start_time, entry.end_time) {
            entry_ref.total_time_minutes += to_minutes(end).saturating_sub(to_minutes(start));
        }

        entry_ref.tags.extend(entry.tags);
        entry_ref.projects.extend(entry.projects);
        entry_ref.activity_types.extend(entry.activity_types);

        if let Some(msg) = entry.message {
            entry_ref.messages.push(msg);
        }
    }

    // Calculate period total and assign impact
    let total_period_minutes: u32 = summary_map.values().map(|s| s.total_time_minutes).sum();

    for summary in summary_map.values_mut() {
        summary.tags.sort();
        summary.tags.dedup();
        summary.projects.sort();
        summary.projects.dedup();
        summary.activity_types.sort();
        summary.activity_types.dedup();
        summary.impact = get_impact_level(summary.total_time_minutes, total_period_minutes);
    }

    let summary_data: Vec<TaskSummary> = summary_map.into_values().collect();

    if args.json {
        let payload = serde_json::json!({
            "meta": {
                "report_type": "weekly_summary",
                "period": {
                    "start": start,
                    "end": end
                },
                "instructions": "You are a professional project manager. Analyze the provided task logs. Use the 'impact' field to prioritize your discussion—focus on tasks marked 'Primary Focus' and 'Significant Effort'. Use the 'messages' array to build specific, accomplishment-oriented bullet points for each task. Provide a 'why it matters' description for the work, identify trends in the activity types, and highlight key accomplishments."
            },
            "data": summary_data
        });
        println!("{}", serde_json::to_string_pretty(&payload)?);
    } else {
        let s = start.as_deref().unwrap_or("Start");
        let e = end.as_deref().unwrap_or("End");
        println!("\nSummary for period: {} to {}", s, e);
        if summary_data.is_empty() {
            println!("No logs found for the selected period.");
            return Ok(());
        }

        let table_rows: Vec<SummaryRow> = summary_data
            .iter()
            .map(|s| SummaryRow {
                task: s.task_key.clone(),
                desc: s.task_description.clone(),
                impact: s.impact.clone(),
                count: s.entry_count,
            })
            .collect();

        println!("{}", Table::new(table_rows));
    }

    Ok(())
}
