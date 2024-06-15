use rand::prelude::*;
use rand::distributions::WeightedIndex;
use crate::matrix::Matrix;



use iced::widget::canvas::{
   Cache, Cursor, Geometry, LineCap, Path, Stroke, LineDash
};
use iced::widget::{canvas, container, text, row, column, button, text_input, slider, scrollable, vertical_space};
use iced::{
   Application, Color, Command, Element, Length, Point, Rectangle,
   Theme, Size, Alignment
};
use iced::theme;
use iced::executor;


// 15/06/24
// I've decided to leave this project unfinished (for now). All


// RENDERER
// stores each component, and draws them to the screen.


// plan so we dont loe our minds
// - create basic boilerplate 4 renderer app (done)
// - create a function that does one iteration of cg (done)
// - create a function that then does it n times, sends back n points to draw (done)
// - Update state types for color, size, etc
// - create a subscription that runs this func every t ms, updates canvas






#[derive(Debug, Clone)]
pub struct RendererState {
   n_points: usize,
   n_points_final: usize,
   background_color: [u8; 3],
   fractal_color: [u8; 3],
   dot_size: f32,
   screen_size: [f32; 2], // 800 is good size for final render
   final_screen_size: [f32; 2],
   _update_time: u64,
   number_of_updates: u64,
   // ui state
   action_add_step_opened: Option<usize>,
   _add_action_opened: bool,
   _ifs_template_opened: bool,
   do_final_render: bool,
}


#[derive(Debug, Clone)]
pub struct _UIState {


}


#[derive(Debug, Clone)]
pub struct Transformation {
   actions: Vec<TransformationType>,
   translation: [f64; 2],
   using_rad: bool,
}


#[derive(Debug, Clone)]
pub enum TransformationType {
   Dilation([f64; 2]),
   Shear([f64; 2]),
   Rotation(f64),
   Reflection(f64)
}


impl Transformation {
   pub fn new(
       using_rad: bool,
   ) -> Self {
       Transformation {actions: vec![], translation: [0.0,0.0], using_rad}
   }


   pub fn translate(&mut self, translation: [f64; 2]) {
       self.translation = translation;
   }


   pub fn dilate(&mut self, dilation: [f64; 2]) {
       self.actions.push(TransformationType::Dilation(dilation))
   }


   pub fn shear(&mut self, shear: [f64; 2]) {
       self.actions.push(TransformationType::Shear(shear))
   }


   pub fn rotate(&mut self, rotation: f64) {
       self.actions.push(TransformationType::Rotation(rotation))
   }


   pub fn reflect(&mut self, reflection: f64) {
       self.actions.push(TransformationType::Reflection(reflection))
   }


   /// Moves action n to index m
   pub fn move_to(&mut self, _target_index: usize, _new_index: usize) -> Result<(), String> {
       todo!()
   }


   /// Deletes action at index n
   pub fn remove_action(&mut self, target_index: usize) -> Result<(), String> {
       if target_index >= self.actions.len() {
           Err(format!("remove_action: target_index is too large! {} >= {}", target_index, self.actions.len()))
       }
       else {
           self.actions.remove(target_index);
           Ok(())
       }
   }


   pub fn build(&self) -> (Matrix, Matrix) {
       let mut transf_matr = Matrix::identity(2);
       for action in self.actions.iter() {
           let new_matr = match action {
               TransformationType::Dilation([x,y]) => Matrix::dil_mat2d(*x, *y),
               TransformationType::Shear([x,y]) => Matrix::shear_mat2d(*x, *y),
               TransformationType::Reflection(angle) => Matrix::refl_mat2d(*angle, self.using_rad),
               TransformationType::Rotation(angle) => Matrix::rot_mat2d(*angle, self.using_rad),
           };
           transf_matr = new_matr * transf_matr;
       }
       let transl_matr = Matrix::new(vec![
           vec![self.translation[0]],
           vec![self.translation[1]],
       ]);
       (transf_matr, transl_matr)
   }
}


impl RendererState {
   // generates default state.
   pub fn default() -> Self {
       RendererState {
           n_points: 3_000,
           background_color: [0, 0, 0],
           fractal_color: [252, 186, 3],
           dot_size: 1.0,
           screen_size: [200.0,200.0],
           final_screen_size: [750.0, 750.0],
           _update_time: 100,
           number_of_updates: 0,
           n_points_final: 100_000,
           action_add_step_opened: None,
           _add_action_opened: false,
           _ifs_template_opened: false,
           do_final_render: false,
       }
   }
}


#[derive(Debug)]
pub struct Renderer {
   transformations: Vec<Transformation>, // (transformation, translation)
   state: RendererState,  // state
   renderer: Cache,
}


#[derive(Debug, Clone)]
pub enum Message {
   UpdateCanvas,
   AddTransformaton(Transformation),
   SettingChange(SettingChange),
   ActionChange(ActionChange),
}


#[derive(Debug, Clone)]
pub enum SettingChange {
   NPointsInRender(String),
   NPointsInFinal(String),
   DotSize(String),
   BackgroundColor(String, usize),
   FractalColor(String, usize),
   ResetToDefault,
   StartFinalRender
}


#[derive(Debug, Clone)]
pub enum ActionChange {
   AddNewAction,
   ToggleRadians(bool, usize),
   DeleteAction(usize), // index
   EditTranslation(String,usize,usize),
   AddNewStep(usize, TransformationType),
   DeleteStep(usize,usize), // action, step
   EditOneVarStep(String, usize, usize), // action, step
   EditTwoVarStep(String, usize, usize, usize), // val, index, action, step
   OpenAddStepMenu(usize),
}


impl Application for Renderer {
   type Message = Message;
   type Theme = Theme;
   type Executor = executor::Default;
   type Flags = ();


   fn new(_flags: ()) -> (Self, Command<Self::Message>) {
       let mut t1 = Transformation::new(false);
       let mut t2 = Transformation::new(false);
       let mut t3 = Transformation::new(false);
       t1.dilate([0.5,0.5]);
       t2.dilate([0.5,0.5]);
       t3.dilate([0.5,0.5]);
       t2.translate([0.5,0.0]);
       t3.translate([0.25,0.5]);


       (Renderer {
           transformations: vec![t1, t2, t3],
           state: RendererState::default(),
           renderer: Default::default(),
       }, Command::none())
   }


   fn title(&self) -> String {
       String::from("IFS Fractal Generator")
   }


   fn view(&self) -> Element<'_, Self::Message> {
       let mut actions: Vec<Element<'_, Self::Message>> = vec![
           text("Actions").size(70).into()
       ];
       for (a_index, actn) in self.transformations.iter().enumerate() {
           let actn_mat = actn.build();
           let determinant = actn_mat.0.det();
           let action_selected: bool = if let Some(ind) = self.state.action_add_step_opened {
               ind == a_index
           } else {false};
           let action_title: Element<'_, Self::Message> = row(vec![
               text(format!("Action {a_index}  ")).size(40).into(),
               text(format!("{determinant}")).size(20).into(),
               button("Delete").style(theme::Button::Destructive).on_press(Message::ActionChange(ActionChange::DeleteAction(a_index))).into()
           ]).into();
           let step_add: Element<'_, Self::Message> = if action_selected {
               row(vec![
                   button("Dilation").on_press(Message::ActionChange(ActionChange::AddNewStep(a_index, TransformationType::Dilation([0.0,0.0])))).into(),
                   button("Shear").on_press(Message::ActionChange(ActionChange::AddNewStep(a_index, TransformationType::Shear([0.0,0.0])))).into(),
                   button("Rotation").on_press(Message::ActionChange(ActionChange::AddNewStep(a_index, TransformationType::Rotation(0.0)))).into(),
                   button("Reflection").on_press(Message::ActionChange(ActionChange::AddNewStep(a_index, TransformationType::Reflection(0.0)))).into(),
               ]).into()
           } else {
               button("Add Step").on_press(Message::ActionChange(ActionChange::OpenAddStepMenu(a_index))).into()
           };
           let step_translation: Element<'_, Self::Message> = row(vec![
               text("Translation").size(30).into(),
               slider(
                   -100..=100, (actn.translation[0] * 100.0) as i32,
                   move |n| Message::ActionChange(ActionChange::EditTranslation(n.to_string(), 0, a_index))
                   ).into(),
                   text_input(
                       "0", &match actn.translation[0] {
                           0.0 => String::from(""),
                           _ => ((actn.translation[0] * 100.0) as isize).to_string()
                       },
                       move |n| Message::ActionChange(ActionChange::EditTranslation(n, 0, a_index))
                   ).into(),


                   slider(
                       -100..=100, (actn.translation[1] * 100.0) as i32,
                       move |n| Message::ActionChange(ActionChange::EditTranslation(n.to_string(), 1, a_index))
                       ).into(),
                       text_input(
                           "0", &match actn.translation[1] {
                               0.0 => String::from(""),
                               _ => ((actn.translation[1] * 100.0) as isize).to_string()
                           },
                           move |n| Message::ActionChange(ActionChange::EditTranslation(n, 1, a_index))
                       ).into()
           ]).into();
           let step_list: Vec<Element<'_, Self::Message>> = actn.actions.iter().enumerate().map(move |(s_index, step)| {
               let step_text = match step {
                   TransformationType::Dilation(_) => "Dilation",
                   TransformationType::Reflection(_) => "Reflection",
                   TransformationType::Rotation(_) => "Rotation",
                   TransformationType::Shear(_) => "Shear"
               };
               let step_input: Element<'_, Self::Message> = match step {
                   TransformationType::Dilation([x, y]) | TransformationType::Shear([x,y]) => {
                       row(vec![
                           slider(
                           -100..=100, (x * 100.0) as i32,
                           move |n| Message::ActionChange(ActionChange::EditTwoVarStep(n.to_string(), 0, a_index.clone(), s_index.clone()))
                           ).into(),
                           text_input(
                               "0", &match x {
                                   0.0 => String::from(""),
                                   _ => ((x * 100.0) as isize).to_string()
                               },
                               move |n| Message::ActionChange(ActionChange::EditTwoVarStep(n, 0, a_index.clone(), s_index.clone()))
                           ).into(),


                           slider(
                               -100..=100, (y * 100.0) as i32,
                               move |n| Message::ActionChange(ActionChange::EditTwoVarStep(n.to_string(), 1, a_index.clone(), s_index.clone()))
                               ).into(),
                               text_input(
                                   "0", &match y {
                                       0.0 => String::from(""),
                                       _ => ((y * 100.0) as isize).to_string()
                                   },
                                   move |n| Message::ActionChange(ActionChange::EditTwoVarStep(n, 1, a_index.clone(), s_index.clone()))
                               ).into()
                       ]).into()
                   },
                   TransformationType::Rotation(deg) | TransformationType::Reflection(deg) => {
                       row(vec![
                           slider(
                               -180..=180, *deg as i32,
                               move |n| Message::ActionChange(ActionChange::EditOneVarStep(n.to_string(), a_index.clone(), s_index.clone()))
                           ).into(),
                           text_input(
                               "0", &match *deg {
                                   0.0 => String::from(""),
                                   _ => (*deg  as isize).to_string()
                               },
                               move |n| Message::ActionChange(ActionChange::EditOneVarStep(n, a_index.clone(), s_index.clone()))
                           ).into()
                       ]).into()
                   }
               };
               row(vec![
                   button("del").style(theme::Button::Destructive).on_press(
                       Message::ActionChange(ActionChange::DeleteStep(a_index, s_index))
                   ).into(),
                   text(step_text).size(30).into(),
                   step_input,
               ]).into()
           }).collect();
           actions.push(action_title);
           actions.push(step_translation);
           actions.push(
               column(step_list).into()
           );
           actions.push(step_add);


       }
       actions.push(button("Add Action").style(theme::Button::Positive).on_press(
           Message::ActionChange(ActionChange::AddNewAction)
       ).into());
       let content: Element<_>;
       if self.state.do_final_render {
           content = row(vec![
               canvas(self as &Self)
                       .width(Length::from(self.state.final_screen_size[0] + 100.0))
                       .height(Length::from(self.state.final_screen_size[1] + 100.0)).into(),
               button("Edit").on_press(Message::SettingChange(SettingChange::StartFinalRender)).into()
           ]).into()
       } else {
           content = container(row(vec![   // Left: Actions, Right: Render/Settings
           scrollable(column(
               actions
           ).width(Length::Fill).padding([25,25,25,75]))
           .into(),


           column(vec![ // Render/Settings
               column(vec![ // Render
                   text("Render")
                   .size(70).into(),
                   container(column(vec![
                    canvas(self as &Self)
                    .width(Length::from(self.state.screen_size[0] * 1.5))
                    .height(Length::from(self.state.screen_size[1] * 1.5)).into(),
                       container(row(vec![
                           button("Render Fractal!")
                           .on_press(Message::SettingChange(SettingChange::StartFinalRender))
                           .into(),
                           button("Show IFS Template") // todo ifs template !!
                           .on_press(Message::UpdateCanvas)
                           .style(theme::Button::Positive)
                           .into(),
                       ]).spacing(10).align_items(Alignment::Center))
                       .center_x().width(Length::from(self.state.screen_size[0] * 1.5))
                       .into()
                   ]).spacing(12.5))
                   .center_x().center_y()
                   .width(Length::Fill).height(Length::Fill).into()
               ]).max_width(400).height(Length::FillPortion(2)).padding(25).spacing(25).align_items(Alignment::Center)
               .into(),
               container(scrollable(column(vec![ // Settings
                   text("Settings")
                   .size(70).into(),


                   container( // todo dry
                       row(vec![
                           container(text("BG Color (RGB)").size(20))
                           .width(Length::FillPortion(3)).center_y().height(30).into(),
                           row(vec![
                               text_input(
                                   "0",
                                   &match self.state.background_color[0] {
                                       0 => String::from(""),
                                       _ => self.state.background_color[0].to_string()
                                   },
                                   |txt| Message::SettingChange(SettingChange::BackgroundColor(txt,0))
                               )
                               .into(),
                               text_input(
                                   "0",
                                   &match self.state.background_color[1] {
                                       0 => String::from(""),
                                       _ => self.state.background_color[1].to_string()
                                   },
                                   |txt| Message::SettingChange(SettingChange::BackgroundColor(txt,1))
                               )
                               .into(),
                               text_input(
                                   "0",
                                   &match self.state.background_color[2] {
                                       0 => String::from(""),
                                       _ => self.state.background_color[2].to_string()
                                   },
                                   |txt| Message::SettingChange(SettingChange::BackgroundColor(txt,2))
                               )
                               .into()
                           ]).spacing(10).width(Length::FillPortion(2))
                           .into()
                       ])
                   )
                   .into(),


                   container(
                       row(vec![
                           container(text("Fractal Color (RGB)").size(20))
                           .width(Length::FillPortion(3)).center_y().height(30).into(),
                           row(vec![
                               text_input(
                                   "0",
                                   &match self.state.fractal_color[0] {
                                       0 => String::from(""),
                                       _ => self.state.fractal_color[0].to_string()
                                   },
                                   |txt| Message::SettingChange(SettingChange::FractalColor(txt,0))
                               )
                               .into(),
                               text_input(
                                   "0",
                                   &match self.state.fractal_color[1] {
                                       0 => String::from(""),
                                       _ => self.state.fractal_color[1].to_string()
                                   },
                                   |txt| Message::SettingChange(SettingChange::FractalColor(txt,1))
                               )
                               .into(),
                               text_input(
                                   "0",
                                   &match self.state.fractal_color[2] {
                                       0 => String::from(""),
                                       _ => self.state.fractal_color[2].to_string()
                                   },
                                   |txt| Message::SettingChange(SettingChange::FractalColor(txt,2))
                               )
                               .into()
                           ]).spacing(10).width(Length::FillPortion(2))
                           .into()
                       ])
                   )
                   .into(),


                   container(
                       row(vec![
                           container(text("Dot Size (px)").size(20))
                           .width(Length::FillPortion(3)).center_y().height(30).into(),
                           row(vec![
                               slider(
                                   1u32..=50u32,
                                   if self.state.dot_size > 5.0 {
                                       50
                                   } else {
                                       (self.state.dot_size * 10.0) as u32
                                   },
                                   |n| Message::SettingChange(SettingChange::DotSize(n.to_string()))
                               ).width(Length::FillPortion(2)).into(),
                               text_input(
                                   "Size",
                                   &(self.state.dot_size * 10.0).to_string(),
                                   |txt| Message::SettingChange(SettingChange::DotSize(txt))
                               ).width(Length::FillPortion(1)).into()
                           ]).spacing(10).width(Length::FillPortion(2)).into()
                       ])
                   ).into(),
                   text("Number of Points:").size(20).into(),
                   container(
                       row(vec![
                           container(text("Test Render (max 25k)").size(20))
                           .width(Length::FillPortion(3)).center_y().height(30).into(),
                           row(vec![
                               text_input(
                                   "0",
                                   &match self.state.n_points {
                                       0 => String::from(""),
                                       _ => self.state.n_points.to_string()
                                   },
                                   |txt| Message::SettingChange(SettingChange::NPointsInRender(txt))
                               )
                               .into(),
                           ]).spacing(10).width(Length::FillPortion(2))
                           .into()
                       ])
                   )
                   .into(),
                   container(
                       row(vec![
                           container(text("Final Render (max 500k)").size(20))
                           .width(Length::FillPortion(3)).center_y().height(30).into(),
                           row(vec![
                               text_input(
                                   "0",
                                   &match self.state.n_points_final {
                                       0 => String::from(""),
                                       _ => self.state.n_points_final.to_string()
                                   },
                                   |txt| Message::SettingChange(SettingChange::NPointsInFinal(txt))
                               )
                               .into(),
                           ]).spacing(10).width(Length::FillPortion(2))
                           .into()
                       ])
                   )
                   .into(),
                   container(
                       button("Reset to Default")
                       .style(theme::Button::Destructive)
                       .on_press(Message::SettingChange(SettingChange::ResetToDefault))
                   ).width(Length::Fill).center_x().center_y()
                   .into(),
               ]).width(Length::Fill).padding([0,25,5,25]).spacing(20).align_items(Alignment::Center)))
               .height(Length::FillPortion(1)).width(Length::Fill).max_width(400)
               .into(),
               vertical_space(40).into(),
           ]).into()
          
       ])).into();
       }
       content
       //content.explain(Color::BLACK)
   }


   fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
       self.renderer.clear();
       match message {
           Message::UpdateCanvas => {
               self.state.number_of_updates += 1;
           },
           Message::AddTransformaton(transformation) => self.add_transformation(transformation),
           // settings
           Message::SettingChange(s) => {
               match s {
                   SettingChange::StartFinalRender => {
                       self.state.do_final_render = !self.state.do_final_render;
                   }
                   SettingChange::BackgroundColor(val, i) => {
                       let val_usize: usize = match val.parse() {
                           Ok(n) => n,
                           Err(_) => return Command::none(),
                       };
                       let val_u8 = if val_usize > 256 {255} else {val_usize as u8};
                       self.state.background_color[i] = val_u8;
                   },
                   SettingChange::FractalColor(val,i) => {
                       let val_usize: usize = match val.parse() {
                           Ok(n) => n,
                           Err(_) => return Command::none(),
                       };
                       let val_u8 = if val_usize > 256 {255} else {val_usize as u8};
                       self.state.fractal_color[i] = val_u8;
                   },
                   SettingChange::DotSize(val) => {
                       let val_u32: u32 = match val.parse() {
                           Ok(n) => n,
                           Err(_) => return Command::none()
                       };
                       let val_f64: f32 = (val_u32 as f32)/10.0;
                       self.state.dot_size = val_f64;
                   },
                   SettingChange::NPointsInRender(val) => {
                       let mut val_usize: usize = match val.parse() {
                           Ok(n) => n,
                           Err(_) => return Command::none(),
                       };
                       if val_usize > 25_000 {
                           val_usize = 25_000
                       }
                       self.state.n_points = val_usize;
                   },
                   SettingChange::NPointsInFinal(val) => {
                       let mut val_usize: usize = match val.parse() {
                           Ok(n) => n,
                           Err(_) => return Command::none(),
                       };
                       if val_usize > 500_000 {
                           val_usize = 500_000
                       }
                       self.state.n_points_final = val_usize;
                   },
                   SettingChange::ResetToDefault => {
                       self.state = RendererState::default()
                   },
               }
           },
           Message::ActionChange(a) => {
               // a_index = action index
               // s_index = step index
               // t_index = tuple index (for x,y steps)
               match a {
                   ActionChange::AddNewAction => {
                       let new_action = Transformation::new(false);
                       self.transformations.push(new_action);
                   },
                   ActionChange::AddNewStep(a_index, step_type) => {
                       match step_type {
                           TransformationType::Dilation(_) => self.transformations[a_index].dilate([1.0,1.0]),
                           TransformationType::Shear(_) => self.transformations[a_index].shear([0.0,0.0]),
                           TransformationType::Reflection(_) => self.transformations[a_index].reflect(0.0),
                           TransformationType::Rotation(_) => self.transformations[a_index].rotate(0.0),
                       }
                       self.state.action_add_step_opened = None
                   },
                   ActionChange::DeleteAction(a_index) => {
                       self.transformations.remove(a_index);
                   },
                   ActionChange::DeleteStep(a_index, s_index) => {
                       if let Err(e) = self.transformations[a_index].remove_action(s_index) {
                           panic!("{}", e)
                       }
                   },
                   ActionChange::EditOneVarStep(val, a_index, s_index) => {
                       let mut val_isize: isize = match val.parse() {
                           Ok(n) => n,
                           Err(_) => if val.len() <= 0 || val == String::from("-") {
                               0
                           } else {return Command::none()}
                       };
                       let val_f64: f64 = (val_isize as f64);
                       let actn = &mut self.transformations[a_index].actions[s_index];
                       match actn {
                           &mut TransformationType::Rotation(_) => *actn = TransformationType::Rotation(val_f64),
                           &mut TransformationType::Reflection(_) => *actn = TransformationType::Reflection(val_f64),
                           _ => panic!("update->EditOneVarStep: Step selected has two values! use EditTwoVarStep instead.")
                       }
                   },
                   ActionChange::EditTranslation(val, t_index, a_index) => {
                       let mut val_isize: isize = match val.parse() {
                           Ok(n) => n,
                           Err(_) => if val.len() <= 0 || val == String::from("-") {
                              0
                           } else {return Command::none()}
                       };
                       if val_isize.abs() > 100 {
                           let val_sign = if val_isize.is_positive() {1} else {-1};
                           val_isize = 100 * val_sign;
                       }
                       let val_f64: f64 = (val_isize as f64)/100.0;
                       self.transformations[a_index].translation[t_index] = val_f64;
                   },
                   ActionChange::EditTwoVarStep(val, t_index, a_index, s_index) => {
                       let mut val_isize: isize = match val.parse() {
                           Ok(n) => n,
                           Err(_) => if val.len() <= 0 || val == String::from("-") {
                               0
                           } else {return Command::none()}
                       };
                       if val_isize.abs() > 400 {
                           let val_sign = if val_isize.is_positive() {1} else {-1};
                           val_isize = 400 * val_sign;
                       }
                       let val_f64: f64 = (val_isize as f64)/100.0;
                       let actn = &mut self.transformations[a_index].actions[s_index];
                       match actn {
                           &mut TransformationType::Dilation(tuple) => {
                               let mut tupcopy = tuple.clone();
                               tupcopy[t_index] = val_f64;
                               *actn = TransformationType::Dilation(tupcopy)
                           },
                           &mut TransformationType::Shear(tuple) => {
                               let mut tupcopy = tuple.clone();
                               tupcopy[t_index] = val_f64;
                               *actn = TransformationType::Shear(tupcopy)
                           },
                           _ => panic!("update->EditTwoVarStep: Step selected has one value! use EditOneVarStep instead.")
                       }
                   }
                   ActionChange::ToggleRadians(using_rad, a_index) => {
                       self.transformations[a_index].using_rad = using_rad;
                   },
                   ActionChange::OpenAddStepMenu(a_index) => {
                       self.state.action_add_step_opened = Some(a_index);
                   },
               }
           }
       };
       Command::none()
   }
}


impl<Message> canvas::Program<Message> for Renderer {
   type State = ();


   fn draw(
           &self,
           state: &Self::State,
           theme: &Theme,
           bounds: Rectangle,
           cursor: Cursor,
       ) -> Vec<Geometry> {
           if self.state.do_final_render {
               for (index, trnsfrm) in self.transformations.iter().enumerate() {
                   let (mata, matb) = trnsfrm.build();
                   println!("Action {index}:");
                   mata.print();
                   matb.print();
                   println!("Probability: {}", mata.det().abs());
                   println!("Steps: {:?}", trnsfrm.actions)
               }
           }
           let [pnt_r, pnt_g, pnt_b] = self.state.fractal_color;
           let [bg_r, bg_g, bg_b] = self.state.background_color;
           let [screen_x, screen_y] = match self.state.do_final_render {
               true => self.state.final_screen_size,
               false => self.state.screen_size,
           };
           let n_points = match self.state.do_final_render {
               true => self.state.n_points_final,
               false => self.state.n_points
           };
           let points = self.create_points(
               (0.0,0.0),
               n_points,
               [screen_x, screen_y]
           );
           let fractal = self.renderer.draw(bounds.size(), |frame| {
               let bg = Path::rectangle(Point::from([0,0]),Size::new(screen_x * 1.5,screen_y * 1.5));
               frame.fill(&bg, Color::from_rgb8(bg_r, bg_g, bg_b));
               for (p_x, p_y) in points {
                   let new_point = Path::line(Point::new(p_x as f32, p_y as f32),Point::new((p_x + 1.0) as f32, (p_y + 1.0) as f32));
                   frame.stroke(&new_point,Stroke {
                       style: canvas::Style::Solid(Color::from_rgb8(pnt_r, pnt_g, pnt_b)),
                       width: self.state.dot_size,
                       line_cap: LineCap::Round,
                       line_join: canvas::LineJoin::Round,
                       line_dash: LineDash {segments: &[1.0], offset: 0}
                   });
                  
               }
           });
           vec![fractal]
   }
}


impl Renderer {
   pub fn add_transformation(&mut self, transformation: Transformation) {
       self.transformations.push(transformation);
   }

   pub fn create_points(
       &self,
       start_point: (f64, f64),
       n_points: usize,
       screen_size: [f32; 2]
   ) -> Vec<(f64, f64)> {
       let trans_mats: Vec<(Matrix,Matrix)> = self.transformations.iter().map(|t| t.build()).collect();
       let trans_weights: Vec<f64> = trans_mats.iter().map(|(trnsfrm, _)| trnsfrm.det().abs()).collect();
       let dist = WeightedIndex::new(&trans_weights).unwrap();
       let mut rng = rand::thread_rng();
       let mut current_pos = Matrix::new(vec![
           vec![start_point.0],
           vec![start_point.1],
       ]);
       let mut point_list: Vec<(f64,f64)> = vec![];
       for _ in 0..n_points {
           let (transf, transl) = trans_mats[dist.sample(&mut rng)].clone();
           current_pos = transf.mul_matrix(&current_pos);
           current_pos = current_pos.plus_matrix(&transl);
           let unwrapped = current_pos.m().clone();
           let cur_x = unwrapped[0][0] * (screen_size[0] as f64) + 50.0;
           let cur_y = screen_size[1] as f64 - (unwrapped[1][0] * (screen_size[1] as f64) - 50.0);
           point_list.push((cur_x, cur_y));
       }
       point_list
   }
  
}
