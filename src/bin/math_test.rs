use  std::io;

fn sum (a:i32,b:i32)->i32{
  a+b
}
fn displayNumber(result :i32)
{
    println!("Values is {:?}",result);
}
fn displayChacarter(result :String)
{
    println!("Values is {:?}",result);
}


fn main()->io::Result<()>{

    let mut input=String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input);
    displayChacarter(input.to_string());

    Ok(())
}