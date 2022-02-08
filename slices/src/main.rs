fn main() {
    let array: [u32; 8] = [6,7,2,3,4,5,6,7];

    let subarray = &array[..3];
    let subarray2 = &array[3..8];
    printer(&subarray);
    printer(subarray2);

    //Create a mutable slice, and mutate it
    let mut array_mut: [u32; 8] = [10,20,30,40,50,60,70,80];
    let sub_array_mut = &mut array_mut[0..5];
    sub_array_mut[2] = 100;
    let sub_array_mut_2 = &mut array_mut[5..8];
    sub_array_mut_2[2] = 100;
    printer(&array_mut);
}

fn printer(array: &[u32]) {

    for item in array.iter().enumerate() {
        println!("{}",item.1);
    }
}


