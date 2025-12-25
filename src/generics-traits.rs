use std::{
    iter::{Product, Sum},
    ops::{AddAssign, Mul, MulAssign},
};

// fn main() {
//     println!("Let's get Rusty!");

//     let my_num = [1, 2, 3, 4];

//     let your_num = [4, 5, 6, 7];
//     // let mut my_sum = 0;
//     // let mut your_sum = 0;

//     // for i in 1..my_num.len() {
//     //     my_sum += my_num[i]
//     // }

//     // for i in 1..your_num.len() {
//     //     your_sum += your_num[i]
//     // }

//     // println!("My sum is {}", my_sum);
//     // println!("My sum is {}", your_sum);

//     let my_sum = reusable(&my_num);
//     let your_sum = reusable(&your_num);

//     let loop_my_sum = using_loops(&my_num);
//     let loop_your_sum = using_loops(&your_num);

//     println!("My sum is {}", my_sum);
//     println!("My sum is {}", your_sum);
//     println!("My sum is {}", loop_my_sum);
//     println!("My sum is {}", loop_your_sum);

//     println!("My nums product is {}", mul_generic(&my_num));
//     println!("My nums product is {}", without_methods(&my_num));
// }

// function which use generics
pub fn reusable<T>(nums: &[T]) -> T
where
    T: Copy + Sum<T>,
{
    nums.iter().copied().sum()
}

/// sum generic
pub fn sum_generic<T: AddAssign + Copy>(nums: &[T]) -> T {
    let mut sum: T = nums[0];

    for i in 1..nums.len() {
        sum += nums[i]
    }
    sum
}

pub fn mul_generic<T: MulAssign + Copy>(nums: &[T]) -> T {
    let mut mul = nums[0];

    for i in nums {
        mul *= *i
    }
    mul
}

// function using general loops
pub fn using_loops(nums: &[i32]) -> i32 {
    let mut sum = 0;

    for i in 0..nums.len() {
        sum += nums[i]
    }
    sum
}

// product function with generics
pub fn product_generic<T>(nums: &[T]) -> T
where
    T: Copy + Product<T>,
{
    nums.iter().copied().product()
}

pub fn without_methods<T>(nums: &[T]) -> T
where
    T: Copy + Mul<Output = T> + From<u8>,
{
    let mut mul = T::from(1);

    for &n in nums {
        mul = mul * n;
    }
    mul
}

pub fn sum_generic1<T: Add<Output = T> + Copy>(nums: &[T]) -> T {
    let mut sum = nums[0];

    for i in 1..nums.len() {
        sum = sum + nums[i]
    }
    sum
}

trait Sellable {
    fn price(&self) -> u16;
    fn description(&self) -> String;
}

pub struct Sword {
    pub name: String,
    pub damage: u16,
    pub swing_time_ms: u16,
}

impl Sellable for Sword {
    fn price(&self) -> u16 {
        (self.damage * 100_u16) / self.swing_time_ms * 10u16
    }
    fn description(&self) -> String {
        format!(
            "{} damage: {} swings time: {} ms",
            self.name, self.damage, self.swing_time_ms
        )
    }
}

pub struct Sheild {
    pub name: String,
    pub armor: u16,
    pub sheild: u16,
}

impl Sellable for Sheild {
    fn price(&self) -> u16 {
        self.armor + self.sheild
    }

    fn description(&self) -> String {
        format!(
            "{} armor: {} sheild: {}",
            self.name, self.armor, self.sheild
        )
    }
}

fn vendor_static<T: Sellable>(items: &T) -> String {
    format!("I offer {} for [{}g]", items.description(), items.price())
}

fn vendor_dynamic(items: &dyn Sellable) -> String {
    format!("I offer {} for [{}g]", items.description(), items.price())
}

// traits are not object safe is
// - it they the methods which return self
// - if they return generic types
// - if they require self: Sized

fn main() {
    let sword = Sword {
        name: "Sword of apollo".into(),
        damage: 30,
        swing_time_ms: 500,
    };

    let sheild = Sheild {
        name: "Sheild of tetris".into(),
        armor: 50,
        sheild: 75,
    };

    println!("Static implementation");
    println!("{}", vendor_static(&sword));
    println!("{}", vendor_static(&sheild));

    // dynamic traits use with reference
    let sellable: Vec<&dyn Sellable> = vec![&sheild, &sword];

    // dynamic traits use with ownership
    let own_sellables: Vec<Box<dyn Sellable>> = vec![
        Box::new(Sword {
            name: "Jaya the emperor".into(),
            damage: 90,
            swing_time_ms: 200,
        }),
        Box::new(Sheild {
            name: "Ashwi the Lord".into(),
            armor: 75,
            sheild: 60,
        }),
    ];
    println!("Dynamic implementation");

    // as here sellable is reference traits in passing we can pass them directly
    for s in sellable {
        println!("{}", vendor_dynamic(s))
    }
    // here the own sellables are the ownership traits we need to pass their reference
    for s in own_sellables {
        println!("{}", vendor_dynamic(s.as_ref()))
    }

    let nums = vec![1, 2, 3, 4];
    println!("Max number is {:?}", max_value(&nums).unwrap())
}

pub fn max_value1<T: PartialOrd + Copy>(nums: &[T]) -> Option<T> {
    let mut iter = nums.iter().copied();
    let mut max_num = iter.next()?;

    for n in iter {
        if n > max_num {
            max_num = n
        }
    }
    Some(max_num)
}

fn max_value<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    let mut iter = items.iter().copied();
    let mut max_num = iter.next()?;

    for n in iter {
        if n > max_num {
            max_num = n
        }
    }

    Some(max_num)
}

fn max_ref<T: PartialOrd>(items: &[T]) -> Option<&T> {
    let mut iter = items.iter();
    let mut max_num = iter.next().unwrap();

    for n in iter {
        if n > &max_num {
            max_num = n
        }
    }

    Some(max_num)
}

// there is no issue in here
fn product<T: Mul<Output = T> + Copy>(items: &[T]) -> T {
    items[0]
}

fn vendor_multipe_items<T: Sellable>(items: &[T]) -> () {
    for item in items {
        println!("I offer {} for [{}g]", item.description(), item.price());
    }
}

trait Add {
    type Output;
}

trait Zero {
    fn zero() -> Self;
}

impl Zero for i32 {
    fn zero() -> Self {
        0
    }
}

impl Zero for i64 {
    fn zero() -> Self {
        0
    }
}

fn sum_zero_generic<T: Zero + AddAssign + Copy>(items: &[T]) -> T {
    let mut sum = T::zero();

    for &n in items {
        sum += n
    }

    sum
}
