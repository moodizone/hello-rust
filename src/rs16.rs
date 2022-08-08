struct Student {
    name: String,
    locker: Option<i32>,
}

pub fn _fn() {
    let students: Vec<Student> = vec![
        Student {
            name: "student with locker".to_owned(),
            locker: Some(123),
        },
        Student {
            name: "student without locker".to_owned(),
            locker: None,
        },
    ];

    for student in students {
        match student.locker {
            Some(amount) => println!("locker number is {:?}", amount),
            None => println!("No number provided"),
        }
    }
}
