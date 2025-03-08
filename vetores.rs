fn main(){
   // let v = vec![23, 24, 25];
    //println!("{}", v[1]);
    let mut v: Vec<u32> = Vec::new();
    v.push(23);
    v.push(24);
    v.push(28);
    v.push(27);
    v.push(22);
    v.push(20);
    v.remove(0);

    println!("{}", v[0]);

}