

// You are given an array A of size N.
// Let us denote S as the sum of all integers present in the array.
// Among all integers present in the array, find the minimum integer X such that S ≤ N*X.
fn minimumInteger(N:i32,mut vec:Vec<i32>)->i32{
   let mut sum=0;
   let mut result=0;
   //let mut vec_origin=vec.clone();
   for i in vec.clone() {
       sum+=i;
       result=i;
   }

   if vec.clone().len()==0 {
      return  result;
   }
   for i in vec {
       if sum<= N*i{
        result= min(i,result);
       }
   }
   result
}
fn min(n:i32,m:i32)->i32{
    let mut i=m-n;
   if i==0{
     return n;
   }
   else if i>0 {
       return n;
   }
   else {
       return m;
   }
}
fn main(){
  let mut  vec=Vec::new();
   vec.push(1);
   vec.push(2);
   vec.push(3);
   vec.push(5);
   vec.push(7);
   vec.push(11);
   vec.push(13);
 let N=vec.len() as i32;
   println!("OUTPUT ={:?}",minimumInteger(N,vec)); 
}