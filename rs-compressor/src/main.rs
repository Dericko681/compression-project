use algos::le::compress;

mod algos;
fn main(){
let string = b"AAABBBCCCCCDDDDE";
let output = compress(string);
println!("{:?}", output);

}