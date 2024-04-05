#![allow(clippy::needless_range_loop)]

use std::mem::swap;

const BUCKETS: usize = 256;
fn radix_sort(a: &mut [u32]) {
    let mut histogram = [[0; BUCKETS]; 4];
    for e in &*a {
        let mut num = *e as usize;

        for i in 0..4 {
            let byte = num & 0xFF;
            histogram[i][byte] += 1;
            num >>= 8;
        }
    }
    for i in 0..4 {
        for j in 1..BUCKETS {
            histogram[i][j] += histogram[i][j - 1];
        }
    }
    let mut sort_from = a.to_vec();
    let mut sort_to: Vec<u32> = vec![0; a.len()];
    for i in 0..4 {
        let mut offsets = histogram[i];
        for e in sort_from.iter() {
            let byte = ((e >> (8 * i)) & 0xFF) as usize;
            offsets[byte] -= 1;
            let pos = offsets[byte];
            sort_to[pos] = *e;
        }
        swap(&mut sort_to, &mut sort_from);
    }
    for (val, target) in sort_from.into_iter().zip(a) {
        *target = val;
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::radix_sort;
    fn is_sorted(a: &[u32]) -> bool {
        a.windows(2).all(|w| w[0] <= w[1])
    }
    #[test]
    fn sort_4_values() {
        let mut a = [0xcccccccc, 0x88888888, 0x44444444, 0x00000000];
        radix_sort(&mut a);
        assert!(is_sorted(&a));
    }
    #[test]
    fn sort_16_values() {
        let mut a = [
            0xffffffff, 0xeeeeeeee, 0xdddddddd, 0xcccccccc, 0xbbbbbbbb, 0xaaaaaaaa, 0x99999999,
            0x88888888, 0x77777777, 0x66666666, 0x55555555, 0x44444444, 0x33333333, 0x22222222,
            0x11111111, 0x00000000,
        ];
        radix_sort(&mut a);
        assert!(is_sorted(&a));
    }

    #[test]
    fn sort_same_values() {
        let mut a = [0xdeadbeef; 1 << 15];
        radix_sort(&mut a);
        assert!(is_sorted(&a));
    }
}
