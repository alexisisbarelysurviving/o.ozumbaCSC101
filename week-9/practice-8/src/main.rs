struct employee {
    ceo:String,
    company:String,
    age:u32
}

fn main() {
    let emp1 = employee {
        company:String::from("Microsoft"),
        ceo:String::from("Bill Gates"),
        age:56
    };
    let emp1 = employee {
        company:String::from("Microsoft"),
        ceo:String::from("Bill Gates"),
        age:56
    };
    let emp2 = employee {
        company:String::from("Google"),
        ceo:String::from("sundai punch"),
        age:51
    };
    display(emp1);
    display(emp2);

    fn display (emp:employee){
        println!("name is :{} company is {} age is {}",emp.ceo, emp.company, emp.age);
    }
}
