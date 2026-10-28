fn main() {
    println!("{}", api::graphql::build_schema().sdl());
}
