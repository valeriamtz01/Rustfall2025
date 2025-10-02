
struct Student
{
    name:String,
    major:String,
}

// methods are added by IMPL statement
impl Student
{
    fn new (n:String, m:String) -> Student // static method
    {
        Self
        {
            name: n,
            major: m, 
        }
    }

    fn set_major(&mut self, my_major:String)
    {
        self.major = my_major;
    }

    fn get_major(&self) -> &String
    {
        return &self.major
    }

}


fn main()
{
    let mut my_student = Student::new("Valeria Martinez".to_string(), "CS".to_string());

    print!("Name: {}", my_student.name);
    println!(". Major: {}", my_student.major);

    my_student.set_major("Computer Science".to_string());
    println!("Updated Major: {}", my_student.get_major());
}
