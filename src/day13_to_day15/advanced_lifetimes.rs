pub fn examples() {
    println!("Advanced Lifetimes Examples:");
    
    let x = 5;
    let r = &x;
    print_ref(r);
    
    let context = Context("context");
    let parser = Parser { context: &context };
    parser.parse("input");
}

fn print_ref<'a, T>(t: &'a T) where T: std::fmt::Debug + 'a {
    println!("t: {:?}", t);
}

struct Context<'a>(&'a str);

struct Parser<'a> {
    context: &'a Context<'a>,
}

impl<'a> Parser<'a> {
    fn parse(&self, input: &str) {
        println!("Parsing {} in context {}", input, self.context.0);
    }
}