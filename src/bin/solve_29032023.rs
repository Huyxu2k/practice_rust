/*
29032023
-Given a string S. The task is to count the number of substrings which contains equal number of lowercase and uppercase letters.
-Example 1:

Input:
S = "gEEk"
Output: 3
Explanation: There are 3 substrings of
the given string which satisfy the
condition. They are "gE", "gEEk" and "Ek".
-Example 2:

Input:
S = "WomensDAY"
Output: 4
Explanation: There are 4 substrings of
the given string which satisfy the
condition. They are "Wo", "ensDAY",
"nsDA" and "sD"
*/

use std::{collections::HashMap, ops::IndexMut, vec};

/* not use
fn countSubstring(s: String) -> i32 {
    let mut vec=subString(s);
    let mut result=0;
   for i in vec {
       if check(i){
        result=result.clone()+1;
       }
   }
   result
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}
fn Input() -> String {
    "test".into()
}
//A,B,C,D,....
fn count_char_to_int_65_90(str: String) -> i32 {
    let mut n = 0;
    for i in str.chars() {
        if (i as i32) >= 65 && (i as i32) <= 90 {
            n = n.clone() + 1;
        }
    }
    n
}
//a,b,c,d,....
fn count_char_to_int_97_122(str: String) -> i32 {
    let mut n = 0;
    for i in str.chars() {
        if (i as i32) >= 97 && (i as i32) <= 122 {
            n = n.clone() + 1;
        }
    }
    n
}
fn check(str: String) -> bool {
    let n1 = count_char_to_int_97_122(str.clone());
    let n2 = count_char_to_int_65_90(str.clone());
    if n1 == n2 {
        true
    } else {
        false
    }
}
 */

//tìm tất cả các chuỗi con
fn subString(str: String) -> Vec<String> {
    let mut vec = Vec::new();
    let leng = str.len() as i32;
    for i in 1..leng {
       
    }
    vec
}
//các chữ hoa -> -1 chữ  thường -> 1
fn convert_char_int(s: &str) -> Vec<i32> {
    let mut vec_0_1 = Vec::new();
    for i in s.chars() {
        if i as i32 >= 65 && i as i32 <= 90 {
            vec_0_1.push(-1);
        } else if i as i32 >= 97 && i as i32 <= 122 {
            vec_0_1.push(1);
        }
    }
    vec_0_1
}
//=>  nếu chuỗi thỏa mãn điều kiện có số chữ hoa = số chữ thường thì tổng chuỗi  đấy phải  =0
fn countSubstring(v: Vec<Vec<i32>>) -> i32 {
    let mut n = 0;
    for i in v {
        if i.len() == 1 {
            continue;
        }
        let mut sum_i = 0;
        for j in i {
            sum_i += j;
        }
        if sum_i == 0 {
            n += 1;
        }
    }
    n
}
fn main() {
    let s = String::from("gEEk");
    println!("Vec {:?}", convert_char_int(&s.to_string()));
}
