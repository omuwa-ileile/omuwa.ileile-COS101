fn main() {
    //Name vector
    let name = vec!["Peter","Jeremy","Chigo","Emeka","Brian","Elnathan","Ronald","Oshoke"];

    //Age vector
    let age = vec![16,17,17,17,17,16,17,17];

    print!("\nAge allocation:\n");

    //loop to iterate elements in vector
    for i in 0..age.len(){
        //iteration through i on the vector
        print!("{} is {} years old\n",name[i],age[i]);
    }
}
