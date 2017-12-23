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

fn main() {
    let input:Vec<i32> = input::str_to_i32_vec_split(input::file_to_string("day10-input"), &",".to_owned());

    let mut vec = Vec::with_capacity(256);
    for i in 0..256 {
        vec.push(i);
    }

    let mut cur_pos:usize = 0;
    let mut skip_size:usize = 0;
    for i in input {
        rev(&mut vec, cur_pos, i as usize);
        cur_pos = (cur_pos + i as usize + skip_size) % 256;
        skip_size += 1;
    }

    println!("answer day10 part1 = {0}", vec[0] * vec[1]);

}
