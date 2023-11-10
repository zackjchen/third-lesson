use studentsystem::{Table,Score,Student,Teacher, Course};

fn main() {
    let mut table = Table::default();

    // student **********************************************************************
    let st1 = Student::new(1,"zack", 9,"male");
    let st2 = Student::new(2,"jewell", 2,"male");
    let st3 = Student::new(3,"lijiajia", 9,"female");

    table.add_students(st1);
    table.add_students(st2);
    table.add_students(st3);

    table.del_students(2);
    let td = table.select_students_by_name("lijiajia").unwrap();
    println!("{:?}", td);
    println!("{:?}", table);


    // course **********************************************************************

    let course1 = Course::new(1,"Chinese");
    let course2 = Course::new(2,"English");
    let course3 = Course::new(3,"Rust");

    table.add_courses(course1);
    table.add_courses(course2);
    table.add_courses(course3);

    table.del_courses(2);
    let language = table.select_courses_by_name("Rust").unwrap();
    println!("{:?}", language);
    println!("{:?}", table);


    // teacher **********************************************************************

    let t1 = Teacher::new(1,"马化腾", 9);
    let t2 = Teacher::new(2,"马云", 2);
    let t3 = Teacher::new(3,"马斯克", 9);

    table.add_teachers(t1);
    table.add_teachers(t2);
    table.add_teachers(t3);

    table.del_teachers(2);
    let tt = table.select_teachers_by_name("马化腾").unwrap();
    println!("{:?}", tt);
    println!("{:?}", table);


    // score **********************************************************************

    let sc1 = Score::new(1,"zack", 1,99);
    let sc2 = Score::new(2,"jewell", 2,88);
    let sc3 = Score::new(3,"lijiajia", 3,100);

    table.add_scores(sc1);
    table.add_scores(sc2);
    table.add_scores(sc3);

    table.del_scores(2);
    let td = table.select_scores_by_name("lijiajia").unwrap();
    println!("{:?}", td);
    println!("{:?}", table)

}

