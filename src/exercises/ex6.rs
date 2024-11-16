use std::cell::RefCell;
use std::cmp::PartialEq;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::Rc;

pub(crate) fn test_all() {
    println!("{:?}", Light::default());
    println!("{:?}", EntangledBit::default());

    let mut b1 = EntangledBit::default();
    let mut b2 = EntangledBit::default();
    println!("{} should be {}", b2.get(),false);
    b1.entangle_with(&mut b2);
    b1.set();
    println!("{} should be {}",b2.get(),true);
}

//TODO review 5 and 6

#[derive(Default, Eq, PartialEq, Debug)]
struct EntangledBit{
    bit: Rc<RefCell<bool>>,
}

impl EntangledBit{
    fn entangle_with(&mut self, other: &mut Self){
        other.bit = self.bit.clone()
    }

    fn set(&mut self){
        *self.bit.borrow_mut() = true;
    }

    fn reset(&mut self){
        *self.bit.borrow_mut() = true;
    }


    fn get(&self) -> bool{
        *self.bit.borrow()
    }
}


trait CompileTimeNode{
    type LeftType: CompileTimeNode;
    type RightType: CompileTimeNode;
    fn is_none() -> bool;


}
struct NullNode{}
struct Node<L: CompileTimeNode, R:CompileTimeNode>{
    left: PhantomData<L>,
    right: PhantomData<R>
}

fn count_nodes<T: CompileTimeNode>() -> i32{
    let mut count = 0;
    if !T::is_none(){
        count = 1;
        count += count_nodes::<T::LeftType>();
        count += count_nodes::<T::RightType>();
    }
    count
}

impl CompileTimeNode for NullNode{
    type LeftType = NullNode;
    type RightType = NullNode;

    fn is_none() -> bool {
        true
    }
}

impl<L: CompileTimeNode,R: CompileTimeNode> CompileTimeNode for Node<L,R>{
    type LeftType = L;
    type RightType = R;

    fn is_none() -> bool {
        false
    }
}


#[derive(Default, Debug)]
struct Light{
    id: u32,
    on: bool,
    burn_out: bool
}

impl Light {
    pub fn new(id: u32, on: bool, burn_out: bool) -> Self {
        Self { id, on, burn_out }
    }
}

#[derive(Default, Debug)]
struct Illumination{
    lights: Vec<Light>,
}

impl Illumination {
    pub fn new(lights: Vec<Light>) -> Self {
        Self { lights }
    }
}

impl Iterator for Illumination{
    type Item = Light;

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.lights.iter().position(|x| {x.burn_out == true})?;
        Some(self.lights.remove(p))
    }
}

trait Sound{
    fn make_sound(&self) -> String;
}

struct Animal{}
struct Person{}
struct Construction{}

impl Sound for Animal{
    fn make_sound(&self) -> String {
        "behhhh".to_owned()
    }
}

impl Sound for Person{
    fn make_sound(&self) -> String {
        "ahhhh".to_owned()
    }
}

impl Sound for Construction{
    fn make_sound(&self) -> String {
        "trrrrr".to_owned()
    }
}

struct FarmCell{
    element: Box<dyn Sound>,
    next: Option<Box<FarmCell>>
}

impl FarmCell{
    pub fn new(element: Box<dyn Sound>) -> Self {
        Self { element, next: None }
    }

    pub fn insert(&mut self, value: Box<dyn Sound>) {
        match self.next {
            None => { self.next = Some(Box::new(FarmCell::new(value))); }
            Some(ref mut n) => { n.insert(value); }
        }
    }
}

impl Sound for FarmCell{
    fn make_sound(&self) -> String {
        self.element.make_sound() +
            &self.next.as_deref().map_or("".to_owned(),
                                         |x| {x.make_sound()})
    }
}



type CarRef = Rc<RefCell<Car>>;

#[derive(Debug)]
struct Car{
    model: String ,
    year: u32 ,
    price: u32 ,
    rent: bool
}

impl Car {
    pub fn new(model: String, year: u32, price: u32, rent: bool) -> Self {
        Self { model, year, price, rent }
    }
}

impl Default for Car{
    fn default() -> Self {
        Self{model: "".to_owned(), year: 2024, price: 0, rent: false}
    }
}

struct CarDealer{
    cars: Vec<CarRef>
}

struct User{
    car: Option<CarRef>
}

impl User{
    fn print_car(&self){
        match self.car {
            None => { println!("User has no car");}
            Some(ref car) => { println!("User has car: {:?}", car); }
        }
    }
}

impl CarDealer{
    pub fn new(cars: Vec<Car>) -> Self {
        Self { cars: cars.into_iter()
            .map(|x| {Rc::new(RefCell::new(x))})
            .collect()
        }
    }

    fn add_car(&mut self, car: Car){
        self.cars.push(Rc::new(RefCell::new(car)));
    }

    fn print_cars(&self){
        self.cars.iter().for_each(|x| {print!("{:?}", x)});
    }

    fn rent_user(&self, user: &mut User, model: String){
        let car_opt = self.cars.iter()
            .find(|&x| {x.borrow().model == model});

        match car_opt{
            None => { println!("Car not found"); }
            Some(car) => {
                user.car = Some(car.clone());
                car.borrow_mut().rent = true;
            }
        }
    }

    fn end_rental(&self, user: &mut User){
        if let Some(ref car) = user.car {
            car.borrow_mut().rent = false;
        } else {
            println!("User has no car");
        }
    }
}



struct TreeNode<T: Clone + PartialOrd + Debug>{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>
}



impl<T: Clone + PartialOrd + Debug> TreeNode<T>{
    pub fn new(value: T) -> Self {
        Self { value, left: None, right: None }
    }

    pub fn from_vector(vec: &Vec<T>) -> Self{
        let mut tree = TreeNode::new(vec[0].clone());

        vec.iter().for_each(|x| {tree.insert(x.clone())});
        tree
    }

    pub fn insert(&mut self, value: T){
        if self.value < value {

            if let Some(r) = self.left.as_deref_mut(){
                r.insert(value);
            } else{
                self.left = Some(Box::new(TreeNode::new(value)));
            }
        }else if self.value > value {
            if let Some(r) = self.left.as_deref_mut(){
                r.insert(value);
            } else{
                self.right = Some(Box::new(TreeNode::new(value)));
            }
        }
    }
}

