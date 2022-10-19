use std::io;
use std::fs::File;
use flate2::read::GzDecoder;
use clap::Parser;
use image::{ImageBuffer, Rgb};
use indicatif::{ProgressBar, ProgressStyle};

type Canvas = Vec<u8>;

#[derive(Parser, Debug)]
#[clap(version)]
struct CliArgs {
    #[clap(short, long, help = "Path to the gzip compressed CSV file")]
    path: String,

    date: String
}

fn color_to_index(color: & [u8]) -> u8 {
    match color {
        b"#000000" => 0,
        b"#00CCC0" => 1,
        b"#94B3FF" => 2,
        b"#6A5CFF" => 3,
        b"#009EAA" => 4,
        b"#E4ABFF" => 5,
        b"#00756F" => 6,
        b"#00A368" => 7,
        b"#00CC78" => 8,
        b"#2450A4" => 9,
        b"#3690EA" => 10,
        b"#493AC1" => 11,
        b"#515252" => 12,
        b"#51E9F4" => 13,
        b"#6D001A" => 14,
        b"#6D482F" => 15,
        b"#7EED56" => 16,
        b"#811E9F" => 17,
        b"#898D90" => 18,
        b"#9C6926" => 19,
        b"#B44AC0" => 20,
        b"#BE0039" => 21,
        b"#D4D7D9" => 22,
        b"#DE107F" => 23,
        b"#FF3881" => 24,
        b"#FF4500" => 25,
        b"#FF99AA" => 26,
        b"#FFA800" => 27,
        b"#FFB470" => 28,
        b"#FFD635" => 29,
        b"#FFF8B8" => 30,
        b"#FFFFFF" => 31,
        _ => panic!("Unrecognized {}", String::from_utf8(color.to_vec()).unwrap())
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
    x: u16, y: u16, timestamp: i64, color: u8,
    canvas: &mut Canvas, canvas_timestamps: &mut Vec<i64>,
    canvas_touched: &mut Vec<bool>
    ) {
    let index: u32 = ((y as u32 * 2000) + x as u32);

    let previous_timestamp: i64 = canvas_timestamps[index as usize];

    if previous_timestamp > timestamp {
        return
    }

    canvas_timestamps[index as usize] = timestamp;
    canvas[index as usize] = color;
    canvas_touched[index as usize] = true;
}

fn str_to_timestamp(value: &str) -> i64 {
    let day: i64 = value[8..10].parse().unwrap();

    let hour: i64 = value[11..13].parse().unwrap();
    let minute: i64 = value[14..16].parse().unwrap();
    let second: i64 = value[17..19].parse().unwrap();

    let mut timestamp: i64 = second;
    timestamp += 60 * minute;
    timestamp += (60 * 60) * hour;
    timestamp += (24 * 60 * 60) * day;

    return timestamp;
}


fn main() -> std::io::Result<()> {
    let args = CliArgs::parse();
    if std::env::args().len() < 3 {
        println!("Usage: <place_dataset.gzip> <timestamp>");
        println!("Example:");
        println!("  ./place-snapshot /foo/snapshot.gzip \"2022-04-04 18:00:00\"");
        std::process::exit(0);
    }

    //let dataset_path: String = std::env::args().nth(1).unwrap();
    let dataset_path: String = args.path.to_string();
    let cutoff: String = args.date.to_string();

    let file = File::open(dataset_path)?;
    let gz = GzDecoder::new(file);

    let mut canvas: Canvas = vec![31; 2000*2000];
    let mut canvas_touched: Vec<bool> = vec![false; 2000*2000];
    let mut canvas_timestamps: Vec<i64> = vec![0; 2000*2000];
    canvas.shrink_to_fit();
    canvas_touched.shrink_to_fit();
    canvas_timestamps.shrink_to_fit();

    let mut csv_reader = csv::Reader::from_reader(io::BufReader::new(gz));

    let mut total_pixels: u64 = 0;
    let mut processed_pixels: u64 = 0;

    let bar = ProgressBar::new(160_353_104);
    bar.set_style(
        ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [ETA {eta_precise}] {bar:40.cyan/blue} {percent}% {pos:>7}/{len:7} {msg} {per_sec}")
        .progress_chars("##-")
    );

    for result in csv_reader.records() {
        total_pixels += 1;

        if total_pixels % 1000 == 0 {
            bar.set_position(total_pixels)
        }

        let record = result?;

        let timestamp: String = record.get(0).unwrap().parse().unwrap();

        if timestamp > cutoff {
            continue;
        }

        //let timestamp_datetime = NaiveDateTime::parse_from_str(
        //    &timestamp[0..18], "%Y-%m-%d %H:%M:%S"
        //).unwrap();
        let timestamp_unix = str_to_timestamp(&timestamp);
        //let timestamp_unix = timestamp_datetime.timestamp();

        let color: &[u8] = record.get(2).unwrap().as_bytes();
        let color: u8 = color_to_index(color);
        let coordinates = record.get(3).unwrap().split(",");
        let coordinates = coordinates.collect::<Vec<&str>>();

        match coordinates.len() {
            4 => {
                println!("Found moderator placement at {}", timestamp);
                let x1: u16 = coordinates.get(0).unwrap().parse().unwrap();
                let y1: u16 = coordinates.get(1).unwrap().parse().unwrap();
                let x2: u16 = coordinates.get(2).unwrap().parse().unwrap();
                let y2: u16 = coordinates.get(3).unwrap().parse().unwrap();

                for x_ in x1..x2+1 {
                    for y_ in y1..y2+1 {
                        update_color(
                            x_, y_, timestamp_unix, color,
                            &mut canvas, &mut canvas_timestamps,
                            &mut canvas_touched
                        );
                    }
                }
            }
            2 => {
                let x: u16 = coordinates.get(0).unwrap().parse().unwrap();
                let y: u16 = coordinates.get(1).unwrap().parse().unwrap();

                update_color(
                    x, y, timestamp_unix, color,
                    &mut canvas, &mut canvas_timestamps, &mut canvas_touched
                );
            }
            _ => panic!("Coordinates weren't in a tuple of 2 or 4!")
        }

        processed_pixels += 1;

    }

    bar.finish();

    println!("Total: {}", total_pixels);
    println!("Untouched pixels:");

    let mut untouched_count: u32 = 0;

    for x in 0..2000 {
        for y in 0..2000 {
            let index: u32 = (y * 2000) + x;
            if canvas_touched[index as usize] == false {
                untouched_count += 1;
                println!("Untouched: {},{}", x, y);
            }
        }
    }
    println!("Untouched pixels in total: {}", untouched_count);
    println!("Rendering image...");

    let mut img = ImageBuffer::new(2000, 2000);

    for x in 0..2000 {
        for y in 0..2000 {
            let index: u32 = (y * 2000) + x;
            let color_index = canvas[index as usize];
            let pixel = index_to_pixel(color_index);
            img.put_pixel(x, y, pixel);
        }
    }

    img.save("place.png").unwrap();

    Ok(())
}
