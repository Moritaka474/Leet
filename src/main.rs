// two sums 


fn main()
{
   let arr = [1,2,3];
   let target = 5;

   let mut first = 0;
   let mut second = 0;

   let mut x = 0;

   'outer: for i in 0..arr.len()
   {
       for j in 0..arr.len()
       {
          if i!=j && target == arr[i]+arr[j]
          {
            first = i;
            second = j;
            x = 1;
            break 'outer;
          }
    
       }

   }

   if x == 1
   {
      println!("output = [{},{}]",first,second);
   }

}