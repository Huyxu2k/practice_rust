use  std::{prelude::*,io, f32::consts::PI};

//
//s - diện tích
//c - chu vi

fn s_rectangle(length:i32,width:i32) ->i32
{
   length*width
}
fn c_rectangle(length:i32,width:i32) ->i32
{
   (length+ width)*2
}
fn s_square(length:i32)->i32
{
    length*length
}
fn c_square(length:i32)->i32
{
    length*4
}
fn s_circle(radius:f32)->f32
{
    radius.powf(2.0)*PI
}
fn c_circle(radius:f32)->f32
{
    radius*3.14*2.0
}
fn to_int(values :String)->i32
{
    let i = match values.replace("\n","").replace("\r","").parse::<i32>() {
        Ok(i) => i,
        Err(_e) => 0

      };
      i
}
fn main()
{
    let mut input=String::new();
    println!("-------------------------MAIN----------------------");
    println!("1.Tính chu vi hình chữ nhật.");
    println!("2.Tính chu vi hình vuông.");
    println!("3.Tính chu vi hình trong.");
    println!("Nhập lựa chọn của bạn ......");
    let stdin=io::stdin();
    stdin.read_line(&mut input);
    let input_int=to_int(input);

    match input_int {
        1=>{
            let mut length=String::new();
            let mut width=String::new();
            println!("Nhập chiều dài:");
            stdin.read_line(&mut length);
            println!("Nhập chiều rộng:");
            stdin.read_line(&mut width);
            let length_int=to_int(length);
            let width_int=to_int(width);
            let result =c_rectangle(length_int,width_int);
            println!("Chu vi của hình chữ nhật có chiều dài {0} và chiều rộng {1} là {2}",length_int,width_int,result);
        }
        2=>{
            println!("Nhập cạnh của hình vuông:");
            let mut length=String::new();
            stdin.read_line(&mut length);
            let length_int=to_int(length);
            let result=c_square(length_int);
            println!("Chu vi của hình vuông có cạnh  {0} là {1}",length_int,result);
        }
        3=>{
            let mut radius=String::new();
            println!("Nhập bán kính của hình tròn:");
            stdin.read_line(&mut radius);
            let radius_int=to_int(radius);
            let result=c_square(radius_int);
            println!("Chu vi của hình vuông có cạnh  {0} là {1}",radius_int,result);
        }
        _=>println!("lựa chọn không phù hợp !")     
    }
}