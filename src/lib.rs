#![no_std]

mod fixed_vec_deque;

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::DrawTarget,
    primitives::{Circle, Primitive, PrimitiveStyle},
    Drawable,
};
use fixed_vec_deque::FixedVecDeque;

use rand::{rngs::SmallRng, Rng, SeedableRng};

// Definitions of the game objects
#[derive(Debug)]
pub struct Game<const N: usize> {
    pub snake: Snake<N>,
    pub food: Food,
    pub score: u32,
    pub game_over: bool,
    pub bounds: (u32, u32),
    rng: rand::rngs::SmallRng,
}
#[derive(Debug)]
pub struct Snake<const N: usize> {
    pub body: FixedVecDeque<Point, N>,
    pub direction: Direction,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub struct Food {
    pub position: Point,
}

pub enum Update {
    Snake(Point, Point),
    Food(Point, Point),
    None,
}

// Implementation of the game objects

impl<const N: usize> Game<N> {
    pub fn new(bounds: (u32, u32)) -> Self {
        let mut snake = Snake {
            body: FixedVecDeque::new(),
            direction: Direction::Right,
        };
        snake.body.push_back(Point {
            x: bounds.0 / 2,
            y: bounds.1 / 2,
        });

        let food = Food {
            position: Point {
                x: (bounds.0 / 2) + 3,
                y: bounds.1 / 2,
            },
        };

        Self {
            rng: SmallRng::seed_from_u64(0),
            snake,
            food,
            score: 0,
            game_over: false,
            bounds,
        }
    }

    pub fn reset(&mut self) {
        self.snake = Snake {
            body: FixedVecDeque::new(),
            direction: Direction::Right,
        };
        self.snake.body.push_back(Point {
            x: self.bounds.0 / 2,
            y: self.bounds.1 / 2,
        });

        self.food = Food {
            position: Point {
                x: (self.bounds.0 / 2) + 3,
                y: self.bounds.1 / 2,
            },
        };

        self.score = 0;
        self.game_over = false;
    }

    pub fn handle_input(&mut self, input: Direction) {
        match input {
            Direction::Up => {
                if self.snake.direction != Direction::Down {
                    self.snake.direction = Direction::Up;
                }
            }
            Direction::Down => {
                if self.snake.direction != Direction::Up {
                    self.snake.direction = Direction::Down;
                }
            }
            Direction::Left => {
                if self.snake.direction != Direction::Right {
                    self.snake.direction = Direction::Left;
                }
            }
            Direction::Right => {
                if self.snake.direction != Direction::Left {
                    self.snake.direction = Direction::Right;
                }
            }
        }
    }

    fn generate_food(&mut self) -> Point {
        let mut x = self.rng.gen_range(0..self.bounds.0);
        let mut y = self.rng.gen_range(0..self.bounds.1);

        while self.snake.body.contains(&Point { x, y }) {
            x = self.rng.gen_range(0..self.bounds.0);
            y = self.rng.gen_range(0..self.bounds.1);
        }

        Point { x, y }
    }

    pub fn update(&mut self) -> Update {
        let mut new_head = self.snake.body.front().unwrap().clone();

        match self.snake.direction {
            Direction::Up => {
                if new_head.y == 0 {
                    new_head.y = self.bounds.1 - 1;
                } else {
                    new_head.y -= 1;
                }
            }
            Direction::Down => {
                if new_head.y == self.bounds.1 - 1 {
                    new_head.y = 0;
                } else {
                    new_head.y += 1;
                }
            }
            Direction::Left => {
                if new_head.x == 0 {
                    new_head.x = self.bounds.0 - 1;
                } else {
                    new_head.x -= 1;
                }
            }
            Direction::Right => {
                if new_head.x == self.bounds.0 - 1 {
                    new_head.x = 0;
                } else {
                    new_head.x += 1;
                }
            }
        }

        if self.snake.body.contains(&new_head) {
            self.game_over = true;
            return Update::None;
        }

        self.snake.body.push_front(new_head);

        if self.snake.body.front().unwrap() == &self.food.position {
            self.score += 1;
            let old_food = self.food.position.clone();
            self.food.position = self.generate_food();

            Update::Food(self.food.position.clone(), old_food)
        } else {
            Update::Snake(
                self.snake.body.front().unwrap().clone(),
                self.snake.body.pop_back().unwrap(),
            )
        }
    }
}

// Game rendering

impl<const N: usize> Drawable for Game<N> {
    type Color = BinaryColor;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.snake.draw(display)?;
        self.food.draw(display)?;

        Ok(())
    }
}

impl<const N: usize> Drawable for Snake<N> {
    type Color = BinaryColor;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        for point in self.body.iter() {
            Circle::with_center(point.into(), 4)
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
                .draw(display)?;
        }

        Ok(())
    }
}

impl Drawable for Food {
    type Color = BinaryColor;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        Circle::with_center((&self.position).into(), 8)
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
            .draw(display)?;

        Ok(())
    }
}

impl From<&Point> for embedded_graphics::prelude::Point {
    fn from(val: &Point) -> Self {
        embedded_graphics::prelude::Point::new((val.x * 8) as i32 + 4, (val.y * 8) as i32 + 4)
    }
}
