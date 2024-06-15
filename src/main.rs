use renderer::{Renderer};
mod renderer;
mod matrix;
use iced::{
   Application, Settings,
};




// testing area
// (translation, dilation, shear, rotation, reflection)
fn main() -> iced::Result {
   Renderer::run(Settings {
       //antialiasing: true,
       ..Settings::default()
   })
} 


