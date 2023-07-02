use proconio::input;

fn main() {
    println!("Game start!");
    let mut left = 20; let mut right = 36;
    for _i in 0 .. 4{
        let mid = (left + right) / 2;

        println!("Is the age less than {} ? (yes/no)", mid);
        input! {
            ans: String
        }

        if ans == "yes" {
            right = mid;
        } else {
            left = mid;
        } 
    }

    println!("The age is {} !", left);
}
