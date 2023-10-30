// 逆ポーランド記号の計算
fn calc(exp: &str) -> Option<i32> {
    let mut st = vec![];
    for c in exp.chars() {
        if c >= '0' && c <= '9' {
            let number = c as i32 - '0' as i32;
            st.push(number);
        } else {
            let a = match st.pop() {
                Some(number) => number,
                None => panic!("Error"),
            };
            let b = match st.pop() {
                Some(number) => number,
                None => panic!("Error"),
            };

            match c {
                '+' => {
                    st.push(b + a);
                }
                '-' => {
                    st.push(b - a);
                }
                '*' => {
                    st.push(b * a);
                }
                '/' => {
                    st.push(b / a);
                }

                _ => {
                    panic!("invalid operator");
                }
            }
        }
    }

    st.last().copied()
}

// 数値、演算子の間にスペースがある場合に対応
fn calc_with_space(exp: &str) -> Option<i32> {
    let mut st = vec![];
    for c in exp.chars() {
        if c >= '0' && c <= '9' {
            let number = c as i32 - '0' as i32;
            st.push(number);
        } else if c == ' ' {
            continue;
        } else {
            let a = match st.pop() {
                Some(number) => number,
                None => panic!("Error"),
            };
            let b = match st.pop() {
                Some(number) => number,
                None => panic!("Error"),
            };

            match c {
                '+' => {
                    st.push(b + a);
                }
                '-' => {
                    st.push(b - a);
                }
                '*' => {
                    st.push(b * a);
                }
                '/' => {
                    st.push(b / a);
                }
                _ => {
                    panic!("invalid operator");
                }
            }
        }
    }

    st.last().copied()
}

fn main() {
    let case_1 = "34+12-*";
    println!("case_1: {}", calc(&case_1).unwrap());
    let case_2 = "3 4 + 1 2 - *";
    println!("case_2: {}", calc_with_space(&case_2).unwrap());
}
