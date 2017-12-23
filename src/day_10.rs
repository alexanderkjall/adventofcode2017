use std::io::Write;

mod input;

fn rev(vec: &mut Vec<i32>, start: usize, size: usize) {
    let mut holder:Vec<i32> = Vec::with_capacity(size);

    for i in 0..size {
        holder.push(vec[((start + i) % 256) as usize])
    }
    for i in 0..size {
        vec[((start + size - i + 255) % 256) as usize] = holder[i as usize];
    }
}

fn xor(vec: &[i32]) -> u8 {
    let mut val:i32 = 0;
    for v in vec {
        val ^= *v;
    }
    return val as u8;
}

fn knot(mut input:Vec<i32>) -> String {
    input.push(17);
    input.push(31);
    input.push(73);
    input.push(47);
    input.push(23);

    let mut vec:Vec<i32> = Vec::with_capacity(256);
    for i in 0..256 {
        vec.push(i);
    }

    let mut cur_pos:usize = 0;
    let mut skip_size:usize = 0;
    for _ in 0..64 {
        for i in &input {
            rev(&mut vec, cur_pos, *i as usize);
            cur_pos = (cur_pos + *i as usize + skip_size) % 256;
            skip_size += 1;
        }
    }

    let mut s = Vec::new();
    for i in 0..16 {
        write!(&mut s, "{:x}", xor(&vec[i * 16 .. i * 16 + 16])).expect("Unable to write");
    }

    return String::from_utf8(s).unwrap();
}

fn main() {
    let mut input:Vec<i32> = input::str_to_i32_vec_split(input::file_to_string("day10-input"), &",".to_owned());

    let mut vec = Vec::with_capacity(256);
    for i in 0..256 {
        vec.push(i);
    }

    let mut cur_pos:usize = 0;
    let mut skip_size:usize = 0;
    for i in &input {
        rev(&mut vec, cur_pos, *i as usize);
        cur_pos = (cur_pos + *i as usize + skip_size) % 256;
        skip_size += 1;
    }

    println!("answer day10 part1 = {0}", vec[0] * vec[1]);
    println!("answer day10 part2 = {0}", knot(input::file_to_string("day10-input").bytes().map(|x| x as i32).collect::<Vec<i32>>()));
}
