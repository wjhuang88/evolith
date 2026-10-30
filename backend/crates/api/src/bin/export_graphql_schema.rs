fn main() {
    let sdl = api::graphql::build_schema().sdl();
    println!("{}", sdl.trim_end());
}
