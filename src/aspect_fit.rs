use num::NumCast;
use std::ops::{Div, Mul};

use crate::size::Size;

pub fn aspect_fit<Scale, Dimension>(
    rect_to_scale: &Size<Dimension>,
    container_rect: &Size<Dimension>,
) -> Scale
where
    for<'a> &'a Scale: Mul<&'a Scale, Output = Scale>,
    Scale: NumCast + Copy + Div<Scale, Output = Scale>,
    for<'b> &'b Dimension: Mul<&'b Dimension, Output = Dimension>,
    Dimension: NumCast + Ord + Copy,
{
    let ref multiplier_for_comparison =
        Scale::from(&rect_to_scale.width * &rect_to_scale.height).unwrap();
    let ref width_scale =
        Scale::from(container_rect.width).unwrap() / Scale::from(rect_to_scale.width).unwrap();
    let ref height_scale =
        Scale::from(container_rect.height).unwrap() / Scale::from(rect_to_scale.height).unwrap();
    if Dimension::from(width_scale * multiplier_for_comparison)
        < Dimension::from(height_scale * multiplier_for_comparison)
    {
        *width_scale
    } else {
        *height_scale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn height_limits() {
        let result: f64 = aspect_fit(
            &Size {
                width: 5,
                height: 10,
            },
            &Size {
                width: 20,
                height: 20,
            },
        );
        assert_eq!(result, 2 as f64)
    }

    #[test]
    fn width_limits() {
        let result: f64 = aspect_fit(
            &Size {
                width: 10,
                height: 5,
            },
            &Size {
                width: 20,
                height: 20,
            },
        );
        assert_eq!(result, 2 as f64)
    }

    #[test]
    fn perfect_fit() {
        let result: f64 = aspect_fit(
            &Size {
                width: 30,
                height: 20,
            },
            &Size {
                width: 45,
                height: 40,
            },
        );
        assert_eq!(result, 1.5)
    }

    #[test]
    fn scale_down() {
        let result: f64 = aspect_fit(
            &Size {
                width: 100,
                height: 200,
            },
            &Size {
                width: 50,
                height: 50,
            },
        );
        assert_eq!(result, 0.25)
    }

    #[test]
    fn custom_types() {
        let result = aspect_fit::<f64, i128>(
            &Size {
                width: 30,
                height: 20,
            },
            &Size {
                width: 45,
                height: 40,
            },
        );
        assert_eq!(result, 1.5)
    }
}
