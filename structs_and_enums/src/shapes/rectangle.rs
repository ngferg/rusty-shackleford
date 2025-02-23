pub struct Rectangle {  // `pub` makes it publicly accessible
    pub width: u32,     // `pub` on fields if you want them accessible too
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}