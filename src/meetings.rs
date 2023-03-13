use std::cmp;

fn overlapping_meetings(meetings_a: Vec<Vec<i32>>, meetings_b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intersection = Vec::new();

    for i in 0..meetings_a.len() {
        for j in 0..meetings_b.len() {
            let (starting_a, starting_b) = (meetings_a[i][0], meetings_b[j][0]);
            let (ending_a, ending_b) = (meetings_a[i][1], meetings_b[j][1]);

            let overlap_status = overlap(starting_a, starting_b, ending_a, ending_b);

            if overlap_status != None {
                intersection.push(overlap_status.unwrap());
            }
        }
    }

    intersection
}

fn overlap(start_a: i32, start_b: i32, end_a: i32, end_b: i32) -> Option<Vec<i32>> {
    let mut intersection_time: Vec<i32> = Vec::new();

    if cmp::max(start_a, start_b) < cmp::min(end_a, end_b) {
        intersection_time.push(cmp::max(start_a, start_b));
        intersection_time.push(cmp::min(end_a, end_b));
        Some(intersection_time)
    } else {
        None
    }
}

fn main() {
    let meeting_sec_a: Vec<Vec<i32>> = vec![vec![13, 15], vec![15, 16], vec![7, 9]];

    let meeting_sec_b: Vec<Vec<i32>> = vec![vec![14, 15], vec![5, 10]];

    let intersection = overlapping_meetings(meeting_sec_a, meeting_sec_b);

    println!("The overlapping timings are {:?}", intersection);
}
