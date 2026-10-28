use unistroke_recognition::prelude::{Point, Recognizer};

#[test]
fn test_issue_3() {
    let points =
        [(0.5, -0.5), (-0.25, -1.0), (-1.0, -0.25)].map(|(x, y)| Point::new(x, y).unwrap());

    let _ = Recognizer::new(None).parse(&points);
}
