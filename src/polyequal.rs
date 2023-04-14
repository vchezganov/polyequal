use polysplit::euclidean::Point;
use polysplit::polyline_split;

pub fn polyline_equal(a: &[Point], b: &[Point], threshold: f64) -> bool {
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
