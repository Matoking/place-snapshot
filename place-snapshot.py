import sys

import gzip
import csv
import datetime

from PIL import Image


COLOR_TO_INDEX = {
    "#000000": 0,
    "#00CCC0": 1,
    "#94B3FF": 2,
    "#6A5CFF": 3,
    "#009EAA": 4,
    "#E4ABFF": 5,
    "#00756F": 6,
    "#00A368": 7,
    "#00CC78": 8,
    "#2450A4": 9,
    "#3690EA": 10,
    "#493AC1": 11,
    "#515252": 12,
    "#51E9F4": 13,
    "#6D001A": 14,
    "#6D482F": 15,
    "#7EED56": 16,
    "#811E9F": 17,
    "#898D90": 18,
    "#9C6926": 19,
    "#B44AC0": 20,
    "#BE0039": 21,
    "#D4D7D9": 22,
    "#DE107F": 23,
    "#FF3881": 24,
    "#FF4500": 25,
    "#FF99AA": 26,
    "#FFA800": 27,
    "#FFB470": 28,
    "#FFD635": 29,
    "#FFF8B8": 30,
    "#FFFFFF": 31
}

INDEX_TO_PIXEL = {
    0: (0, 0, 0),
    1: (0, 204, 192),
    2: (148, 179, 255),
    3: (106, 92, 255),
    4: (0, 158, 170),
    5: (228, 171, 255),
    6: (0, 117, 111),
    7: (0, 163, 104),
    8: (0, 204, 120),
    9: (36, 80, 164),
    10: (54, 144, 234),
    11: (73, 58, 193),
    12: (81, 82, 82),
    13: (81, 233, 244),
    14: (109, 0, 26),
    15: (109, 72, 47),
    16: (126, 237, 86),
    17: (129, 30, 159),
    18: (137, 141, 144),
    19: (156, 105, 38),
    20: (180, 74, 192),
    21: (190, 0, 57),
    22: (212, 215, 217),
    23: (222, 16, 127),
    24: (255, 56, 129),
    25: (255, 69, 0),
    26: (255, 153, 170),
    27: (255, 168, 0),
    28: (255, 180, 112),
    29: (255, 214, 53),
    30: (255, 248, 184),
    31: (255, 255, 255),
}


def update_color(x, y, timestamp, color, canvas, canvas_timestamps, canvas_touched):
    index = (y * 2000) + x
    previous_timestamp = canvas_timestamps[index]

    if previous_timestamp > timestamp:
        return

    canvas_timestamps[index] = timestamp
    canvas[index] = COLOR_TO_INDEX[color]
    canvas_touched[index] = True


def str_to_timestamp(time_str):
    day = int(time_str[8:10])
    hour = int(time_str[11:13])
    minute = int(time_str[14:16])
    second = int(time_str[17:19])

    timestamp = second
    timestamp += 60 * minute
    timestamp += 60 * 60 * hour
    timestamp += 24 * 60 * 60 * day

    return timestamp


def main():
    if len(sys.argv) < 3:
        print("Usage: <place_dataset.csv.gzip> <timestamp>")
        print("Example: ")
        print("  ./place-snapshot.py /foo/snapshot.csv.gzip \"2022-04-04 18:00:00\"")
        sys.exit(0)

    dataset_path = sys.argv[1]
    cutoff = sys.argv[2]

    canvas = [31 for _ in range(2000*2000)]
    canvas_timestamps = [0 for _ in range(2000*2000)]
    canvas_touched = [False for _ in range(2000*2000)]

    csv_reader = csv.reader(gzip.open(dataset_path, "rt"))

    total_pixels = 0
    processed_pixels = 0

    for record in csv_reader:
        total_pixels += 1

        timestamp = record[0]

        if timestamp > cutoff:
            continue

        timestamp_unix = str_to_timestamp(timestamp)

        color = record[2]
        coordinates = [int(coord) for coord in record[3].split(",")]

        if len(coordinates) == 4:
            print(f"Found moderator placement at {timestamp}")
            x1, y1, x2, y2 = coordinates

            for x_ in range(x1, x2+1):
                for y_ in range(y1, y2+1):
                    update_color(
                        x_, y_, timestamp_unix, color,
                        canvas, canvas_timestamps, canvas_touched
                    )
        else:
            x, y = coordinates
            update_color(
                x, y, timestamp_unix, color,
                canvas, canvas_timestamps, canvas_touched
            )

        processed_pixels += 1

        if total_pixels % 100000 == 0:
            print(f"{total_pixels} pixels iterated, {processed_pixels} processed so far...")

    print(f"Total {total_pixels}")
    print("Untouched pixels:")

    for x in range(0, 2000):
        for y in range(0, 2000):
            index = (y * 2000) + x
            if not canvas_touched[index]:
                print(f"Untouched: {x},{y}")

    print("Rendering image...")

    img = Image.new("RGB", (2000, 2000))

    for x in range(0, 2000):
        for y in range(0, 2000):
            index = (y * 2000) + x
            color_index = canvas[index]
            img.putpixel((x, y), INDEX_TO_PIXEL[color_index])

    img.save("place.png")


if __name__ == "__main__":
    main()
