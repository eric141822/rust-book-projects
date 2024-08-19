pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // components is a vec of Box<dyn Draw> trait objects, i.e. a vec of references to objects 
    //  that implement the Draw trait
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: i32,
    pub height: i32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("Drawing a button of width {} and height {} and label {}", self.width, self.height, self.label);
    }
}