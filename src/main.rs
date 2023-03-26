//Imports the date/time module from the chrono crate
use chrono::{DateTime, Local};
//Imports the CSV reader module from the csv crate 
use csv::ReaderBuilder; 
//Imports the file I/O module from the standard library
use std::fs::File; 
//Imports the buffered reader module from the standard library
use std::io::{BufRead, BufReader}; 
//Imports Path buffer
use std::path::PathBuf;
//Imports the native dialog box
use native_dialog::{FileDialog, DialogType};

//Defines a structure to represent a calendar event
struct CalendarEvent {
    title: String,
    start_time: DateTime<local>,
    end_time: DateTime<local>,
    location: String,
}

fn main() {
    //Startup
    println!("Hello, world!");
    //Opens the dialog to import a Calendar event
    let dialog = FileDialog::new()
        .set_location("/home/user") //Start locatio
        .set_title("Select a file") //Window title
        //.add_filter("Text Files", &["txt"]) Filter
        .set_confirm_style(DialogType::YesNo) //
        .show_open_single_file(); //File selection

        match dialog {
        //Opens the CSV file with additional error handler
        Ok(file) => {
            //Creates a buffered reader for the file
            let reader = BufReader::new(File::open(file).unwrap().expect("Failed to open the CSV file"));
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
                events.push(CalendarEvent { title, start_time, end_time, location });
        }

        //Iterates over each event in the events vector
        for event in events {
            //Prints out the event in the format HH:MM dd.mm.YYYY
            println!("{} - {} to {} at {}", event.title, event.start_time.format("at %H:%M on the %d.%m.%Y"), event.end_time.format("%H:%M%d.%m%Y"), event.location);
        }

        //Creates a folder where the events are saved
        fs::create_dir_all("calendar_events").unwrap();

        //Writes each event to a seperate file in the folder
        for (i, event) in events.iter().enumerate() {
            //Writes the filename where the X is a number that increases for every event
            let filename = format!("calendar_event_{}.txt", i);
            let path = PathBuf::from(format!("calendar_events/{}", filename));
            let mut file = File::create(path).unwrap();
            writeln!(file, "{} - {} to {} at {}", event.title, event.start_time.format("%H:%M %d.%m.%Y"), event.end_time.format("%H:%M %d.%m.%Y"), event.location).unwrap();
        }
    },
    Err(e) => println!("Error: {:?}", e),
    }
}


