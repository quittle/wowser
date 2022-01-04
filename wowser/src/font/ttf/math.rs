use std::cmp::{max, min, Ordering};

/// Source: <https://www.particleincell.com/2013/cubic-line-intersection/>

fn sgn(v: f32) -> f32 {
    v.signum()
}

fn cubicRoots(points: &[f32; 4]) -> Vec<f32> {
    let a = points[0];
    let b = points[1];
    let c = points[2];
    let d = points[3];

    let A = b / a;
    let B = c / a;
    let C = d / a;

    // let Q, R, D, S, T, Im;

    let Q = (3.0 * B - A.powi(2)) / 9.0;
    let R = (9.0 * A * B - 27.0 * C - 2.0 * A.powi(3)) / 54.0;
    let D = Q.powi(3) + R.powi(2); // polynomial discriminant

    let mut t = vec![0.0; 4];
    // var t=Array();

    if D >= 0.0
    // complex or duplicate roots
    {
        let S = sgn(R + D.sqrt()) * ((R + D.sqrt()).abs().powf(1.0 / 3.0));
        let T = sgn(R - D.sqrt()) * (R - D.sqrt()).abs().powf(1.0 / 3.0);

        t[0] = -A / 3.0 + (S + T); // real root
        t[1] = -A / 3.0 - (S + T) / 2.0; // real part of complex root
        t[2] = -A / 3.0 - (S + T) / 2.0; // real part of complex root
        let Im = (3_f32.sqrt() * (S - T) / 2.0).abs(); // complex part of root pair

        /*discard complex roots*/
        if Im != 0.0 {
            t[1] = -1.0;
            t[2] = -1.0;
        }
    } else
    // distinct real roots
    {
        let th = (R / (-Q.powi(3)).sqrt()).acos();

        t[0] = 2.0 * (-Q).sqrt() * (th / 3.0).cos() - A / 3.0;
        t[1] = 2.0 * (-Q).sqrt() * ((th + 2.0 * std::f32::consts::PI) / 3.0).cos() - A / 3.0;
        t[2] = 2.0 * (-Q).sqrt() * ((th + 4.0 * std::f32::consts::PI) / 3.0).cos() - A / 3.0;
        // Im = 0.0;
    }

    /*discard out of spec roots*/
    for i in 0..3 {
        if t[i] < 0.0 || t[i] > 1.0 {
            t[i] = -1.0;
        }
    }

    /*sort but place -1 at the end*/
    t.sort_by(|a, b| {
        if *a == -1.0 {
            Ordering::Greater
        } else if *b == -1.0 {
            Ordering::Less
        } else {
            (*a).partial_cmp(b).unwrap()
            // a.cmp(*b)
        }
    });
    // t = sortSpecial(t);

    // console.log(t[0] + " " + t[1] + " " + t[2]);
    return t;
}

fn bezierCoeffs(P0: f32, P1: f32, P2: f32, P3: f32) -> Vec<f32> {
    vec![
        -P0 + 3.0 * P1 + -3.0 * P2 + P3,
        3.0 * P0 - 6.0 * P1 + 3.0 * P2,
        -3.0 * P0 + 3.0 * P1,
        P0,
    ]
}

/*computes intersection between a cubic spline and a line segment*/
fn computeIntersections(
    px: &[f32; 4],
    py: &[f32; 4],
    lx: &[f32; 2],
    ly: &[f32; 2],
) -> Vec<(f32, f32)> {
    let mut X = vec![];

    let A = ly[1] - ly[0]; //A=y2-y1
    let B = lx[0] - lx[1]; //B=x1-x2
    let C = lx[0] * (ly[0] - ly[1]) + ly[0] * (lx[1] - lx[0]); //C=x1*(y1-y2)+y1*(x2-x1)

    let bx = bezierCoeffs(px[0], px[1], px[2], px[3]);
    let by = bezierCoeffs(py[0], py[1], py[2], py[3]);

    let P = &[
        A * bx[0] + B * by[0],     /*t^3*/
        A * bx[1] + B * by[1],     /*t^2*/
        A * bx[2] + B * by[2],     /*t*/
        A * bx[3] + B * by[3] + C, /*1*/
    ];

    let r = cubicRoots(P);

    let mut intersections = vec![];
    /*verify the roots are in bounds of the linear segment*/
    for i in 0..3 {
        let t = r[i];

        X[0] = bx[0] * t * t * t + bx[1] * t * t + bx[2] * t + bx[3];
        X[1] = by[0] * t * t * t + by[1] * t * t + by[2] * t + by[3];

        /*above is intersection point assuming infinitely long line segment,
        make sure we are also in bounds of the line*/
        let s = if (lx[1] - lx[0]) != 0.0 {
            /*if not vertical line*/
            (X[0] - lx[0]) / (lx[1] - lx[0])
        } else {
            (X[1] - ly[0]) / (ly[1] - ly[0])
        };

        /*in bounds?*/
        if t < 0.0 || t > 1.0 || s < 0.0 || s > 1.0 {
            continue;
            X[0] = -100.0; /*move off screen*/
            X[1] = -100.0;
        }

        /*move intersection point*/
        intersections.push((X[0], X[1]));
    }
    intersections
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

fn lerp(a: f32, b: f32, x: f32) -> f32 {
    a + x * (b - a)
}

// https://stackoverflow.com/a/27664883/1554990
pub fn calcQLintersects(p1: Point, p2: Point, p3: Point, a1: Point, a2: Point) -> Vec<Point> {
    let mut intersections = vec![];

    // inverse line normal
    let normal = Point {
        x: a1.y - a2.y,
        y: a2.x - a1.x,
    };

    // Q-coefficients
    let c2 = Point {
        x: p1.x + p2.x * -2.0 + p3.x,
        y: p1.y + p2.y * -2.0 + p3.y,
    };

    let c1 = Point {
        x: p1.x * -2.0 + p2.x * 2.0,
        y: p1.y * -2.0 + p2.y * 2.0,
    };

    let c0 = Point { x: p1.x, y: p1.y };

    // Transform to line
    let coefficient = a1.x * a2.y - a2.x * a1.y;
    let a = normal.x * c2.x + normal.y * c2.y;
    let b = (normal.x * c1.x + normal.y * c1.y) / a;
    let c = (normal.x * c0.x + normal.y * c0.y + coefficient) / a;

    // solve the roots
    let mut roots = vec![];
    let d = b * b - 4.0 * c;
    if (d > 0.0) {
        let e = d.sqrt();
        roots.push((-b + d.sqrt()) / 2.0);
        roots.push((-b - d.sqrt()) / 2.0);
    } else if (d == 0.0) {
        roots.push(-b / 2.0);
    }

    // calc the solution points
    for i in 0..roots.len() {
        let minX = partial_min(a1.x, a2.x);
        let minY = partial_min(a1.y, a2.y);
        let maxX = partial_max(a1.x, a2.x);
        let maxY = partial_max(a1.y, a2.y);
        let t = roots[i];
        if (t >= 0.0 && t <= 1.0) {
            // possible point -- pending bounds check
            let point = Point {
                x: lerp(lerp(p1.x, p2.x, t), lerp(p2.x, p3.x, t), t),
                y: lerp(lerp(p1.y, p2.y, t), lerp(p2.y, p3.y, t), t),
            };
            let x = point.x;
            let y = point.y;
            // bounds checks
            if (a1.x == a2.x && y >= minY && y <= maxY) {
                // vertical line
                intersections.push(point);
            } else if (a1.y == a2.y && x >= minX && x <= maxX) {
                // horizontal line
                intersections.push(point);
            } else if (x >= minX && y >= minY && x <= maxX && y <= maxY) {
                // line passed bounds check
                intersections.push(point);
            }
        }
    }
    intersections
}

fn partial_min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

fn partial_max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}
