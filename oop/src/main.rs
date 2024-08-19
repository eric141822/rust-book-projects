use gui::Draw;

/* 
Using dyn Trait to Accept Values of Different Types requires the use of dynamic dispatch.
Dynamic dispatch means that the decision about which method implementation to call gets made at runtime.
In contrast to a static dispatch, this will incur a runtime performance penalty.
*/
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a SelectBox of width {} and height {} and options {:?}", self.width, self.height, self.options);
    }
}

fn main() {
    let screen = gui::Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(gui::Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            })
        ],
    };

    screen.run();
}