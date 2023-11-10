#![allow(warnings)]
#![allow(dead_code)]


#[macro_use]
use curdfn::CURD;


#[derive(Debug)]
pub struct Student{
    id: u32,
    name: String,
    grade: i8 ,// 年级
    sex: String
}
impl Student {
    pub fn new(id: u32, name: &str, grade: i8, sex: &str) -> Self{
        Student{id, name:name.into(), grade, sex: sex.into()}
    }
}

#[derive(Debug)]
pub struct Course{
    name: String,
    id: u32,
}

impl Course {
    pub fn new(id: u32,name: &str) -> Self{
        Course{ id, name: name.into()}
    }
}

#[derive(Debug)]
pub struct Teacher{
    name: String,
    id: u32,
    course_id: u32, // 老师教什么课
}

impl Teacher {
    pub fn new(id: u32,name: &str, course_id: u32) -> Self{
        Teacher { name: name.into(), id: id, course_id: course_id }
    }
}

#[derive(Debug)]
pub struct Score {
    student_id: u32,
    student_name: String,
    course_id: u32,
    score: u32,
}


impl Score {
    pub fn new(student_id: u32,studen_name:&str, course_id: u32, score: u32) -> Self{
        Score { student_id, student_name: studen_name.into(), course_id, score } 
    }
}


#[derive(Debug,Default, CURD)]
pub struct Table{
    students: Vec<Student>,
    courses: Vec<Course>,
    teachers: Vec<Teacher>,
    scores: Vec<Score>,
}

impl Table {
//     pub fn new() -> Self{
//         Default::default()
//     }

//     pub fn add_student(&mut self,student: Student){
//         self.students.push(student);
//     }
//     pub fn delete_student(&mut self,name: &str){
//         let mut index:i32 = -1;
//         for (i,st) in self.students.iter().enumerate() {
//             if &st.name == name {
//                 index = i as i32;
//             }
//         }
//         if index>0 {
//             self.students.remove(index as usize);            
//         }else {
//             panic!("connot find the student")
//         }

//     }

    // pub fn select_students_by_name<'a>(&'a self ,name: &str) -> Result<&'a Student,&str> {
    //     for (i,row) in self.students.iter().enumerate(){
    //         if row.name == name{
    //             return Ok(&self.students[i])
    //         }else{
    //             Err("没找到")
    //         }
    //     }
    // }
}