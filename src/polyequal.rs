use polysplit::euclidean::Point;
use polysplit::polyline_split;

pub fn polyline_equal1(a: &[Point], b: &[Point], threshold: f64) -> bool {
    let threshold = Some(threshold);

    let first_split = polyline_split(a, b, threshold);
    let segments = match first_split {
        Ok(v) => v,
        Err(_) => return false,
    };

    let mut polyline = Vec::new();
    polyline.extend_from_slice(&segments[0]);
    for segment in segments.iter() {
        polyline.extend_from_slice(&segment[1..]);
    }

    let second_split = polyline_split(&b, &polyline, threshold);

    second_split.is_ok()
}


pub fn polyline_equal(b: &[Point], a: &[Point], threshold: f64) -> bool {
    let threshold = Some(threshold);

    let first_split = polyline_split(a, b, threshold);
    let first_segments = match first_split {
        Ok(v) => v,
        Err(_) => return false,
    };

    let mut polyline = Vec::new();
    polyline.extend_from_slice(&first_segments[0]);
    for segment in first_segments.iter() {
        polyline.extend_from_slice(&segment[1..]);
    }

    let second_split = polyline_split(&b, &polyline, threshold);
    let second_segments = match second_split {
        Ok(v) => v,
        Err(_) => return false,
    };

    // 0
    let p0 = polyline[0];
    let p1 = second_segments[0][0];
    polyline[0] = Point((p0.0 + p1.0) / 2.0, (p0.1 + p1.1) / 2.0);

    for (i, p0) in polyline.iter_mut().skip(1).enumerate() {
        let p1 = second_segments[i].last().unwrap();
        // polyline[i] = Point((p0.0 + p1.0) / 2.0, (p0.1 + p1.1) / 2.0);
        *p0 = Point((p0.0 + p1.0) / 2.0, (p0.1 + p1.1) / 2.0);
    }


    println!("{:?}", polyline);
    panic!("asd");

    true
}


// 0 ---  1 ---  2 ---  3 ---  4 --- 5
//
// 0 - 1, 1 - 2, 2 - 3, 3 - 4, 4 - 5
