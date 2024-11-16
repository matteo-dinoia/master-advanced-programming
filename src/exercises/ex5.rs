use std::fmt::{Debug, Display};
use rand::{random, thread_rng, Rng};
use crate::exercises::ex5::CarrotState::Burnt;

pub(crate) fn test_all() {
    print(&vec![1,2,3,4]);
    print2(&vec![1,2,3,4]);
    //vec![4.0].print();
    println!("{:?}", Book::default());
    println!("{:?}", Book::default_with_cat(Category::Comedy));

    let mut lib = Library::default();
    lib.populate();
    println!("{:?}", lib);

    let p = Pair(5, "str".to_owned()) + "str";
    println!("{:?}", p);
    println!("{:?}", p * 5);
}

trait Heatable{
    fn cook(&mut self);
}
trait Friable{
    fn cook(&mut self);
}

trait Heater{
    fn heat(&self, food: &mut dyn Heatable);
}
trait Frier{
    fn fry(&self, food: &mut dyn Heatable);
}

struct Oven{}
struct Pan{}

impl Heater for Oven{
    fn heat(&self, food: &mut dyn Heatable) {
       food.cook();
    }
}

impl Frier for Pan{
    fn fry(&self, food: &mut dyn Heatable) {
        food.cook();
    }
}

struct Pie{ready: bool}
struct Carrot{state: CarrotState}

#[derive(Eq, PartialEq)]
enum CarrotState{Raw , Cooked , Fried , Burnt}

trait Edible{
    fn eat(&self);
}

impl Heatable for Pie{
    fn cook(&mut self) {
        if self.ready {
            println!("Yoy burned the pie!");
        }else{
            self.ready = true;
        }
    }
}

impl Heatable for Carrot{
    fn cook(&mut self) {
        if(self.state == CarrotState::Raw){
            self.state = CarrotState::Cooked;
        }else{
            self.state = CarrotState::Burnt;
        }
    }
}

impl Friable for Carrot{
    fn cook(&mut self) {
        if(self.state == CarrotState::Raw){
            self.state = CarrotState::Fried;
        }else{
            self.state = CarrotState::Burnt;
        }
    }
}

impl Edible for Pie{
    fn eat(&self) {
        if self.ready {
            println!("yummy");
        }else {
            println!("you got stomach ache");
        }
    }
}

impl Edible for Carrot{
    fn eat(&self) {
        println!("{}", match self.state {
            CarrotState::Raw => { "mmh, crunchy"}
            CarrotState::Cooked => {"mmh, yummy"}
            CarrotState::Fried => {"mmh, crispy"}
            _ => {"mmh, burnt"}
        });
    }
}




struct Gate<S>{ _s: S }

struct Closed{}
struct Open{}
struct Stopped{reason: String}

trait OpenClosable {
    fn close(self) -> Result<Gate<Closed>, Gate<Stopped>>;
    fn open(self) -> Result<Gate<Open>, Gate<Stopped>>;
}

impl OpenClosable for Gate<Open>{
    fn close(self) -> Result<Gate<Closed>, Gate<Stopped>> {
        if random::<bool>() {
            Ok(Gate{ _s: Closed{} })
        } else{
            Err(Gate{ _s: Stopped{reason: "No luck".to_owned() } })
        }
    }

    fn open(self) -> Result<Gate<Open>, Gate<Stopped>> {
        Err(Gate{ _s: Stopped{reason: "Already open".to_owned() } })
    }
}

impl OpenClosable for Gate<Closed>{
    fn close(self) -> Result<Gate<Closed>, Gate<Stopped>> {
        Err(Gate{ _s: Stopped{reason: "Already closed".to_owned() } })
    }

    fn open(self) -> Result<Gate<Open>, Gate<Stopped>> {
        if random::<bool>() {
            Ok(Gate{ _s: Open{} })
        } else{
            Err(Gate{ _s: Stopped{reason: "No luck".to_owned()} })
        }
    }
}

impl OpenClosable for Gate<Stopped>{
    fn close(self) -> Result<Gate<Closed>, Gate<Stopped>> {
        Ok(Gate{ _s: Closed{} })
    }

    fn open(self) -> Result<Gate<Open>, Gate<Stopped>> {
        Ok(Gate{ _s: Open{} })
    }
}


#[derive(Debug)]
struct Pair(i32, String);

impl std::ops::Add::<i32> for Pair{
    type Output = Pair;

    fn add(self, rhs: i32) -> Self::Output {
        Self(self.0 + rhs, self.1)
    }
}

impl std::ops::Sub::<i32> for Pair{
    type Output = Pair;

    fn sub(self, rhs: i32) -> Self::Output {
        Self(self.0 - rhs, self.1)
    }
}

impl std::ops::Add::<& str> for Pair{
    type Output = Pair;

    fn add(self, rhs: &str) -> Self::Output {
        Self(self.0, self.1 + rhs)
    }
}

impl std::ops::Sub::<&str> for Pair{
    type Output = Pair;

    fn sub(self, rhs: &str) -> Self::Output {
        Self(self.0, self.1.replace(rhs, ""))
    }
}

impl std::ops::Add::<Pair> for Pair{
    type Output = Pair;

    fn add(self, rhs: Pair) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + &rhs.1)
    }
}

impl std::ops::Sub::<Pair> for Pair{
    type Output = Pair;

    fn sub(self, rhs: Pair) -> Self::Output {
        Self(self.0 - rhs.0, self.1.replace(&rhs.1, ""))
    }
}

impl std::ops::Mul::<i32> for Pair{
    type Output = Pair;

    fn mul(self, rhs: i32) -> Self::Output {
        let mut a = 1;
        let mut b = "".to_owned();
        for i in 0..rhs{
            a = a * self.0;
            b = b + &self.1;
        }
        Self(a,b)
    }
}

struct Tasks{
    tasks: Vec<Task>
}

struct Task{
    name: String,
    priority: i32,
    done: bool,
}

impl Iterator for Tasks{
    type Item = Task;

    fn next(&mut self) -> Option<Self::Item> {
        self.tasks
            .iter()
            .position(|t| !t.done)
            .map(|i| self.tasks.remove(i))
    }
}

fn restricted<T, U>(t1: T, t2: T, u: U) -> T
        where T:  Ord + Debug + PartialOrd, U: Display{
    if(t1 < t2){
        println!("minor: <{:?}>", t1);
        println!("u: <{u}>");

        t1
    }else{
        println!("minor: <{:?}>", t2);
        println!("u: <{u}>");

        t2
    }


}

#[derive(Debug, Clone)]
struct Book{
    title: String,
    cat: Category,
}

#[derive(Debug, Default, Clone)]
enum Category {
    Fantasy,
    Horror,
    SciFi,
    Romance,
    Thriller,
    Historical,
    Comedy,
    Drama,
    Poetry,
    #[default]
    Other,
}

#[derive(Debug, Default)]
struct Library {
    bookcases: [Vec<Book>; 10]
}

impl Default for Book{
    fn default() -> Self {
        let max = thread_rng().gen_range(1..21) + 1;
        let mut title = "".to_owned();
        for i in 0..max{
            title.push(('a' as u8 + thread_rng().gen_range(0..25) as u8) as char);
        }

        let cat = Category::default();
        Self{ title, cat }
    }
}

impl Book{
    fn default_with_cat(c: Category) -> Self{
        Self{ cat: c, ..Book::default() }
    }
}

trait  Populatable{
    fn populate(&mut self);
}

impl Populatable for Library{
    fn populate(&mut self) {
        for i in 0..10{
            for j in 0..3 {
                self.bookcases[i].push(Book::default_with_cat(Category::default()));
            }
        }

    }
}

trait Printable{
    fn print(&self);
}

impl Printable for i32{
    fn print(&self) {
        println!("{}", self);
    }
}

impl Printable for String{
    fn print(&self) {
        println!("{}", self);
    }
}

impl<T: Printable> Printable for Vec<T>{
    fn print(&self) {
        self.iter().for_each(|x| { x.print() });
    }
}

fn print<T>(printable: &T) where T: Printable{
    printable.print();
}

fn print2(printable: &dyn Printable){
    printable.print();
}