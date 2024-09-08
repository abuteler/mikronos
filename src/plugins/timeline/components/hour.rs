use std::fmt;
use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum Hour {
  Zero          = 0,
  One           = 1,
  Two           = 2,
  Three         = 3,
  Four          = 4,
  Five          = 5,
  Six           = 6,
  Seven         = 7,
  Eight         = 8,
  Nine          = 9,
  Ten           = 10,
  Eleven        = 11,
  Twelve        = 12,
  Thirteen      = 13,
  Fourteen      = 14,
  Fifteen       = 15,
  Sixteen       = 16,
  Seventeen     = 17,
  Eighteen      = 18,
  Nineteen      = 19,
  Twenty        = 20,
  TwentyOne     = 21,
  TwentyTwo     = 22,
  TwentyThree   = 23,
}

impl Hour {
  pub const VALUES: [Self; 24] = [
    Self::Zero,
    Self::One,
    Self::Two,
    Self::Three,
    Self::Four,
    Self::Five,
    Self::Six,
    Self::Seven,
    Self::Eight,
    Self::Nine,
    Self::Ten,
    Self::Eleven,
    Self::Twelve,
    Self::Thirteen,
    Self::Fourteen,
    Self::Fifteen,
    Self::Sixteen,
    Self::Seventeen,
    Self::Eighteen,
    Self::Nineteen,
    Self::Twenty,
    Self::TwentyOne,
    Self::TwentyTwo,
    Self::TwentyThree
  ];
}

impl fmt::Display for Hour {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Zero => "00",
        Self::One => "01",
        Self::Two => "02",
        Self::Three => "03",
        Self::Four => "04",
        Self::Five => "05",
        Self::Six => "06",
        Self::Seven => "07",
        Self::Eight => "08",
        Self::Nine => "09",
        Self::Ten => "10",
        Self::Eleven => "11",
        Self::Twelve => "12",
        Self::Thirteen => "13",
        Self::Fourteen => "14",
        Self::Fifteen => "15",
        Self::Sixteen => "16",
        Self::Seventeen => "17",
        Self::Eighteen => "18",
        Self::Nineteen => "19",
        Self::Twenty => "20",
        Self::TwentyOne => "21",
        Self::TwentyTwo => "22",
        Self::TwentyThree => "23",
      }
    )
}
}

#[derive(Component)]
pub struct Active;


// #[derive(Component)]
// pub struct HourBox {
//   pub hour: Hour,
//   pub outer: Entity,
// }
// impl HourBox {
//   pub fn new(hour: Hour, outer: Entity) -> Self {
//     Self {
//       hour,
//       outer,
//     }
//   }
// }

#[derive(Component)]
pub struct CurrentTimeText;
