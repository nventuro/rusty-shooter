extern crate sdl2;

mod phi;
mod views;

fn main() {
    ::phi::spawn("rusty-shooter", |_| {
        Box::new(::views::RedView)
    });
}
