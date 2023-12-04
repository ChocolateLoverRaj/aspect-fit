use num::NumCast;

pub struct Size<Dimension>
where
    Dimension: NumCast + Ord,
{
    pub width: Dimension,
    pub height: Dimension,
}
