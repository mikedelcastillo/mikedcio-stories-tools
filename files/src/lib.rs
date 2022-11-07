pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// struct Progress<'a> {
//     text: &'a str,
//     step_current: u32,
//     step_total: u32,
// }

// impl<'a> Progress<'a> {
//     fn new(step_total: u32) -> Self {
//         Self {
//             text: "",
//             step_current: 0,
//             step_total,
//         }
//     }

//     fn set_text(&mut self, text: &'a str) -> &Self {
//         self.text = text;
//         self
//     }

//     fn step(&mut self, text: &'a str) -> &Self {
//         self.set_text(text);
//         self.step_current += 1;
//         self
//     }

//     fn get_percent(&self) -> f32 {
//         (self.step_current as f32) / (self.step_total as f32)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
