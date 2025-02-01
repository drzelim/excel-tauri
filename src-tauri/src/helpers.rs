use crate::file_io::{read_files, save_grouped_employees, add_text_to_filename};
use crate::models::Employee;
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use std::collections::HashMap;

use std::path::Path;
use std::path::PathBuf;

use calamine::Data;

pub fn merge_duplicates(employees: &Vec<Employee>) -> Vec<Employee> {
    let mut aggregated: HashMap<String, (f32, String, NaiveDateTime, String)> = HashMap::new();

    for (i, employee) in employees.iter().enumerate() {
        let entry = aggregated.entry(employee.name.clone()).or_insert((
            0.0,
            employee.task_name.clone(),
            employee.date.clone(),
            String::from(""),
        ));
        entry.0 += employee.duration;

        if i != 0 {
            entry.3 = format!(
                "{} \n{} - {} ч.",
                entry.3, employee.description, employee.duration
            );
        } else {
            entry.3 = format!("{} - {} ч.", employee.description, employee.duration);
        }
    }

    aggregated
        .into_iter()
        .map(
            |(name, (duration, task_name, date, description))| Employee {
                name,
                duration,
                task_name,
                date,
                description,
            },
        )
        .collect()
}

pub fn get_grouped_tasks(
    employees: &Vec<Employee>,
) -> HashMap<String, HashMap<String, Vec<Employee>>> {
    let mut grouped_tasks: HashMap<String, HashMap<String, Vec<Employee>>> = HashMap::new();

    for employee in employees.iter() {
        let entry = grouped_tasks
            .entry(employee.task_name.to_string())
            .or_insert(HashMap::new());

        let year_month = format!("{:02}-{:04}", employee.date.month(), employee.date.year());

        let inner_entry = entry.entry(year_month).or_insert(Vec::new());

        inner_entry.push(employee.clone());
    }

    grouped_tasks
}

pub fn extract_date_from_row(cell: &Data) -> Option<NaiveDateTime> {
    match cell {
        Data::String(s) => NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok(),
        Data::DateTime(date) => Some(excel_days_to_naive_datetime(date.as_f64())),
        _ => None,
    }
}

fn excel_days_to_naive_datetime(excel_days: f64) -> NaiveDateTime {
    // Начальная дата (30 декабря 1899 года)
    let base_date = NaiveDate::from_ymd_opt(1899, 12, 30)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let days = excel_days.floor() as i64;
    let fractional_day = excel_days.fract();

    let seconds_in_day = 24 * 60 * 60;
    let seconds = (fractional_day * seconds_in_day as f64).round() as u32;

    base_date
        .checked_add_signed(chrono::Duration::days(days))
        .unwrap()
        .checked_add_signed(chrono::Duration::seconds(seconds as i64))
        .unwrap()
}

pub fn naive_datetime_to_excel_days(naive_datetime: NaiveDateTime) -> f64 {
    let base_date = NaiveDate::from_ymd_opt(1899, 12, 30)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let duration = naive_datetime - base_date;

    let days = duration.num_days() as f64;

    let seconds_in_day = 24.0 * 60.0 * 60.0;
    let fractional_day = duration.num_seconds() as f64 % seconds_in_day / seconds_in_day;

    days + fractional_day
}

pub fn create_report(path: &PathBuf) -> String {
    let employees = read_files(path.as_path()).unwrap();
    let file_name = path.to_str().unwrap().split_once("input").unwrap().1;

    let output_path_str = format!("{}{}", "output", file_name);
    let output_path = Path::new(&output_path_str);

    let mut grouped_tasks = get_grouped_tasks(&employees);

    for values in grouped_tasks.values_mut() {
        for inner_value in values.values_mut() {
            *inner_value = merge_duplicates(inner_value);
        }
    }

    let title = Vec::from([
        String::from("Задача"),
        String::from("Имя"),
        String::from("Затраченное время"),
        String::from("Дата"),
        String::from("Комментарии"),
    ]);

    save_grouped_employees(&title, &grouped_tasks, Path::new(&output_path)).unwrap();
    output_path_str
}

pub fn create_report_with_path(path: &PathBuf) -> String {
    let data = read_files(path.as_path());
    let error_message = String::from("Incorrect file");

    let employees = match data {
        Ok(data_employees) => data_employees,
        _ => {
            return error_message;
        }
    };

    if employees.len() == 0 {
        return error_message;
    }
    
    let output_path_string = add_text_to_filename(path, "-output");
    let output_path = Path::new(&output_path_string);

    let mut grouped_tasks = get_grouped_tasks(&employees);

    for values in grouped_tasks.values_mut() {
        for inner_value in values.values_mut() {
            *inner_value = merge_duplicates(inner_value);
        }
    }

    let title = Vec::from([
        String::from("Задача"),
        String::from("Имя"),
        String::from("Затраченное время"),
        String::from("Дата"),
        String::from("Комментарии"),
    ]);

    let result = save_grouped_employees(&title, &grouped_tasks, Path::new(&output_path));

    match result {
        Ok(_) => output_path_string,
        Err(e) => e.to_string()
    }
}
