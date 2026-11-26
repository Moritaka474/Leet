use std::io;
fn main()
{
    let mut x = String::new();
    let mut alice_turn = true;

    println!("Enter the number: ");
    io::stdin().read_line(&mut x).expect("Error");

    let mut stone: u32 = x.trim().parse().expect("Error");

    for i in (1..=10).rev()
    {
        if stone >= i
        {
            stone -= i;
            alice_turn = !alice_turn;
        }
        else
        {
            break;
        }
    }

    if alice_turn
    {
        println!("Bob wins");
    }
    else
    {
        println!("Alice wins");
    }
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