use std::fmt;
use bevy::prelude::*;

#[derive(Debug)]
pub enum Hour {
  Zero,
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Eleven,
  Twelve,
  Thirteen,
  Fourteen,
  Fifteen,
  Sixteen,
  Seventeen,
  Eighteen,
  Nineteen,
  Twenty,
  TwentyOne,
  TwentyTwo,
  TwentyThree
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
pub struct HourBox {
  pub hour: Hour,
  pub active: bool,
}
impl HourBox {
  pub fn new(hour: Hour, active: bool) -> Self {
    Self {
      hour,
      active,
    }
  }
}

#[derive(Component)]
pub struct CurrentTimeText;
