use std::collections::HashMap;

fn main() {
    let mut v = vec![1,7,87,4,54,3,2,32,43,533,4,34,436,5,567,7,347,8,9, 500, 7];
    let size = count(&v);
    println!("Size of vector is {}", size);
    sort(&mut v, size);
    println!("Sorted Vector is:");
    print_vector(&v);
    println!("Median of vector is: {}", find_median(&v, size));
    println!("Mode of vector is: {}", find_mode(&v, size));

}

fn count(v: &Vec<i32>) -> i32{
    let mut count = 0;
    for _ in v {
        count+=1;
    }
    return count;
}

fn sort(v: &mut Vec<i32>, count: i32) {
    for i in (0..count) {
        let mut smallest = i;
        for j in (i..count) {
            if (v[smallest as usize] > v[j as usize]){
                smallest = j;
            }
        }
        let temp = v[smallest as usize];
        v[smallest as usize] = v[i as usize];
        v[i as usize] = temp;  
    }
}

fn print_vector(v: &Vec<i32>) {
    for i in v {
        print!("{} ", i);
    }
    println!("");
}

fn find_median(v: &Vec<i32>,count: i32) -> i32 {
    let median_index = count/2;
    if count %2 == 0 {
        return (v[median_index as usize] + v[(median_index+1) as usize])/2;
    } else {
        return v[median_index as usize];
    }
    
}

fn find_mode(v: &Vec<i32>, count: i32) -> &i32 {
    let mut hash_map = HashMap::new();

    for i in v {
        let num = hash_map.entry(i).or_insert(0);
        *num +=1;
    }

    let mut mode = &v[0];
    let mut mode_count = hash_map.get(&mode);
    for i in v {
        let count = hash_map.get(i);
        if (count > mode_count) {
            mode_count = count;
            mode = i;
        }
    }
    return mode;
}