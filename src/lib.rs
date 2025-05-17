use minijinja::{context, Environment};

fn main() {
    let mut env = Environment::new();
    
    env.add_template("layout.html", include_str!("../templates/layout.html")).unwrap();
    env.add_template("index.html", include_str!("../templates/index.html")).unwrap();

    let template = env.get_template("index.html").unwrap();
    println!("{}", template.render(context! {title => "matx.dev"}).unwrap());
}
