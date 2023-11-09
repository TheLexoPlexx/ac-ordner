use chrono::{Datelike, Month, NaiveDate};
use native_dialog::FileDialog;
use std::io::{stdin, stdout, Write};

fn main() {
    let mut s = String::new();
    print!("Jahreszahl eingeben: ");
    let _ = stdout().flush();

    stdin().read_line(&mut s).expect("Keine valide Eingabe.");

    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    //parse s into a year
    let year: i32 = s.parse().unwrap();

    //ask for a directory
    let path = FileDialog::new()
        .reset_location()
        .show_open_single_dir()
        .unwrap();

    let path = match path {
        Some(path) => path,
        None => return,
    };

    println!("Ausgewählter Ordner: {}", path.display());

    //create a folder for the year
    let year_folder = path.join("Packlisten ".to_string() + &year.to_string());

    if year_folder.exists() {
        println!("Bestehender Ordner wird erweitert...");
    } else {
        std::fs::create_dir(&year_folder).expect("Failed to create year folder.");
    }

    let mut date = NaiveDate::from_ymd_opt(year, 1, 1).expect("Failed to parse date.");

    //get all weekdays for the given year
    let mut weekdays = Vec::new();

    while date.year() == year {
        if date.weekday() != chrono::Weekday::Sat && date.weekday() != chrono::Weekday::Sun {
            weekdays.push(date);
        }
        date = date.succ_opt().expect("Failed to parse succeeding date.");
    }

    for item in weekdays {
        let month_folder = year_folder.join(format!(
            "{} - {}",
            item.month(),
            translate_months(Month::try_from(u8::try_from(item.month()).unwrap()).unwrap())
        ));
        if !month_folder.exists() {
            std::fs::create_dir(&month_folder).expect("Failed to create month folder.");
        }

        let week = item.iso_week().week();
        let mut leading_zero = "";
        if week < 10 {
            leading_zero = "0";
        }
        let week_folder = month_folder.join(format!("KW{leading_zero}{}", item.iso_week().week()));
        if !week_folder.exists() {
            std::fs::create_dir(&week_folder).expect("Failed to create week folder.");
        }

        let day_folder = week_folder.join(item.format("%d.%m.%Y").to_string());
        if !day_folder.exists() {
            std::fs::create_dir(&day_folder).expect("Failed to create day folder.");
        }
    }

    //create a folder for each month in the format "1 - Januar"
    // let mut month_folders = Vec::new();
    // for i in 1..13 {
    //     let month_folder = year_folder.join(format!("{} - {}", i, weekdays[i - 1].month()));
    //     std::fs::create_dir(&month_folder).expect("Failed to create month folder.");
    //     month_folders.push(month_folder);
    // }

    println!("> Fertig.")

    // Jahr eingeben
    // Packlisten-Ordner auswählen
    // generieren, auf doppelte prüfen
    // fertig
}

fn translate_months(month: chrono::Month) -> String {
    let m = match month {
        chrono::Month::January => "Januar",
        chrono::Month::February => "Februar",
        chrono::Month::March => "März",
        chrono::Month::April => "April",
        chrono::Month::May => "Mai",
        chrono::Month::June => "Juni",
        chrono::Month::July => "Juli",
        chrono::Month::August => "August",
        chrono::Month::September => "September",
        chrono::Month::October => "Oktober",
        chrono::Month::November => "November",
        chrono::Month::December => "Dezember",
    };
    m.to_string()
}
