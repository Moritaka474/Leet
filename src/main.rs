
fn main() {
    let l1 = [2, 4, 3];
    let l2 = [5, 6];

    let mut l3: Vec<i32> = Vec::new();
    let mut carry = 0;

    let max_len = l1.len().max(l2.len());

    for i in 0..max_len {
        let a = if i < l1.len() { l1[i] } else { 0 };
        let b = if i < l2.len() { l2[i] } else { 0 };

        let sum = a + b + carry;

        l3.push(sum % 10);
        carry = sum / 10;
    }

    if carry > 0 {
        l3.push(carry);
    }

    println!("{:?}", l3);
}




/* 
// This is a simple program that adds two arrays element-wise and stores the result in a third array.
fn main()
{
    let l1 = [1,2];
    let l2 = [3,4];
    let mut l3 = [0,0];

    let len1 = l1.len();

    for i in 0..len1
    {
        l3[i] = l1[i] + l2[i];
    }

    println!("{:?}", l3);


}
*/