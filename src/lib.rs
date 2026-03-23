use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Pos<T> {
    x: T,
    y: T,
}

impl<T> Pos<T>
where
    T: Copy,
{
    #[must_use]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub const fn x(self) -> T {
        self.x
    }

    #[must_use]
    pub const fn y(self) -> T {
        self.y
    }
}

impl<T> Add<Self> for Pos<T>
where
    T: Add<T, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign<Self> for Pos<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub<Self> for Pos<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> SubAssign<Self> for Pos<T>
where
    T: SubAssign<T>,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

macro_rules! impl_pos_signed {
    ($($t:ty),*) => {
        $(
            impl Pos<$t> {
                #[must_use]
                pub const fn up(self) -> Self {
                    Self { y: self.y - 1, ..self }
                }

                #[must_use]
                pub const fn right(self) -> Self {
                    Self { x: self.x + 1, ..self }
                }

                #[must_use]
                pub const fn down(self) -> Self {
                    Self { y: self.y + 1, ..self }
                }

                #[must_use]
                pub const fn left(self) -> Self {
                    Self { x: self.x - 1, ..self }
                }

                pub fn adj(self) -> impl Iterator<Item = Self> {
                    [self.up(), self.right(), self.down(), self.left()].into_iter()
                }
            }
        )*
    };
}

macro_rules! impl_pos_unsigned {
    ($($t:ty),*) => {
        $(
            impl Pos<$t> {
                #[must_use]
                pub fn up(self) -> Option<Self> {
                    Some(Self { y: self.y.checked_sub(1)?, ..self })
                }

                #[must_use]
                pub fn right(self) -> Option<Self> {
                    Some(Self { x: self.x.checked_add(1)?, ..self })
                }

                #[must_use]
                pub fn down(self) -> Option<Self> {
                    Some(Self { y: self.y.checked_add(1)?, ..self })
                }

                #[must_use]
                pub fn left(self) -> Option<Self> {
                    Some(Self { x: self.x.checked_sub(1)?, ..self })
                }

                pub fn adj(self) -> impl Iterator<Item = Self> {
                    [self.up(), self.right(), self.down(), self.left()]
                        .into_iter()
                        .flatten()
                }
            }
        )*
    };
}

impl_pos_signed!(i8, i16, i32, i64, i128, isize);
impl_pos_unsigned!(u8, u16, u32, u64, u128, usize);
