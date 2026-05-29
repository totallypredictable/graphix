// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

pub fn binpow(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 1;
    }
    let res: i64 = binpow(a, b / 2);
    if b % 2 != 0 {
        return res * res * a;
    } else {
        return res * res;
    }
}
