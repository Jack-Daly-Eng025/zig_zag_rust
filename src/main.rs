pub struct Solution;


impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if s.len()<=1 ||  num_rows <= 1{
        return s;
        }
        let n_s = num_rows as usize;
        let mut r = vec![String::default(); n_s];
        let mut c_r = 0;
        let mut d_p = false;
        s.chars().for_each(|c|{
            r[c_r].push(c);
            if c_r == 0 || c_r == (n_s - 1){
                d_p= !d_p;
                }
            if d_p {
                c_r += 1;
                }
            else{
                c_r -= 1;}
                });
        r.concat()
    }
}


fn main(){
    let string_s = "supatrooper".to_string();

    let rows = 4;

    let mut zig_zag = Solution::convert(string_s, rows);

    println!("{zig_zag}");

    zig_zag = Solution::convert("banana".to_string(), 3);

    println!("{zig_zag}");
}