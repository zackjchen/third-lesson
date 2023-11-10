use studentsystem::{Table,Score,Student,Teacher};

fn main() {
    let mut table = Table::default();
    let st1 = Student::new(1,"zack", 9,"male");
    let st2 = Student::new(2,"jewell", 2,"male");
    let st3 = Student::new(3,"lijiajia", 9,"female");

    table.add_students(st1);
    table.add_students(st2);
    table.add_students(st3);

    table.del_students(2);
    let td = table.select_students_by_name("lijiajia").unwrap();
    println!("{:?}", td);
    println!("{:?}", table)

}

