extern crate flate2;
extern crate csv;
extern crate chrono;

use std::io;
use std::process;
use std::fs::File;
use flate2::read::GzDecoder;
use chrono::{DateTime, NaiveDateTime, Utc};
use image::{ImageBuffer, Rgb};

const PLACE_HISTORY_FILE: &'static str = "/mnt/LinuxDataSSD_B/2022_place_canvas_history.csv.gzip";

fn color_to_index(color: &str) -> u8 {
    match color {
        "#000000" => 0,
        "#00CCC0" => 1,
        "#94B3FF" => 2,
        "#6A5CFF" => 3,
        "#009EAA" => 4,
        "#E4ABFF" => 5,
        "#00756F" => 6,
        "#00A368" => 7,
        "#00CC78" => 8,
        "#2450A4" => 9,
        "#3690EA" => 10,
        "#493AC1" => 11,
        "#515252" => 12,
        "#51E9F4" => 13,
        "#6D001A" => 14,
        "#6D482F" => 15,
        "#7EED56" => 16,
        "#811E9F" => 17,
        "#898D90" => 18,
        "#9C6926" => 19,
        "#B44AC0" => 20,
        "#BE0039" => 21,
        "#D4D7D9" => 22,
        "#DE107F" => 23,
        "#FF3881" => 24,
        "#FF4500" => 25,
        "#FF99AA" => 26,
        "#FFA800" => 27,
        "#FFB470" => 28,
        "#FFD635" => 29,
        "#FFF8B8" => 30,
        "#FFFFFF" => 31,
        _ => panic!("Unrecognized {}", color)
    }
}

fn index_to_pixel(index: u8) -> Rgb<u8> {
    match index {
        0 => Rgb([0, 0, 0]),
        1 => Rgb([0, 204, 192]),
        2 => Rgb([148, 179, 255]),
        3 => Rgb([106, 92, 255]),
        4 => Rgb([0, 158, 170]),
        5 => Rgb([228, 171, 255]),
        6 => Rgb([0, 117, 111]),
        7 => Rgb([0, 163, 104]),
        8 => Rgb([0, 204, 120]),
        9 => Rgb([36, 80, 164]),
        10 => Rgb([54, 144, 234]),
        11 => Rgb([73, 58, 193]),
        12 => Rgb([81, 82, 82]),
        13 => Rgb([81, 233, 244]),
        14 => Rgb([109, 0, 26]),
        15 => Rgb([109, 72, 47]),
        16 => Rgb([126, 237, 86]),
        17 => Rgb([129, 30, 159]),
        18 => Rgb([137, 141, 144]),
        19 => Rgb([156, 105, 38]),
        20 => Rgb([180, 74, 192]),
        21 => Rgb([190, 0, 57]),
        22 => Rgb([212, 215, 217]),
        23 => Rgb([222, 16, 127]),
        24 => Rgb([255, 56, 129]),
        25 => Rgb([255, 69, 0]),
        26 => Rgb([255, 153, 170]),
        27 => Rgb([255, 168, 0]),
        28 => Rgb([255, 180, 112]),
        29 => Rgb([255, 214, 53]),
        30 => Rgb([255, 248, 184]),
        31 => Rgb([255, 255, 255]),
        _ => panic!("Unrecognized {}", index)
    }
}


fn update_color(
    x: u16, y: u16, timestamp: i64, color: &str,
    canvas: &mut Vec<u8>, canvas_timestamps: &mut Vec<i64>
    ) {
    let index: u32 = ((y as u32 * 2000) + x as u32);
    let previous_timestamp: i64 = canvas_timestamps[index as usize];

    if previous_timestamp > timestamp {
        return
    }

    canvas_timestamps[index as usize] = timestamp;

    canvas[index as usize] = color_to_index(color);
}


fn main() -> std::io::Result<()> {
    if std::env::args().len() < 3 {
        println!("Usage: <place_dataset.gzip> <timestamp>");
        println!("Example:");
        println!("  ./place-snapshot /foo/snapshot.gzip \"2022-04-04 18:00:00\"");
        std::process::exit(0);
    }

    let dataset_path: String = std::env::args().nth(1).unwrap();
    let cutoff: String = std::env::args().nth(2).unwrap();

    let file = File::open(dataset_path)?;
    let gz = GzDecoder::new(file);

    let mut canvas: Vec<u8> = vec![31; 2000*2000];
    let mut canvas_timestamps: Vec<i64> = vec![0; 2000*2000];

    let mut csv_reader = csv::Reader::from_reader(io::BufReader::new(gz));

    let mut total_pixels: u64 = 0;
    let mut processed_pixels: u64 = 0;

    for result in csv_reader.records() {
        total_pixels += 1;
        let record = result?;

        let timestamp: String = record.get(0).unwrap().parse().unwrap();

        if timestamp > cutoff {
            continue;
        }

        let timestamp_datetime = NaiveDateTime::parse_from_str(
            &timestamp, "%Y-%m-%d %H:%M:%S%.f UTC"
        ).unwrap();
        let timestamp_datetime: DateTime<Utc> = DateTime::from_utc(
            timestamp_datetime, Utc
        );
        let timestamp_unix = timestamp_datetime.timestamp();

        let user_id = record.get(1).unwrap();
        let color = record.get(2).unwrap();
        let coordinates = record.get(3).unwrap().split(",");
        let coordinates = coordinates.collect::<Vec<&str>>();

        if coordinates.len() == 4 {
            println!("Found moderator placement at {}", timestamp);
            let x1: u16 = coordinates.get(0).unwrap().parse().unwrap();
            let y1: u16 = coordinates.get(1).unwrap().parse().unwrap();
            let x2: u16 = coordinates.get(2).unwrap().parse().unwrap();
            let y2: u16 = coordinates.get(3).unwrap().parse().unwrap();

            for x_ in x1..x2+1 {
                for y_ in y1..y2+1 {
                    update_color(
                        x_, y_, timestamp_unix, color,
                        &mut canvas, &mut canvas_timestamps
                    );
                }
            }
        } else {
            let x: u16 = coordinates.get(0).unwrap().parse().unwrap();
            let y: u16 = coordinates.get(1).unwrap().parse().unwrap();

            update_color(
                x, y, timestamp_unix, color,
                &mut canvas, &mut canvas_timestamps
            );
        }

        processed_pixels += 1;

        if total_pixels % 1000000 == 0 {
            println!("{} pixels iterated, {} processed so far...", total_pixels, processed_pixels);
        }
    }

    println!("Total: {}", total_pixels);
    println!("Rendering image...");

    let mut img = ImageBuffer::new(2000, 2000);

    for x in 0..2000 {
        for y in 0..2000 {
            let index: u32 = ((y * 2000) + x);
            let color_index = canvas[index as usize];
            let pixel = index_to_pixel(color_index);
            img.put_pixel(x, y, pixel);
        }
    }

    img.save("place.png").unwrap();

    Ok(())
}
