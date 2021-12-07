use std::fmt::{Debug, Display};

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
    let _integer = Point { _x: 5, _y: 10 };
    let _float = Point { _x: 1.0, _y: 4.0 };
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    //{
    //    let r;
    //    {
    //        let x = 5;
    //        r = &x;
    //    }
    //    println!("r: {}", r);
    //}
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    //&i32        // a reference
    //&'a i32     // a reference with an explicit lifetime
    //&'a mut i32 // a mutable reference with an explicit lifetime
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // won't compile
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);
    let _s: &'static str = "I have a static lifetime.";
}

fn _longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct _ImportantExcerpt<'a> {
    part: &'a str,
}

fn _test() {
    // does not compile, i can't outlive what it refers because of the annotation we used
    // let i;
    //{
    //    let novel = String::from("Call me Ishmael. Some years ago...");
    //    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    //    i = ImportantExcerpt {
    //        part: first_sentence,
    //    };
    //}
    //print!("{}", i.part);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn notify(_item: &impl Summary) {}
pub fn notify_with_traits_bound<T: Summary>(_item: &T) {}
pub fn notify_with_multiple_traits(_item1: &impl Summary, _item2: &impl Summary) {}
pub fn notify_with_factorisation<T: Summary>(_item1: &T, _item2: &T) {}
pub fn notify_combined(_item: &(impl Summary + Display)) {}
pub fn notify_combined_traits_bound<T: Summary + Display>(_item: &T) {}
fn _some_function_with_traits_bounds<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    todo!()
}
fn _returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
    // illegal, we return two different types
    // if switch {
    //     NewsArticle {
    //         headline: String::from(
    //             "Penguins win the Stanley Cup Championship!",
    //         ),
    //         location: String::from("Pittsburgh, PA, USA"),
    //         author: String::from("Iceburgh"),
    //         content: String::from(
    //             "The Pittsburgh Penguins once again are the best \
    //              hockey team in the NHL.",
    //         ),
    //     }
    // } else {
    //     Tweet {
    //         username: String::from("horse_ebooks"),
    //         content: String::from(
    //             "of course, as you probably already know, people",
    //         ),
    //         reply: false,
    //         retweet: false,
    //     }
    // }
}

struct _Pair<T> {
    x: T,
    y: T,
}

impl<T> _Pair<T> {
    fn _new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> _Pair<T> {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// blanket implementations
// impl<T: Display> ToString for T {
//     fn to_string(&self) -> String {
//         todo!()
//     }
// }

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl SummaryWithDefault for NewsArticle {
    fn summarize_author(&self) -> String {
        todo!()
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait SummaryWithDefault {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

enum _Result<T, E> {
    Ok(T),
    Err(E),
}

struct Point<T> {
    _x: T,
    _y: T,
}

impl<T> Point<T> {
    fn _x(&self) -> &T {
        &self._x
    }
}
impl Point<f32> {
    fn _distance_from_origin(&self) -> f32 {
        (self._x.powi(2) + self._y.powi(2)).sqrt()
    }
}

struct _MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> _MixedPoint<T, U> {
    fn _mixup<V, W>(self, other: _MixedPoint<V, W>) -> _MixedPoint<T, W> {
        _MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn _largest_with_fix<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn _largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn _largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
