#[macro_use]
extern crate mysql;
use std::io;

#[derive(Debug, PartialEq, Eq)]
struct Student {
    sid: i32,
    name: Option<String>,
    age: Option<String>,
    email: Option<String>,
}

fn main() {
    println!("enter your sid");
    let mut sid = String::new();
    io::stdin()
        .read_line(&mut sid)
        .expect("failed to read  a line");
    let sid: i32 = sid.trim().parse().unwrap();

    println!("enter your name");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("failed to read  a line");
    let name: String = name.trim().parse().unwrap();

    println!("enter your age");
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("failed to read  a line");
    let age: String = age.trim().parse().unwrap();

    println!("enter your email");
    let mut email = String::new();
    io::stdin()
        .read_line(&mut email)
        .expect("failed to read  a line");
    let email: String = email.trim().parse().unwrap();

    let student = Student {
        sid: sid,
        name: Some(name),
        age: Some(age),
        email: Some(email),
    };

    insert(student);
    fetch();
}

fn insert(student: Student) {
    let pool = mysql::Pool::new("mysql://root:@localhost:3325/q3db").unwrap();

    let students = vec![student];

    for mut stmt in pool
        .prepare(
            "INSERT INTO tblstudnet (sid, name, email, age)  VALUES (:sid,:name:, :age, :email)",
        )
        .into_iter()
    {
        //into_iter() method key value pare return in stmt vaiable
        for s in students.iter() {
            stmt.execute(params! {
                "sid"=>s.sid,
                "name"=>&s.name,
                 "age"=>&s.age,
                 "email"=>&s.email,
            })
            .unwrap();
        }
    }
}

fn fetch() {
    let pool = mysql::Pool::new("mysql://root:@localhost:3325/q3db").unwrap();

    let selected_student: Vec<Student> = pool
        .prep_exec("SELECT sid,name,age,email FROM tblstudent", ())
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (sid, name, email, age) = mysql::from_row(row);
                    Student {
                        sid: sid,
                        name: name,
                        age: age,
                        email: email,
                    }
                })
                .collect()
        })
        .unwrap();

    for s in 0..selected_student.len() {
        println!(
            "ID : {} Name : {:#?} Age : {:#?} Email : {:#?}",
            selected_student[s].sid,
            selected_student[s].name,
            selected_student[s].age,
            selected_student[s].email
        );
    }
}

fn update(student: Student) {
    let pool = mysql::Pool::new("mysql://root:@localhost:3325/q3db").unwrap();
    let students = vec![student];

    for mut  stmt in pool
        .prepare("UPDATE tblstudent  set  name=:name , age=:age ,email=:email WHERE sid=:sid")
        .into_iter()
    {
        for s in students.iter() {
            stmt.execute(params! {
                "sid"=>s.sid,
                "name"=>&s.name,
                "age"=>&s.age,
                "email"=>&s.email,
            })
            .unwrap();
        }
    }
}

fn delete(student: Student) {
    let pool = mysql::Pool::new("mysql://root:@localhost:3325/q3db").unwrap();
    let students = vec![student];
    for mut stmt in pool.prepare("DELETE tblstudent WHERE sid=:sid").into_iter() {
        for s in students.iter() {
            stmt.execute(params! {
                "sid"=>s.sid,
                "name"=>&s.name,
                "age"=>&s.age,
                "email"=>&s.email
            })
            .unwrap();
        }
    }
}
