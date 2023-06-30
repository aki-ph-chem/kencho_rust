use proconio::input; 

fn main() {
    input! {
        n: usize,
        v: i32,
        array: [i32; n],
    }

    let mut is_exist = false;
    let mut index_of_v = 0;
    for (index,value) in array.iter().enumerate() {
        if *value == v {
            is_exist = true;
            index_of_v = index;
            break;
        } 
    }

    if is_exist {
        println!("Yes {}", index_of_v);
    } else {
        println!("No");
    }
}
