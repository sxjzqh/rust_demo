fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// build_user 参数名与字段名都完全相同，我们可以使用字段初始化简写
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// 使用没有命名字段的元组结构体来创建不同的类型
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


// 没有任何字段的类单元结构体
struct AlwaysEqual;

// 面积计算的几种方式
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// 方法
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 带有更多参数的方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // 关联函数，不是方法了，一般做结构体构造
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}