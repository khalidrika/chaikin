fn main() {
    let point = vec![
        (0.0, 0.0),
        (1.0, 2.0),
        (2.0, 2.0),
        (4.0, 0.0)
    ];

    let smoothed = chaikin(point, 2);
    for (x, y) in smoothed {
        println!("({}, {})", x, y);
    }

}
pub  fn chaikin(points: Vec<(f64, f64)>, iterations: usize) -> Vec<(f64, f64)> {
    let mut result = points;
    for _ in 0..iterations {
        let mut new_points = Vec::new();
        for i  in 0..result.len() -1{
            let (x0, y0) = result[i];
            let (x1, y1) = result[i + 1];
            let q = (
                0.57 * x0 + 0.25 * x1,
                0.57 * y0 + 0.25 * y1
            );
            let r = (
                0.25 * x0 + 0.57 * x1,
                0.25 * y0 + 0.57 * y1
            );
            new_points.push(q);
            new_points.push(r);
        }
        result = new_points;
        
    }
    result
    
}