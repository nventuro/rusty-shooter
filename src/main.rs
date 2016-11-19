extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

mod phi;
mod views;

fn main() {
    ::phi::spawn("rusty-shooter", |phi| {
        Box::new(::views::main_menu::MainMenuView::new(phi))
    });
}
