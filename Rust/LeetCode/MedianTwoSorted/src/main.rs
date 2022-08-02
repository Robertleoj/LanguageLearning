
/*
Algo: 
    Start with larger array, call it A, and the other B.

    Binary search A. 
        For each run of that binary search, say we have element a. 
        Binary search B to find how many element in B are below and above a.
            Let's say we have n1 and n2, and m1 and m2 elements on the left and right of a in A and B, respectively.
            then, if n1 + m1 = n2 + m2, we have found the median. 
            if n1 + m1 = n2 + m2 + 1, then (a + min(A[idx(a) + 1], B[idx(b) + 1])) / 2 is the median.
            if n1 + m1 = n2 + m2 + 2, then min(a, b) is the median.

    binary searching A is log(n), and for each of those, we have log(m), 
    which makes log(n)*log(m) = log(n + m)
 */


struct Solution;

use std::cmp;
impl Solution {


    fn bin_search(arr : &Vec<i32>, val: i32) -> (usize, usize, usize) {
        let (mut min, mut max) = (0, arr.len());

        loop {
            if min == max {
                return (min, min, arr.len() - min);
            }

            let mid = (max + min) / 2;

            if arr[mid] > val {
                max = mid;
            } else {
                min = mid + 1;
            }
        }
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {

        let (mut B, mut A) = if nums1.len() > nums2.len() { (nums1, nums2)} else {(nums2, nums1)};
        'a: loop {
            std::mem::swap(&mut A, &mut B);

            let (mut amin, mut amax) = (0, A.len() - 1);
            let mut prevmid = amax + 1;

            loop {
                let mid = (amin + amax) / 2;
                if mid == prevmid { continue 'a;}

                let a = A[mid];

                // define bidx as the index where no element below (not including) this index is larger than a
                let (bidx, below, above) = Self::bin_search(&B, a);

                let on_left = below + mid;
                let on_right = above + (A.len() - 1 - mid);


                if on_left == on_right {
                    return a as f64;
                }

                println!("amin: {amin} mid: {mid} amax: {amax} ");
                println!("on_left: {on_left} on_right: {on_right}");
                println!("a: {a}");
                if on_right > on_left && on_right - on_left <= 2{
                    let mut minr;
                    if mid == A.len() - 1 {minr= B[bidx];}
                    else if bidx == B.len() {minr = A[mid + 1];}
                    else {minr = cmp::min(A[mid + 1], B[bidx]);}

                    if on_left + 1 == on_right {
                        return (a + minr) as f64 / 2.;
                    } else if on_left + 2 == on_right {
                        return minr as f64;
                    } 
                }

                if on_left > on_right && on_left - on_right <= 2{
                    let mut maxl;
                    if mid == 0 {maxl= B[bidx - 1];}
                    else if bidx == 0 {maxl = A[mid - 1];}
                    else {maxl = cmp::max(A[mid - 1], B[bidx - 1]);}

                    if on_left == on_right + 1 {
                        return (a + maxl) as f64 / 2.;
                    } else if on_left -2 == on_right {
                        return maxl as f64;
                    } 
                }

                if on_left > on_right && on_left - on_right == 1 && mid == 0{
                    return (a + B.last().unwrap()) as f64 / 2.;
                }


                if (on_left <= on_right){
                    amin = mid + 1;
                } else {
                    amax = mid;
                }
                prevmid = mid;
            }
        }
    }
}


fn main() {
    // let v1 = vec![0,0,0,0,0];
    // let v2 = vec![-1,0,0,0,0,0,1];

    let v1 = vec![1,2];
    let v2 = vec![-1,3];
    // let v1 = vec![1, 3, 7, 8];
    // let v2 = vec![2, 9, 10];
    println!("{}", Solution::find_median_sorted_arrays(v1, v2));

}
