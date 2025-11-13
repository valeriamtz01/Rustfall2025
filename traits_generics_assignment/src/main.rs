trait ShowInfo
{
    fn show_info(&self);
}

struct Undergrad
{
    name:String,
    major: String,
    gpa: f64,
}

struct Grad
{
    name: String,
    major: String,
    gpa: f64,
    thesis: String,
}

impl ShowInfo for Undergrad
{
    fn show_info(&self)
    {
        println!("Undergrad Student: {} | Major: {} | GPA: {}\n", self.name, self.major, self.gpa);
    }
}

impl ShowInfo for Grad
{
    fn show_info(&self)
    {
        println!("Grad Student: {} | Major: {} | GPA: {} | Thesis: {}\n", self.name, self.major, self.gpa, self.thesis);
    } 
}

struct Enrollment<T:ShowInfo>
{
    students:Vec<T>,
}

impl<T: ShowInfo> Enrollment<T>
{
    fn new() -> Self{
        Enrollment{students: Vec::new()}
    }

    fn add_student(&mut self, students: T) {
        self.students.push(students);
    }

    fn show_all(&self)
    {
        for s in &self.students{
            s.show_info();
        }
    }
}

fn main ()
{
    //create students
    let u1 = Undergrad
    {
        name: "Ana".to_string(),
        major: "Computer Science".to_string(),
        gpa: 3.8,
    };

    let g1 = Grad
    {
        name: "Juan".to_string(),
        major: "Computer Engineering".to_string(),
        gpa: 3.9,
        thesis: "Machine Learning Models".to_string(),
    };

    let mut undergrad_enrollment = Enrollment::<Undergrad>::new();
    undergrad_enrollment.add_student(u1);

    let mut grad_enrollment = Enrollment::<Grad>::new();
    grad_enrollment.add_student(g1);

    println!("Undegraduate Enrollment: ");
    undergrad_enrollment.show_all();

    println!("Graduate Enrollment: ");
    grad_enrollment.show_all();
}