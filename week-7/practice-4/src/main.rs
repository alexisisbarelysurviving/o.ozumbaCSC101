fn main() {
    let name = vec! ["mary","sam","sally","greg","ade","mark","june","ife"];
    let age = vec! [16,17,18,19,20,21,22,23];
    print!("\n Age allocation:\n");
    for i in 0..age.len()
    {
        print!("{} is {} years old\n",name[i],age[i]);
    }
}
