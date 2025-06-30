/*
Given an projected area, find the rotation of a cube
*/ 

use std::f32::consts::PI;
extern crate nalgebra as na;
use na::{Matrix3, Vector3};

fn main() {
    // Change this var to add area
    let target_area = 1.414213;//temp.sqrt();//1.414213;//temp.sqrt();

    // This is the acuracy of the calculated angles
    let step = 0.00000000000002;

    // Range of angles a (rotation in x) and b (rotation in y)
    let temp:f64 = 3.0;
    let mut bmax:f64 = PI as f64/4.0 as f64;
    let mut bmin:f64 = 0.0;
    let temp:f64 = 2.0;
    let mut amax:f64 = (temp.sqrt()/2.0).atan();
    let mut amin:f64 = 0.0;
    let mut angles:Vec<f64> = Vec::new();

    // calculate the are at the max for b
        // b = PI/4
        // a = 0.61540309
    // if it is higer modify a.
        // check area for both maxs
        // if the area is lower go to half of a
        // if the area is higher return error
    // if it is lower do bynary seach with b = max and a = 0

    let mut area = calc_area(amin, bmax);
    println!("area: {area}");

    // if area only requires rotation in b
    if target_area <= area {
        loop {
            let bmid = bmin + (bmax - bmin)/2.0;
            area = calc_area(amin, bmid);
            println!("area: {area}");
            // if area is around target_area, return
            if (target_area >= area - step) && (target_area <= area + step) {
                angles.push(amin);
                angles.push(bmid);
                break;
            }

            // if target area is less than current, make max = mid
            else if  target_area < area - step {
                bmax = bmid;
            }

            // if taget area is greater than current, make min = mid
            else if target_area > area + step {
                bmin = bmid;
            }
        }
    }
    // if target area is bigger than area(0, bmax) do binary seach with bmax to find a
    else {
        loop {
            let amid = amin + (amax - amin)/2.0;
            area = calc_area(amid, bmax);
            println!("area: {area}");            
            // if area is around target_area, return
            if (target_area >= area - step) && (target_area <= area + step) {
                angles.push(amid);
                angles.push(bmax);
                break;
            }

            // if target area is less than current, make max = mid
            else if target_area < area - step {
                amax = amid;
            }

            // if taget area is greater than current, make min = mid
            else if target_area > area + step {
                amin = amid;
            }
        }
    }

    // print angles
    for angle in &angles {
        println!("angle: {angle}");
    }

    // calculate faces
    // multiply the original center by the rotation matrix 
    let o_faces0 = Vector3::new(0.5, 0.0, 0.0);
    let o_faces1 = Vector3::new(0.0, 0.5, 0.0);
    let o_faces2 = Vector3::new(0.0, 0.0, 0.5);
    let rotation_matrix = Matrix3::new(
        angles[1].cos(), 0.0, angles[1].sin(),
        angles[0].sin()*angles[1].sin(), angles[0].cos(), -angles[0].sin()*angles[1].cos(),
        -angles[1].sin()*angles[0].cos(), angles[0].sin(), angles[0].cos()*angles[1].cos(),
    );

    // print report
    println!("Case1: ");
    let product = rotation_matrix * o_faces0;
    println!("{}, {}, {}", product[0], product[1], product[2]);
    let product = rotation_matrix * o_faces1;
    println!("{}, {}, {}", product[0], product[1], product[2]);
    let product = rotation_matrix * o_faces2;
    println!("{}, {}, {}", product[0], product[1], product[2]);
}

// function to calculate area in km^2
// input: the two angles of rotation on the x and y axis
fn calc_area(a:f64, b:f64) -> f64 { 
    // sinα+cosα(cosβ+sinβ)
    // a is the rotation by x in radians
    // b is the rotation by y in radians
    // we discard Z because it does not change the projection
    a.sin() + a.cos() * (b.cos() + b.sin()) 
}
