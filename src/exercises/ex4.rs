use rand::Rng;
use std::collections::{HashMap, LinkedList};
use std::f32::consts::PI;
use std::fmt::{Debug, Display, Formatter};
use std::mem;
use std::ops::{Add, Sub};

pub(crate) fn test_all() {
    println!("{:?}", find_equal("Ciao", "Caiooao"));
    println!("{:?}", find_equal("Ciao", "Caioo"));

    println!(
        "{:?}",
        lucky_slice("Caioolashklhguisdygiudlfkdhgihukshidvhjidhkjfhkjdshfajskfhdjfgdsjkfgjksdghg")
    );

    let marco3m = Person::new("Marco3", None, None);
    let marco2af = Person::new("Marco2", Some(&marco3m), None);
    let marco2am = Person::new("Marco2", None, None);
    let marco2bf = Person::new("Marco2", None, None);
    let marco2bm = Person::new("Marco2", None, None);
    let marco1f = Person::new("Marco1a", Some(&marco2am), Some(&marco2af));
    let marco1m = Person::new("Marco1b", Some(&marco2bm), Some(&marco2bf));
    let marco = Person::new("Marco", Some(&marco1m), Some(&marco1f));
    println!("{:?}", marco.find_relatives(0));
    println!("{:?}", marco.find_relatives(1));
    println!("{:?}", marco.find_relatives(2));
    println!("{:?}", marco.find_relatives(3));
    println!("{:?}", marco.find_relatives(4));

    println!("Roots: {:?}", marco.find_roots());

    let mut l = LinkedList::new();
    l.push_back(10.5f64);
    l.push_back(11.5f64);
    l.push_back(12.5f64);
    println!("{:?}", l.split());

    println!("Skip: {}", skip_prefix("3315683243", "39"));
    println!("Skip: {}", skip_prefix("3", "39"));
    println!("Skip: {}", skip_prefix("32", "39"));
    println!("Skip: {}", skip_prefix("39", "39"));
    println!("Skip: {}", skip_prefix("3315683243", "+39"));
    println!("Skip: {}", skip_prefix("+393315683243", "+39"));
}

#[derive(Eq, PartialEq)]
enum Role{GUEST, USER, ADMIN}
#[derive(Eq, PartialEq, Hash)]
enum Permission{READ, WRITE, EXECUTE}

struct Action{
    action: String,
    permissions: HashMap<Permission, bool>
}

struct User{
    name: String,
    role: Role,
    actions: Vec<Action>
}

trait Auth{
    fn check_permission(&self , action: &str , permission_type: &Permission) -> bool;
    fn can_write(&self, string: &str) -> bool;
    fn can_read(&self, string: &str) -> bool;
    fn can_execute(&self, string: &str) -> bool;
}

impl Auth for User{
    fn check_permission(&self, action: &str, permission_type: &Permission) -> bool {
        let a = self.actions.iter().find(|&x| {x.action == action});

        match a{
            None => { false }
            Some(act) => {
                act.permissions.get(permission_type).is_some_and(|&x| {x})
            }
        }
    }

    fn can_write(&self, string: &str) -> bool {
        self.check_permission(string, &Permission::WRITE)
    }

    fn can_read(&self, string: &str) -> bool {
        self.check_permission(string, &Permission::READ)
    }

    fn can_execute(&self, string: &str) -> bool {
        self.check_permission(string, &Permission::EXECUTE)
    }
}

impl Default for Action{
    fn default() -> Self {
        let mut h = HashMap::new();
        h.insert(Permission::READ, false);
        h.insert(Permission::WRITE, false);
        h.insert(Permission::EXECUTE, false);
        Self{action: "".to_string(), permissions: h}
    }
}

impl Action{
    pub fn new(action: String , read: bool , write: bool, execute: bool) -> Self {
        let mut h = HashMap::new();
        h.insert(Permission::READ, read);
        h.insert(Permission::WRITE, write);
        h.insert(Permission::EXECUTE, execute);
        Self { action, permissions: h}
    }
    

}

impl Default for User{
    fn default() -> Self {
        Self{name: "Guest".to_owned(), role: Role::GUEST, actions: vec![]}
    }
}

impl User{
    fn change_role(&mut self, role: Role) -> Result<(), String>{
        match (role, &self.role) {
            (r, &Role::ADMIN) => {
                self.role = r;
                Ok(())
            }
            (Role::ADMIN, _) => {
                Err("Cannot set to admin as not an admin".to_owned())
            }
            (r, &Role::USER) => {
                self.role = r;
                Ok(())
            }
            (Role::GUEST, &Role::GUEST) => {Ok(())}
            _ => { Err("Cannot set to user as guest".to_owned())}
        }
    }
}

fn sudo_change_permission(user: &mut User, act_str: String, permission: Permission, value: bool){
    let act = user.actions.iter_mut().find(|x| {x.action == act_str}).unwrap();
    act.permissions.insert(permission, value);
}

struct Chair<'a> {
    color: &'a str,
    quantity: &'a usize,
}

struct Wardrobe<'a> {
    color: &'a str,
    quantity: &'a usize,
}

trait Object<'a> {
    fn build(&'a self) -> &'a str;
    fn get_quanity(&'a self) -> String;
}

impl<'a> Object<'a> for Chair<'a> {
    fn build(&'a self) -> &'a str {
        "Chair has been build"
    }

    fn get_quanity(&'a self) -> String {
        format!("We have {} chairs", self.quantity)
    }
}

impl<'a> Display for Chair<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.quantity <= &0 {
            write!(f, "You have no chair")
        } else {
            if self.quantity == &1 {
                write!(f, "You have 1 {} chair", self.color)
            } else {
                write!(f, "You have {} {} chairs", self.quantity, self.color)
            }
        }
    }
}
impl<'a> Display for Wardrobe<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.quantity <= &0 {
            write!(f, "You have no wardrobe")
        } else {
            if self.quantity == &1 {
                write!(f, "You have 1 {} wardrobe", self.color)
            } else {
                write!(f, "You have {} {} wardrobes", self.quantity, self.color)
            }
        }
    }
}

impl<'a> Object<'a> for Wardrobe<'a> {
    fn build(&'a self) -> &'a str {
        "Wardrobe has been build"
    }

    fn get_quanity(&'a self) -> String {
        format!("We have {} wardrobe", self.quantity)
    }
}

fn skip_prefix<'a>(telephone_number: &'a str, prefix: &'a str) -> &'a str {
    if prefix.len() >= telephone_number.len() {
        return telephone_number;
    }

    let split = telephone_number.split_at(prefix.len());

    if (split.0 == prefix) {
        split.1
    } else {
        telephone_number
    }
}

struct Point {
    x: i32,
    y: i32,
}

struct Circle {
    center: Point,
    radius: i32,
}

struct Rect {
    top_left: Point,
    bottom_right: Point,
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            center: Point { x: 0, y: 0 },
            radius: 1,
        }
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            top_left: Point { x: -1, y: 1 },
            bottom_right: Point { x: 1, y: -1 },
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

struct Area(f32);

impl Default for Area {
    fn default() -> Self {
        Self(0.0)
    }
}

impl Display for Area {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Area is {} cm²", self.0)
    }
}

trait GetArea {
    fn get_area(&self) -> Area;
}

impl GetArea for Point {
    fn get_area(&self) -> Area {
        Area::default()
    }
}

impl GetArea for Circle {
    fn get_area(&self) -> Area {
        Area((self.radius * self.radius) as f32 * PI)
    }
}

impl GetArea for Rect {
    fn get_area(&self) -> Area {
        Area(
            ((self.bottom_right.x * self.top_left.x) * (self.top_left.y * self.bottom_right.y))
                as f32,
        )
    }
}

impl Add for Area {
    type Output = Area;

    fn add(self, rhs: Self) -> Self::Output {
        Area(self.0 + rhs.0)
    }
}

impl Add for &dyn GetArea {
    type Output = Area;

    fn add(self, rhs: Self) -> Self::Output {
        Area(self.get_area().0 + rhs.get_area().0)
    }
}

fn sum_area(slice: &[&dyn GetArea]) -> Area {
    slice
        .iter()
        .map(|&x| x.get_area())
        .fold(Area::default(), |acc, item| acc + item)
}

trait Split<'a> {
    type ReturnType;

    fn split(&'a self) -> (Self::ReturnType, Self::ReturnType);
}

impl<'a> Split<'a> for String {
    type ReturnType = &'a str;

    fn split(&'a self) -> (Self::ReturnType, Self::ReturnType) {
        self.split_at(self.len() / 2)
    }
}

impl<'a> Split<'a> for &'a [i32] {
    type ReturnType = &'a [i32];

    fn split(&'a self) -> (Self::ReturnType, Self::ReturnType) {
        self.split_at(self.len() / 2)
    }
}

impl<'a> Split<'a> for LinkedList<f64> {
    type ReturnType = LinkedList<f64>;

    fn split(&'a self) -> (Self::ReturnType, Self::ReturnType) {
        let mut left = self.clone();
        let right = left.split_off(left.len() / 2);
        (left, right)
    }
}

pub struct Person<'a> {
    name: String,
    father: Option<&'a Person<'a>>,
    mother: Option<&'a Person<'a>>,
}

impl Debug for Person<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<'a> Person<'a> {
    pub fn new(name: &str, father: Option<&'a Person<'a>>, mother: Option<&'a Person<'a>>) -> Self {
        Self {
            name: name.to_string(),
            father,
            mother,
        }
    }

    pub fn find_relatives(&self, generations: u32) -> Vec<&Person> {
        let mut res: Vec<&Person> = vec![self];

        for i in 0..generations {
            let mut tmp: Vec<&Person> = vec![];

            for k in 0..res.len() {
                if let Some(f) = res[k].father {
                    tmp.push(f);
                }
                if let Some(m) = res[k].mother {
                    tmp.push(m);
                }
            }

            mem::swap(&mut tmp, &mut res);
        }

        res
    }

    fn find_roots_int(&'a self, relatives: &mut Vec<&'a Person<'a>>) {
        if self.mother.is_none() || self.father.is_none() {
            relatives.push(self);
            return;
        }

        self.mother.unwrap().find_roots_int(relatives);
        self.father.unwrap().find_roots_int(relatives);
    }

    pub fn find_roots(&self) -> Vec<&Person> {
        let mut res = vec![];
        self.find_roots_int(&mut res);
        res
    }
}

fn find_equal<'a, 'b>(s1: &'a str, s2: &'b str) -> Option<(&'a str, &'b str)> {
    for i in 0..(s1.len() - 1) {
        for j in 0..(s2.len() - 1) {
            if s1.chars().nth(i) == s2.chars().nth(j)
                && s1.chars().nth(i + 1) == s2.chars().nth(j + 1)
            {
                return Some((&s1[i..i + 2], &s2[j..j + 2]));
            }
        }
    }
    None
}

fn lucky_slice(input_str: &str) -> Option<String> {
    let mut rand = String::new();
    for i in 0..input_str.len() {
        rand.push(('a' as u8 + (rand::thread_rng().gen_range(0..25))) as char);
    }
    //println!("{rand}");
    Some(find_equal(&rand, input_str)?.1.to_string())
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    //THIS LINE
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

struct DoubleRef<'a, 'b: 'a, T> {
    r: &'a T,
    s: &'b T,
}
