fn main() {
    let age: i32 = 23;
    let school_student: bool = age < 18;
    let college_student: bool = age >= 18 && age <= 22;
    let working_adult: bool = age > 22 && age < 65;
    let senior_citizen: bool = age >= 65;

    if school_student {
        println!("The person is a school student.");
    } else if college_student {
        println!("The person is a college student.");
    } else if working_adult {
        println!("The person is a working adult.");
    } else if senior_citizen {
        println!("The person is a senior citizen.");
    } 

    let colleg_passed_out = !college_student;
    println!("Is the person a college pass-out? {}", colleg_passed_out);

}
