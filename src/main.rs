//Imports the date/time module from the chrono crate
use chrono::{DateTime, Local};
//Imports the CSV reader module from the csv crate 
use csv::ReaderBuilder; 
//Imports the file I/O module from the standard library
use std::fs::File; 
//Imports the buffered reader module from the standard library
use std::io::{BufRead, BufReader}; 

//Defines a struct to represent a calendar event
struct CalendarEvent {
    title: String,
    start_time: DateTime<local>,
    end_time: DateTime<local>,
}

fn main() {
    //Startup
    println!("Hello, world!");
    //Opens the CSV file with additional error handler
    let file = File::open("calendar.csv").unwrap().expect("Failed to open the CSV file");
    //Creates a buffered reader for the file
    let reader = BufReader::new(file);
    //Creates a CSV reader for the buffered reader
    let mut csv_reader = ReaderBuilder::new().has_headers(false).from_reader(reader);

    //Creates a vector to hold the calendar events
    let mut events = Vec::new();

    //Iterates over each record in the CSV file
    for result in csv_reader.records() {
        //Error handler
        let record = result.expect("Failed to load, file might be poorly formatted");
        //Unwraps the records
        let record = result.unwrap();
        //Gets the title from the first column
        let title = record[0].to_string();
        //Gets the start and end time from the second and third columns and parses them into DateTime objects
        let start_time = DateTime::parse_from_rfc3339(&record[1]).unwrap().with_timezone(&local);
        let end_time = DateTime::parse_from_rfc3339(&record[2]).unwrap().with_timezone(&local);
        //Adds a new CalendarEvent struct to the events vector
        events.push(CalendarEvent { title, start_time, end_time });
    }

    //Iterates over each event in the events vector
    for event in events {
        //Prints out the event in the format HH:MM dd.mm.YYYY
        println!("{} - {} to {}", event.title, event.start_time.format("%H:%M %d.%m.%Y"), event.end_time.format("%H:%M%d.%m%Y"));
    }
}


