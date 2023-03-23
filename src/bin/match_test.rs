use std::{io};

fn main() ->io::Result<()>
{
    let mut input=String::new();
    let stdin= io::stdin();
    println!("Nhập 1 số nguyên:");
    //check giá trị nhập vào

    stdin.read_line(&mut input);
    let my_int =to_int(input);
// loop {

//     if my_int>11 || my_int<0 
//     {
//       break;
//     }
    match my_int {
        1 => println!("Đây là số 1."),
        2 => println!("Đây là số 2."),
        3 => println!("Đây là số 3."),
        4 => println!("Đây là số 4."),
        5 => println!("Đây là số 5."),
        6 => println!("Đây là số 6."),
        7 => println!("Đây là số 7."),
        8 => println!("Đây là số 8."),
        9 => println!("Đây là số 9."),
        _ => println!("Ngoài phạm vi .")
        
  //  }
}


Ok(())
}

fn to_int(values :String)->i32
{
    let i = match values.replace("\n","").replace("\r","").parse::<i32>() {
        Ok(i) => i,
        Err(_e) => 0

      };
      i
}
  