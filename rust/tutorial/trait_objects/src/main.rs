extern crate trait_objects;

fn main() {
    let screen = trait_objects::Screen {
        components: vec![
            Box::new(trait_objects::Button {
                width: 50,
                height: 10,
                label: String::from("Press here"),
            }),
            Box::new(trait_objects::SelectBox {
                width: 100,
                height: 20,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Go to hell"),
                ],
            }),
        ],
    };

    screen.run();
}
