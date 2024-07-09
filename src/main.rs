use in_put::input;


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
    }

fn main() {

    let  user_input = input!("talk 2 me ");
    println!("You entered: {:?}", &user_input);
    print_type_of(&user_input);

}
