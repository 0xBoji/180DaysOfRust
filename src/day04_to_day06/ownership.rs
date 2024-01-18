//ownership works

    pub fn run() {
        let my_banhmi = String::from("bánh mì");
        eat_pizza(my_banhmi); 
        }

    fn eat_pizza(banhmi: String) {
        println!("I am eating {}!", banhmi);
    }