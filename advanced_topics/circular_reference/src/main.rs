// circular reference is where two objects can reference one another
// this can be expressed as a many to many relationship

// here we are making a struct with a lifeline generic
// because we need to introduce a reference

// use std::rc::Rc;
// use std::cell::RefCell;
//
// struct Student<'a> {
//     name: String,
//     courses: Vec<&'a Course<'a>>,
// }
//
// impl<'a> Student<'a> {
//     fn new(name: &str) -> Student<'a> {
//         Student {
//             name: name.into(),
//             courses: Vec::new(),
//         }
//     }
// }
//
// struct Course<'a> {
//     name: String,
//     students: Vec<&'a Student<'a>>,
// }
//
// impl<'a> Course<'a> {
//     fn new(name: &str) -> Course<'a> {
//         Course {
//             name: name.into,
//             students: Vec::new(),
//         }
//     }
//
//     fn add_student(&'a mut self, student: &'a mut Student<'a>) {
//         student.courses.push(self);
//         self.students.push(student);
//         // here we have a problem with mutability and borrowing
//         // we can use RefCell
//     }
// }
//
// // using RefCell
//
// struct RefCellStudent {
//     name: String,
//     courses: Vec<Rc<RefCell<RefCellCourse>>>,
// }
//
// impl RefCellStudent {
//     fn new(name: &str) -> RefCellStudent {
//         RefCellStudent {
//             name: name.into(),
//             courses: Vec::new(),
//         }
//     }
// }
//
// struct RefCellCourse {
//     name: String,
//     students: Vec<Rc, RefCell<RefCellStudent>>,
// }
//
// // need to access the borrow_mut function available on RefCell
// // in order to secure the drop and implement circular reference
// impl RefCellCourse {
//     fn new(name: &str) -> RefCellCourse {
//         RefCellCourse {
//             name: name.into(),
//             students: Vec::new(),
//         }
//     }
//     fn add_student(
//         course: Rc<RefCell<RefCellCourse>>,
//         student: Rc<RefCell<RefCellStudent>>,
//     ) {
//         // here we are using clone, but every time you have to do this
//         // you will need to use clone as well
//         student.borrow_mut().courses.push(course.clone());
//         course.borrow_mut().students.push(student);
//     }
// }

// the most eloquent way of solving this is to use a relational table
// that will store the associations
// struct Enrollment {
// course: Course
// student: Student
// }

struct EnrStudent {
    name: String,
}

impl EnrStudent {
    fn courses(&self, platform: Platform) -> Vec<String> {
        platform.enrollments
            .iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}
struct EnrCourse {
    name: String,
}

struct Enrollment<'a> {
    student: &'a EnrStudent,
    course: &'a EnrCourse,
}

impl<'a> Enrollment<'a> {
    fn new(student: &'a EnrStudent, course: &'a EnrCourse) -> Enrollment<'a> {
        Enrollment { student, course }
    }
}

struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>
}

impl<'a> Platform<'a> {
    fn new() -> Platform<'a> {
        Platform {
            enrollments: Vec::new()
        }
    }
    fn enroll(&mut self, student: &'a EnrStudent, course: &'a EnrCourse) {
        self.enrollments.push(
            Enrollment::new(student, course)
        )
    }
}

fn main() {
    println!("Hello, world!");
    // let john = Student::new("John");
    // // this is a drop problem in this way
    // let course = Course::new("Rust Course");
    // // the dropping problem is based on life time, because
    // // both objects use circular reference
    // course.add_student(john);
    //
    // let jane = Rc::new(
    //     RefCell::new(
    //         RefCellStudent("Jane")
    //     )
    // );
    //
    // let refCourse = Rc::new(
    //     RefCell::new(
    //         RefCellCourse("Rust Course")
    //     )
    // );
    // // still horrendous, can make better
    let john = EnrStudent {
        name: "John".into()
    };
    let course = EnrCourse {
        name: "Intro to Rust".into()
    };

    let mut p = Platform::new();
    p.enroll(&john, &course);
    for c in john.courses(p) {
        println!("John is taking {}", c);
    }
}
