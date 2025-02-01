use crate::{
    helpers::{extract_date_from_row, naive_datetime_to_excel_days},
    models::Employee,
};
use std::{fs, path::PathBuf};
use std::{collections::HashMap, path::Path};

use calamine::{open_workbook, DataType, Reader, Xlsx, XlsxError};
use rust_xlsxwriter::*;

pub fn read_files(path: &Path) -> Result<Vec<Employee>, XlsxError> {
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");

    let sheet_names = workbook.sheet_names();
    let first_sheet = sheet_names.get(0).unwrap();

    match workbook.worksheet_range(first_sheet) {
        Ok(range) => {
            let mut employees: Vec<Employee> = Vec::new();

            if range.get_size().1 < 9 {
                return Ok(employees);
            }

            for row in range.rows() {
                let first_row_cell = row[0].get_string().unwrap_or("").to_string();
    
                if first_row_cell == "Проект" {
                    break;
                }
    
                if first_row_cell == "" || row.is_empty() {
                    continue;
                }
    
                // let name = row[6].get_string().unwrap_or("").to_string();
                // let duration = row[7].get_float().unwrap_or(0.0) as f32;
                // let task_name = row[3].get_string().unwrap_or("").to_string();
                // let date = extract_date_from_row(&row[5].clone()).unwrap_or_default();
                // let description = row[8].get_string().unwrap_or("").to_string();

                // let name = match row.get(6) {
                //     Some(s) => s.get_string().unwrap_or("").to_string(),
                //     None => {
                //         "".to_string()  // Можно вернуть пустую строку или обработать иначе
                //     }
                // };

                let name = row.get(6).unwrap().get_string().unwrap_or_default().to_string();
                let duration = row.get(7).unwrap().get_float().unwrap_or(0.0) as f32;
                let task_name = row.get(3).unwrap().get_string().unwrap_or("").to_string();
                let date = extract_date_from_row(&row.get(5).unwrap().clone()).unwrap_or_default();
                let description = row.get(8).unwrap().get_string().unwrap_or("").to_string();
    
                employees.push(Employee {
                    name,
                    duration,
                    task_name,
                    date,
                    description,
                });
            }
    
            return Ok(employees);
        },
        Err(e) => Err(e)
    }
}

fn wtire_employees(worksheet: &mut Worksheet, employees: &Vec<Employee>, offset: u32) {
    let date_format = Format::new().set_num_format("yyyy-mm.dd hh:MM:ss");
    let duration_format = Format::new().set_num_format("0.00");
    let text_format = Format::new().set_text_wrap();

    for (i, employe) in employees.iter().enumerate() {
        let inx = i as u32;

        worksheet
            .write(inx + offset + 1, 0, &employe.task_name)
            .unwrap();
        worksheet.write(inx + offset + 1, 1, &employe.name).unwrap();
        worksheet
            .write_with_format(inx + offset + 1, 2, employe.duration, &duration_format)
            .unwrap();
        worksheet
            .write_with_format(
                inx + offset + 1,
                3,
                naive_datetime_to_excel_days(employe.date),
                &date_format,
            )
            .unwrap();
        worksheet
            .write_with_format(
                inx + offset + 1,
                4,
                employe.description.clone(),
                &text_format,
            )
            .unwrap();
    }
}

pub fn save_grouped_employees(
    titles: &Vec<String>,
    tasks: &HashMap<String, HashMap<String, Vec<Employee>>>,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook = Workbook::new();
    let bold_format = Format::new().set_bold();
    let task_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_font_color(Color::White)
        .set_background_color(Color::Blue)
        .set_align(FormatAlign::Center)
        .set_font_size(13)
        .set_text_wrap();

    let mounth_format = Format::new()
        .set_background_color(Color::RGB(0xb8cad4))
        .set_align(FormatAlign::Center)
        .set_font_size(13)
        .set_bold();

    let worksheet = workbook.add_worksheet();

    worksheet.set_column_width(0, 30)?;
    worksheet.set_column_width(1, 22)?;
    worksheet.set_column_width(2, 14)?;
    worksheet.set_column_width(3, 20)?;
    worksheet.set_column_width(4, 70)?;

    for i in 0..titles.len() {
        let data = titles.get(i);
        worksheet.write_with_format(0, i as u16, data, &bold_format)?;
    }

    let mut count: u32 = 1;

    let mut keys: Vec<String> = tasks.keys().cloned().collect();
    keys.sort();

    for key in keys {
        count += 1;
        worksheet.merge_range(count, 0, count, 4, &key, &task_format)?;
        // worksheet.write_with_format(count, 0, &key, &task_format)?;

        for value in tasks[&key].iter() {
            count += 1;
            worksheet.merge_range(count, 0, count, 4, value.0, &mounth_format)?;
            // worksheet.write_with_format(count, 0, value.0, &mounth_format)?;

            wtire_employees(worksheet, &value.1, count);
            count += value.1.len() as u32 + 1;
        }
    }
    workbook.save(path)?;

    Ok(())
}

// pub fn save_grouped_employees(titles: &[String; 4], tasks: &HashMap<String, Vec<Employee>>, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
//     let mut workbook = Workbook::new();
//     let bold_format = Format::new().set_bold();
//     let merge_format = Format::new()
//         .set_border(FormatBorder::Thin)
//         .set_align(FormatAlign::Center);

//     let worksheet = workbook.add_worksheet();

//     worksheet.set_column_width(0, 22)?;
//     worksheet.set_column_width(1, 14)?;
//     worksheet.set_column_width(2, 20)?;
//     worksheet.set_column_width(3, 70)?;

//     for i in 0..titles.len() {
//         let data = titles.get(i);
//         worksheet.write_with_format(0, i as u16, data, &bold_format)?;
//     }

//     let mut count: u32 = 1;
//     for (key, value) in tasks {
//         count +=1;
//         worksheet.merge_range(count, 0, count, 3, key, &merge_format)?;

//         wtire_employees(worksheet, &value, count);
//         count += value.len() as u32 + 1;
//     }
//     workbook.save(path)?;

//     Ok(())
// }

pub fn read_dir(path: &Path) -> std::io::Result<Vec<PathBuf>> {
    let files: Vec<_> = fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .map(|entry| entry.path())
        .collect();

    Ok(files)
}

pub fn add_text_to_filename(path: &PathBuf, text: &str) -> String {

    if let Some(parent) = path.parent() {
        if let Some(stem) = path.file_stem() {
            if let Some(extension) = path.extension() {
                let new_filename = format!("{}{}.{}", stem.to_string_lossy(), text, extension.to_string_lossy());
                let new_path = parent.join(new_filename);
                return new_path.to_string_lossy().to_string();
            }
        }
    }

    path.to_string_lossy().to_string()
}
