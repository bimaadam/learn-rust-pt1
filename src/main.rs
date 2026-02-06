mod variables;
mod conditions;
mod borrowchecker;
mod ownership;
mod function;

fn main() {
    variables::run();
    conditions::run();
    borrowchecker::run();
    ownership::run();
    function::say_hello();
}