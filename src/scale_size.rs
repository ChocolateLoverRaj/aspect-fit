use std::ops::{Div, Mul};

use num::NumCast;

use crate::size::Size;

pub fn scale_size<Scale, Dimension>(scale: &Scale, size: &Size<Dimension>) -> Size<Dimension>
where
    for<'a> &'a Scale: Mul<&'a Scale, Output = Scale>,
    Scale: NumCast + Copy + Div<Scale, Output = Scale>,
    for<'b> &'b Dimension: Mul<&'b Dimension, Output = Dimension>,
    Dimension: NumCast + Ord + Copy,
{
    Size {
        width: Dimension::from(scale * Scale::from(size.width).as_ref().unwrap()).unwrap(),
        height: Dimension::from(scale * Scale::from(size.height).as_ref().unwrap()).unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale() {
        let result = scale_size(
            &1.1,
            &Size {
                width: 100,
                height: 20,
            },
        );
        assert_eq!(result.width, 110);
        assert_eq!(result.height, 22);
    }

    #[test]
    fn scale_custom_types() {
        let result = scale_size(
            &(1.1 as f32),
            &Size::<i128> {
                width: 100,
                height: 20,
            },
        );
        assert_eq!(result.width, 110);
        assert_eq!(result.height, 22);
    }
}
