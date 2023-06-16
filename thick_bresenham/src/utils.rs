const LINE_OVERLAP_NONE: usize = 0; // No line overlap, like in standard Bresenham
const LINE_OVERLAP_MAJOR: usize = 0x01; // Overlap - first go major then minor direction. Pixel is drawn as extension after actual line
const LINE_OVERLAP_MINOR: usize = 0x02; // Overlap - first go minor then major direction. Pixel is drawn as extension before next line
const LINE_OVERLAP_BOTH: usize = 0x03; // Overlap - both

#[derive(PartialEq)]
pub enum ThicknessMode {
    LineThicknessMiddle = 0,
    LineThicknessDrawClockwise = 1,
    LineThicknessDrawCounterclockwise = 2,
}

#[inline]
// ported from https://github.com/ArminJo/Arduino-BlueDisplay/blob/master/src/LocalGUI/ThickLine.hpp
fn get_line_overlap(
    from: (isize, isize),
    to: (isize, isize),
    overlap: usize,
    map_width: usize,
) -> Vec<(usize, usize)> {
    let mut x1 = from.0;
    let x2 = to.0;
    let mut y1 = from.1;
    let y2 = to.1;
    let mut line: Vec<(usize, usize)> = vec![];
    let mut dx: isize;
    let mut dy: isize;
    let dx2: isize;
    let dy2: isize;
    let mut err: isize;
    let step_x: isize;
    let step_y: isize;
    let max_width = (map_width - 1) as isize;
    dx = x2 - x1;
    dy = y2 - y1;
    if dx < 0 {
        dx = -dx;
        step_x = -1;
    } else {
        step_x = 1;
    }
    if dy < 0 {
        dy = -dy;
        step_y = -1;
    } else {
        step_y = 1;
    }
    dx2 = dx << 1;
    dy2 = dy << 1;
    line.push((
        x1.clamp(0, max_width) as usize,
        y1.clamp(0, max_width) as usize,
    ));
    if dx > dy {
        err = dy2 - dx;
        while x1 != x2 {
            x1 += step_x;
            if err >= 0 {
                if overlap & LINE_OVERLAP_MAJOR != 0 {
                    line.push((
                        x1.clamp(0, max_width) as usize,
                        y1.clamp(0, max_width) as usize,
                    ));
                }
                y1 += step_y;
                if overlap & LINE_OVERLAP_MINOR != 0 {
                    line.push((
                        (x1 - step_x).clamp(0, max_width) as usize,
                        y1.clamp(0, max_width) as usize,
                    ));
                }
                err -= dx2;
            }
            err += dy2;
            line.push((
                x1.clamp(0, max_width) as usize,
                y1.clamp(0, max_width) as usize,
            ));
        }
    } else {
        err = dx2 - dy;
        while y1 != y2 {
            y1 += step_y;
            if err >= 0 {
                if overlap & LINE_OVERLAP_MAJOR != 0 {
                    line.push((
                        x1.clamp(0, max_width) as usize,
                        y1.clamp(0, max_width) as usize,
                    ));
                }
                x1 += step_x;
                if overlap & LINE_OVERLAP_MINOR != 0 {
                    line.push((
                        x1.clamp(0, max_width) as usize,
                        (y1 - step_y).clamp(0, max_width) as usize,
                    ));
                }
                err -= dy2;
            }
            err += dx2;
            line.push((
                x1.clamp(0, max_width) as usize,
                y1.clamp(0, max_width) as usize,
            ));
        }
    }
    line
}

#[inline]
// ported from https://github.com/ArminJo/Arduino-BlueDisplay/blob/master/src/LocalGUI/ThickLine.hpp
fn get_line_overlap2(
    from: (isize, isize),
    to: (isize, isize),
    overlap: usize,
) -> Vec<(isize, isize)> {
    let mut x1 = from.0;
    let x2 = to.0;
    let mut y1 = from.1;
    let y2 = to.1;
    let mut line: Vec<(isize, isize)> = vec![];
    let mut dx: isize;
    let mut dy: isize;
    let dx2: isize;
    let dy2: isize;
    let mut err: isize;
    let step_x: isize;
    let step_y: isize;
    dx = x2 - x1;
    dy = y2 - y1;
    if dx < 0 {
        dx = -dx;
        step_x = -1;
    } else {
        step_x = 1;
    }
    if dy < 0 {
        dy = -dy;
        step_y = -1;
    } else {
        step_y = 1;
    }
    dx2 = dx << 1;
    dy2 = dy << 1;
    line.push((x1, y1));
    if dx > dy {
        err = dy2 - dx;
        while x1 != x2 {
            x1 += step_x;
            if err >= 0 {
                if overlap & LINE_OVERLAP_MAJOR != 0 {
                    line.push((x1, y1));
                }
                y1 += step_y;
                if overlap & LINE_OVERLAP_MINOR != 0 {
                    line.push((x1 - step_x, y1));
                }
                err -= dx2;
            }
            err += dy2;
            line.push((x1, y1));
        }
    } else {
        err = dx2 - dy;
        while y1 != y2 {
            y1 += step_y;
            if err >= 0 {
                if overlap & LINE_OVERLAP_MAJOR != 0 {
                    line.push((x1, y1));
                }
                x1 += step_x;
                if overlap & LINE_OVERLAP_MINOR != 0 {
                    line.push((x1, y1 - step_y));
                }
                err -= dy2;
            }
            err += dx2;
            line.push((x1, y1));
        }
    }
    line
}

#[inline]
fn try_max(x: isize, y: isize, result: &mut bool) -> (usize, usize) {
    if x < 0 && y < 0 {
        *result = true;
        return (0, 0);
    } else if x < 0 {
        *result = true;
        return (0, y as usize);
    } else if y < 0 {
        *result = true;
        return (x as usize, 0);
    }
    *result = false;
    (x as usize, y as usize)
}

#[inline]
// ported from https://github.com/ArminJo/Arduino-BlueDisplay/blob/master/src/LocalGUI/ThickLine.hpp
// i strongly discourage use of the unsafe_unchecked version unless you're 100% sure of what you're doing
pub fn get_thick_line(
    from: (usize, usize),
    to: (usize, usize),
    line_width: usize,
    thick_mode: ThicknessMode,
    map_width: usize,
    unsafe_unchecked: bool,
) -> Vec<(usize, usize)> {
    let mut line: Vec<(isize, isize)> = vec![];
    let mut x1 = from.0 as isize;
    let mut x2 = to.0 as isize;
    let mut y1 = from.1 as isize;
    let mut y2 = to.1 as isize;
    let mut dx: isize;
    let mut dy: isize;
    let dx2: isize;
    let dy2: isize;
    let mut err: isize;
    let mut step_x: isize;
    let mut step_y: isize;
    let max_width = (map_width - 1) as isize;
    if x1 > max_width {
        x1 = max_width;
    }
    if x2 > max_width {
        x2 = max_width;
    }
    if y1 > max_width {
        y1 = max_width;
    }
    if y2 > max_width {
        y2 = max_width;
    }
    fn convert(input: &Vec<(isize, isize)>, max_width: isize) -> Vec<(usize, usize)> {
        let mut retval: Vec<(usize, usize)> = Vec::with_capacity(input.len());
        for x in 0..input.len() {
            if input[x].0 >= 0
                && input[x].0 <= max_width
                && input[x].1 >= 0
                && input[x].1 <= max_width
            {
                retval.push((input[x].0 as usize, input[x].1 as usize));
            }
        }
        retval
    }
    unsafe fn fast_convert(input: Vec<(isize, isize)>) -> Vec<(usize, usize)> {
        let mut v = std::mem::ManuallyDrop::new(input);
        Vec::from_raw_parts(v.as_mut_ptr() as *mut (usize, usize), v.len(), v.capacity())
    }
    if line_width <= 1 {
        line = get_line_overlap2((x1, y1), (x2, y2), LINE_OVERLAP_NONE);
        return convert(&line, max_width);
    }
    dy = x2 - x1;
    dx = y2 - y1;
    let mut swap = true;
    if dx < 0 {
        dx = -dx;
        step_x = -1;
        swap = !swap;
    } else {
        step_x = 1;
    }
    if dy < 0 {
        dy = -dy;
        step_y = -1;
        swap = !swap;
    } else {
        step_y = 1;
    }
    dx2 = dx << 1;
    dy2 = dy << 1;
    let mut overlap: usize;
    let mut draw_start_adjust_count = line_width / 2;
    if thick_mode == ThicknessMode::LineThicknessDrawCounterclockwise {
        draw_start_adjust_count = line_width - 1;
    } else if thick_mode == ThicknessMode::LineThicknessDrawClockwise {
        draw_start_adjust_count = 0;
    }
    if dx >= dy {
        if swap {
            draw_start_adjust_count = (line_width - 1) - draw_start_adjust_count;
            step_y = -step_y;
        } else {
            step_x = -step_x;
        }
        err = dy2 - dx;
        for _ in (0..draw_start_adjust_count).rev() {
            x1 -= step_x;
            x2 -= step_x;
            if err >= 0 {
                y1 -= step_y;
                y2 -= step_y;
                err -= dx2;
            }
            err += dy2;
        }
        line.append(&mut get_line_overlap2(
            (x1, y1),
            (x2, y2),
            LINE_OVERLAP_NONE,
        ));
        err = dy2 - dx;
        for _ in (1..line_width).rev() {
            x1 += step_x;
            x2 += step_x;
            overlap = LINE_OVERLAP_NONE;
            if err >= 0 {
                y1 += step_y;
                y2 += step_y;
                err -= dx2;
                overlap = LINE_OVERLAP_MAJOR;
            }
            err += dy2;
            line.append(&mut get_line_overlap2((x1, y1), (x2, y2), overlap));
        }
    } else {
        if swap {
            step_x = -step_x;
        } else {
            draw_start_adjust_count = (line_width - 1) - draw_start_adjust_count;
            step_y = -step_y;
        }
        err = dx2 - dy;
        for _ in (0..draw_start_adjust_count).rev() {
            y1 -= step_y;
            y2 -= step_y;
            if err >= 0 {
                x1 -= step_x;
                x2 -= step_x;
                err -= dy2;
            }
            err += dx2;
        }
        line.append(&mut get_line_overlap2(
            (x1, y1),
            (x2, y2),
            LINE_OVERLAP_NONE,
        ));
        err = dx2 - dy;
        for _ in (1..line_width).rev() {
            y1 += step_y;
            y2 += step_y;
            overlap = LINE_OVERLAP_NONE;
            if err >= 0 {
                x1 += step_x;
                x2 += step_x;
                err -= dy2;
                overlap = LINE_OVERLAP_MAJOR;
            }
            err += dx2;
            line.append(&mut get_line_overlap2((x1, y1), (x2, y2), overlap));
        }
    }
    if unsafe_unchecked {
        unsafe { fast_convert(line) }
    } else {
        convert(&line, max_width)
    }
}

#[inline]
// ported from https://github.com/ArminJo/Arduino-BlueDisplay/blob/master/src/LocalGUI/ThickLine.hpp
pub fn get_thick_line_unchecked(
    from: (usize, usize),
    to: (usize, usize),
    line_width: usize,
    thick_mode: ThicknessMode,
    map_width: usize,
) -> Vec<(usize, usize)> {
    let mut line: Vec<(usize, usize)> = vec![];
    let mut x1 = from.0 as isize;
    let mut x2 = to.0 as isize;
    let mut y1 = from.1 as isize;
    let mut y2 = to.1 as isize;
    let mut dx: isize;
    let mut dy: isize;
    let dx2: isize;
    let dy2: isize;
    let mut err: isize;
    let mut step_x: isize;
    let mut step_y: isize;
    let max_width = (map_width - 1) as isize;
    if x1 > max_width {
        x1 = max_width;
    }
    if x2 > max_width {
        x2 = max_width;
    }
    if y1 > max_width {
        y1 = max_width;
    }
    if y2 > max_width {
        y2 = max_width;
    }
    if line_width <= 1 {
        return get_line_overlap((x1, y1), (x2, y2), LINE_OVERLAP_NONE, map_width);
    }
    dy = x2 - x1;
    dx = y2 - y1;
    let mut swap = true;
    if dx < 0 {
        dx = -dx;
        step_x = -1;
        swap = !swap;
    } else {
        step_x = 1;
    }
    if dy < 0 {
        dy = -dy;
        step_y = -1;
        swap = !swap;
    } else {
        step_y = 1;
    }
    dx2 = dx << 1;
    dy2 = dy << 1;
    let mut overlap: usize;
    let mut draw_start_adjust_count = line_width / 2;
    if thick_mode == ThicknessMode::LineThicknessDrawCounterclockwise {
        draw_start_adjust_count = line_width - 1;
    } else if thick_mode == ThicknessMode::LineThicknessDrawClockwise {
        draw_start_adjust_count = 0;
    }
    if dx >= dy {
        if swap {
            draw_start_adjust_count = (line_width - 1) - draw_start_adjust_count;
            step_y = -step_y;
        } else {
            step_x = -step_x;
        }
        err = dy2 - dx;
        for _ in (0..draw_start_adjust_count).rev() {
            x1 -= step_x;
            x2 -= step_x;
            if err >= 0 {
                y1 -= step_y;
                y2 -= step_y;
                err -= dx2;
            }
            err += dy2;
        }
        line.append(&mut get_line_overlap(
            (x1, y1),
            (x2, y2),
            LINE_OVERLAP_NONE,
            map_width,
        ));
        err = dy2 - dx;
        for _ in (1..line_width).rev() {
            x1 += step_x;
            x2 += step_x;
            overlap = LINE_OVERLAP_NONE;
            if err >= 0 {
                y1 += step_y;
                y2 += step_y;
                err -= dx2;
                overlap = LINE_OVERLAP_MAJOR;
            }
            err += dy2;
            line.append(&mut get_line_overlap(
                (x1, y1),
                (x2, y2),
                overlap,
                map_width,
            ))
        }
    } else {
        if swap {
            step_x = -step_x;
        } else {
            draw_start_adjust_count = (line_width - 1) - draw_start_adjust_count;
            step_y = -step_y;
        }
        err = dx2 - dy;
        for _ in (0..draw_start_adjust_count).rev() {
            y1 -= step_y;
            y2 -= step_y;
            if err >= 0 {
                x1 -= step_x;
                x2 -= step_x;
                err -= dy2;
            }
            err += dx2;
        }
        line.append(&mut get_line_overlap(
            (x1, y1),
            (x2, y2),
            LINE_OVERLAP_NONE,
            map_width,
        ));
        err = dx2 - dy;
        for _ in (1..line_width).rev() {
            y1 += step_y;
            y2 += step_y;
            overlap = LINE_OVERLAP_NONE;
            if err >= 0 {
                x1 += step_x;
                x2 += step_x;
                err -= dy2;
                overlap = LINE_OVERLAP_MAJOR;
            }
            err += dx2;
            line.append(&mut get_line_overlap(
                (x1, y1),
                (x2, y2),
                overlap,
                map_width,
            ));
        }
    }
    // these are 2.5x slower than using get_thick_line() to remove dupes:
    //line.sort();
    //line.dedup();
    line
}

#[inline(always)]
pub fn distance(pos1: (usize, usize), pos2: (usize, usize)) -> f32 {
    let x1 = pos1.0 as f32;
    let x2 = pos2.0 as f32;
    let y1 = pos1.1 as f32;
    let y2 = pos2.1 as f32;
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

// ported from http://members.chello.at/%7Eeasyfilter/bresenham.html
#[inline]
pub fn get_line_unchecked(from: (usize, usize), to: (usize, usize)) -> Vec<(usize, usize)> {
    let mut line: Vec<(usize, usize)> = vec![];
    let mut ix0 = from.0 as isize;
    let mut iy0 = from.1 as isize;
    let ix1 = to.0 as isize;
    let iy1 = to.1 as isize;
    let dx: isize = ix1.abs_diff(ix0) as isize;
    let mut sx: isize = 1;
    if ix0 > ix1 {
        sx = -1;
    }
    let dy: isize = -(iy1.abs_diff(iy0) as isize);
    let mut sy: isize = 1;
    if iy0 > iy1 {
        sy = -1
    }
    let mut err = dx + dy;
    let mut e2: isize = 0;
    loop {
        line.push((ix0 as usize, iy0 as usize));
        if ix0 == ix1 && iy0 == iy1 {
            break;
        }
        e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            ix0 += sx;
        }
        if e2 <= dx {
            err += dx;
            iy0 += sy;
        }
    }
    line
}

// ported from http://members.chello.at/%7Eeasyfilter/bresenham.html
/// if remove_out_of_bounds = false, points outside of map will be placed on edge of map
/// if remove_out_of_bounds = true, points outside of map will be ignored
#[inline]
pub fn get_line(
    from: (usize, usize),
    to: (usize, usize),
    map_width: usize,
    remove_out_of_bounds: bool,
) -> Vec<(usize, usize)> {
    let mut line: Vec<(usize, usize)> = vec![];
    let mut ix0 = from.0 as isize;
    let mut iy0 = from.1 as isize;
    let ix1 = to.0 as isize;
    let iy1 = to.1 as isize;
    let dx: isize = ix1.abs_diff(ix0) as isize;
    let mut sx: isize = 1;
    let imap_width = map_width as isize;
    let mut clamped: bool = false;
    let mut try_clamp = |coord: isize| -> usize {
        if coord > imap_width - 1 {
            clamped = true;
            map_width - 1
        } else if coord < 0 {
            clamped = true;
            0
        } else {
            coord as usize
        }
    };

    if ix0 > ix1 {
        sx = -1;
    }
    let dy: isize = -(iy1.abs_diff(iy0) as isize);
    let mut sy: isize = 1;
    if iy0 > iy1 {
        sy = -1
    }
    let mut err = dx + dy;
    let mut e2: isize = 0;
    let mut skip_point: bool;
    loop {
        skip_point = false;
        if remove_out_of_bounds
            && (ix0 > imap_width - 1 || ix0 < 0 || iy0 > imap_width - 1 || iy0 < 0)
        {
            skip_point = true;
        }
        if !skip_point {
            line.push((try_clamp(ix0), try_clamp(iy0)))
        };
        if ix0 == ix1 && iy0 == iy1 {
            break;
        }
        e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            ix0 += sx;
        }
        if e2 <= dx {
            err += dx;
            iy0 += sy;
        }
    }
    if !remove_out_of_bounds && clamped {
        line.sort();
        line.dedup();
    }
    line
}

#[inline]
pub fn get_full_circle_naive(
    xm: usize,
    ym: usize,
    r: f32,
    width: usize,
    with_nipples: bool,
    ignore_center: bool,
) -> Vec<(usize, usize)> {
    let mut full_circle: Vec<(usize, usize)> = vec![];
    let mut nipples: Vec<(usize, usize)> = vec![];
    let startx: usize;
    let starty: usize;
    let endx: usize;
    let endy: usize;
    let r2 = r as usize;
    if xm >= r2 {
        startx = xm - r2;
    } else {
        startx = 0;
    }
    if ym >= r2 {
        starty = ym - r2;
    } else {
        starty = 0;
    }
    if width > xm + r2 {
        endx = xm + r2
    } else {
        endx = width - 1;
    }
    if width > ym + r2 {
        endy = ym + r2;
    } else {
        endy = width - 1;
    }
    let mut dist: f32;
    for x in startx..=endx {
        for y in starty..=endy {
            if ignore_center && x == xm && y == ym {
                continue;
            }
            dist = distance((x, y), (xm, ym));
            if dist < r {
                full_circle.push((x, y));
            } else if with_nipples && dist == r {
                nipples.push((x, y));
            }
        }
    }
    if with_nipples {
        full_circle.append(&mut nipples);
    }
    full_circle
}

#[inline]
//adapted from http://members.chello.at/%7Eeasyfilter/bresenham.html
pub fn get_full_circle(xm: usize, ym: usize, mut r: isize, width: usize) -> Vec<(usize, usize)> {
    let xm = xm as isize;
    let ym = ym as isize;
    let width2 = width as isize;

    let mut x = -r;
    let mut y = 0;
    let mut err: isize = 2 - 2 * r;
    let mut empty_circle: Vec<(usize, usize)> = vec![];
    let mut full_circle: Vec<(usize, usize)> = vec![];
    let mut clamped: bool = false;
    // need to know if clamp has ocurred, as this can create duplicate coordinates
    let mut try_clamp = |coord: isize| -> usize {
        if coord < 0 {
            clamped = true;
            0
        } else if coord >= width2 {
            clamped = true;
            width - 1
        } else {
            coord as usize
        }
    };
    while x < 0 {
        empty_circle.push((try_clamp(xm - x), try_clamp(ym + y)));
        empty_circle.push((try_clamp(xm - y), try_clamp(ym - x)));
        empty_circle.push((try_clamp(xm + x), try_clamp(ym - y)));
        empty_circle.push((try_clamp(xm + y), try_clamp(ym + x)));
        r = err;
        if r <= y {
            y += 1;
            err += y * 2 + 1;
        }
        if r > x || err > y {
            x += 1;
            err += x * 2 + 1;
        }
    }
    if empty_circle.len() == 0 {
        return empty_circle;
    }
    // sort by X axis. this allows easy dupe removal and quickly getting coords inside the circle
    empty_circle.sort();
    if clamped {
        empty_circle.dedup();
    }
    let mut lowest = usize::MAX;
    let mut highest = 0;
    let mut current = empty_circle.first().unwrap().0;
    //find lowest Y and highest Y for each X coordinate, and grab all the coords between (inclusive)
    for p in empty_circle {
        if p.0 != current {
            for y in lowest..=highest {
                full_circle.push((current, y));
            }
            lowest = usize::MAX;
            highest = 0;
            current = p.0;
        }
        if p.1 < lowest {
            lowest = p.1;
        }
        if p.1 > highest {
            highest = p.1;
        }
    }
    for y in lowest..=highest {
        full_circle.push((current, y));
    }
    full_circle
}
