fn main(){
    let my_string: &str = "abcd:1234";
    let new_string: &str = my_string.split(":").collect::<Vec<&str>>()[1];
    println!("{:#?}", new_string)
}