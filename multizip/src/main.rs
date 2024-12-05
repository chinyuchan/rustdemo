use itertools::multizip;

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let v2 = vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    let v3 = vec![21, 22, 23, 24, 25, 26, 27, 28, 29, 30];
    let v4 = vec![31, 32, 33, 34, 35, 36, 37, 38, 39, 40];
    let vs = v.chunks(4).map(|x| x.into()).collect::<Vec<Vec<i32>>>();
    let vs2 = v2.chunks(4).map(|x| x.into()).collect::<Vec<Vec<i32>>>();
    let vs3 = v3.chunks(4).map(|x| x.into()).collect::<Vec<Vec<i32>>>();
    let vs4 = v4.chunks(4).map(|x| x.into()).collect::<Vec<Vec<i32>>>();

    for (i, (x, y, z, u)) in multizip((vs, vs2, vs3, vs4)).enumerate() {
        println!("================ {} ================", i);
        println!("{:?},{:?},{:?},{:?}", x, y, z, u);
    }
}
