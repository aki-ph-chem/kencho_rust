fn check(s: &str) -> bool {
    let s_chars:Vec<char> = s.chars().collect();
    let mut state = vec![];
    let mut result = vec![];

    for i in 0..s.len() {
        if s_chars[i] == '(' {
            state.push(i);
        } else {
            match state.pop() {
                Some(t) => {result.push((t, i));},
                None => {
                    println!("Error");
                    return false;
                }
            }
        }
    } 

    // 結果を出力
    
    // スタックが空 -> カッコが過剰
    if !state.is_empty() {
        println!("too many");
        return false;
    }

    // 整合している場合
    result.sort();
    for i in 0..result.len() {
        if i != 0 {
            print!(", ");
        }
        print!("( {},{} )", result[i].0, result[i].1);
    }
    println!("");

    true
}

fn main() {
    let case_1 = "((()())())";
    let case_2 = "())";
    let case_3 = "(()";

    check(case_1);
    check(case_2);
    check(case_3);
}
