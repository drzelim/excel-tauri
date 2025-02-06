use std::collections::HashMap;

use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Employee {
    pub name: String,
    pub duration: f32,
    pub task_name: String,
    pub date: NaiveDateTime,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub duration: f32,
    pub employees: HashMap<String, Vec<Employee>>
}

impl Task {
    pub fn sum_duration(&mut self) {
        self.duration = 0.0;
        for inner_value in self.employees.values() {
            for employee in inner_value {
                self.duration += employee.duration;
            }
        }
    }
}
