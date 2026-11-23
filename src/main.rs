fn main() {
    let x = 121;
    let mut found = true;

    if x < 0 {
        println!("the integer is not palindrome");
    } else {
        let s = x.to_string(); // convert into string
        let arr: Vec<char> = s.chars().collect();
        let len = arr.len();

        //    for i in (0..arr.len()).rev()
        for i in 0..len / 2 {
            if arr[i] != arr[len - 1 - i] {
                found = false;
                break;
            }
        }

        if found == false {
            println!("the integer is not palindrome");
    
        } else {
            println!("the integer is palindrome");
                   
        }
    }
}
