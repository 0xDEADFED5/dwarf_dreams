mod utils;
use nanorand::Rng;
use std::time::{Duration, Instant};


fn test_fat_lines(tests: usize, map_width: usize, line_width: usize) {
    println!("----------------------- [ test begin ] -----------------------");
    let mut rng = nanorand::tls_rng();
    let mut points1: Vec<(usize, usize)> = vec![];
    let mut points2: Vec<(usize, usize)> = vec![];
    let mut line: Vec<(usize, usize)>;
    let mut len: usize = 0;
    for _ in 0..tests {
        points1.push((rng.generate_range(0..map_width), rng.generate_range(0..map_width)));
        points2.push((rng.generate_range(0..map_width), rng.generate_range(0..map_width)));
    }

    let start = Instant::now();
    for x in 0..tests {
        line = utils::get_thick_line_unchecked(
            points1[x],
            points2[x],
            line_width,
            utils::ThicknessMode::LineThicknessMiddle,
            map_width,
        );
        len += line.len();
    }
    let duration = start.elapsed();
    println!(
        "{} iterations for get_thick_line_unchecked() took: {:?}",
        tests, duration
    );
    println!(
        "total pixels = {}, line width = {}, canvas size = {}x{}\n",
        len, line_width, map_width, map_width
    );


    len = 0;
    let start = Instant::now();
    for x in 0..tests {
        line = utils::get_thick_line(
            points1[x],
            points2[x],
            line_width,
            utils::ThicknessMode::LineThicknessMiddle,
            map_width,
            false,
        );
        len += line.len();
    }
    let duration = start.elapsed();
    println!(
        "{} iterations for get_thick_line() took: {:?}",
        tests, duration
    );
    println!(
        "total pixels = {}, line width = {}, canvas size = {}x{}\n",
        len, line_width, map_width, map_width
    );

    len = 0;
    let start = Instant::now();
    for x in 0..tests {
        line = utils::get_thick_line(
            points1[x],
            points2[x],
            line_width,
            utils::ThicknessMode::LineThicknessMiddle,
            map_width,
            true,
        );
        len += line.len();
    }
    let duration = start.elapsed();
    println!(
        "{} iterations for get_thick_line(unsafe unchecked) took: {:?}",
        tests, duration
    );
    println!(
        "total pixels = {}, line width = {}, canvas size = {}x{}\n",
        len, line_width, map_width, map_width
    );
}
fn main() {
    test_fat_lines(5_000, 1000, 500);
    test_fat_lines(5_000, 1000, 200);
    test_fat_lines(10_000, 1000, 50);
    test_fat_lines(50_000, 1000, 10);
}
