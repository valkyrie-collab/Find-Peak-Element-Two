use std::{
    io::{
        self,
        Write,
    },
};

fn find_peak(arr: &Vec<Vec<i32>>, mut left: i32, mut right: i32) -> [i32; 2] {
    let mut mid: i32;

    while left <= right {
        mid = left + (right - left) / 2;
        let sub_arr: &Vec<i32> = &arr[mid as usize];
        let mut fxi: i32 = -1;
        let mut max: i32 = i32::MIN;

        for (xi, &xe) in sub_arr.iter().enumerate() {
            max = if xe > max {fxi = xi as i32; xe} else {max};
        }

        let up_val: i32;
        let down_val: i32;

        if mid - 1 == -1 {
            up_val = -1;
        } else {
            up_val = arr[mid as usize - 1][fxi as usize];
        }

        if mid + 1 == arr.len() as i32 {
            down_val = -1;
        } else {
            down_val = arr[mid as usize + 1][fxi as usize];
        }

        // println!("up_val: {} | down_val: {} | max: {}", up_val, down_val, max);

        if max > up_val && max > down_val {
            return [mid, fxi];
        } else if max < up_val && max > down_val {
            right = mid - 1;
        } else if max > up_val && max < down_val {
            left = mid + 1;
        } else {
            left = mid + 1;
        }

    }

    [-1, -1]
}

fn main() {
    let mut buf: String = String::new();

    print!("Enter the size of the array: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();

    let size: usize = if let Ok(x) = buf.trim().parse() {x} else {0};
    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(size);

    for _xi in 0..size {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let child_size: usize = if let Ok(x) = buf.trim().parse() {x} else {0};
        let mut row: Vec<i32> = vec![-1; child_size];

        for xx in 0..child_size {
            buf.clear();
            io::stdin().read_line(&mut buf).unwrap();
            row[xx] = if let Ok(x) = buf.trim().parse() {x} else {-1}
        }

        arr.push(row);
    }

    // let arr: Vec<Vec<i32>> = vec![vec![1,2,3,4,5,6,7,8],vec![2,3,4,5,6,7,8,9],vec![3,4,5,6,7,8,9,10],vec![4,5,6,7,8,9,10,11]];
    let coordinates: [i32; 2] = find_peak(&arr, 0, (arr.len() - 1) as i32);
    println!("The coordinates are: {:?}", coordinates);
}
