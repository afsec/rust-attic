/*

type Point coordinateSystem = Point Float Float

type WorldCoordinate = WorldCoordinate Never

type ScreenCoordinate = ScreenCoordinate Never

playerStartPos : Point WorldCoordinate
playerStartPos = Point 2 3

healthBarPos : Point ScreenCoordinate
healthBarPos = Point 200 10
*/

use std::{fmt::Debug, marker::PhantomData};

trait NextState<T>
where
    Self: Debug,
{
    fn next_state(self) -> Self;
}

#[derive(Debug)]
struct WorldCoordinate;

impl NextState<WorldCoordinate> for Point<WorldCoordinate> {
    fn next_state(self) -> Self {
        let Self { mut x, mut y, kind } = self;
        x *= 5.0;
        y *= 5.0;
        Self { x, y, kind }
    }
}

#[derive(Debug)]
struct ScreenCoordinate;
impl NextState<ScreenCoordinate> for Point<ScreenCoordinate> {
    fn next_state(self) -> Self {
        let Self { mut x, mut y, kind } = self;
        x *= 2.0;
        y *= 2.0;
        Self { x, y, kind }
    }
}

#[derive(Debug)]
struct Point<T> {
    x: f32,
    y: f32,
    kind: T,
}

#[derive(Debug)]
struct Destination<T> {
    address: String,
    kind: T,
}


impl NextState<WorldCoordinate> for Destination<WorldCoordinate> {
    fn next_state(self) -> Self {
        let Self { mut address, kind } = self;
        address.push_str(" (Arrived - WC)");

        Self { address, kind }
    }
}


impl NextState<ScreenCoordinate> for Destination<ScreenCoordinate> {
    fn next_state(self) -> Self {
        let Self { mut address, kind } = self;
        address.push_str(" (Arrived - SC)");

        Self { address, kind }
    }
}

fn main() {
    println!("Hello, world!");
    let x = get_point(WorldCoordinate);
    dbg!(&x);
    dbg!(x.next_state());

    let x = get_destination(WorldCoordinate);
    dbg!(&x);
    dbg!(x.next_state());

    let y = get_point(ScreenCoordinate);
    dbg!(&y);
    dbg!(y.next_state());

    let y = get_destination(ScreenCoordinate);
    dbg!(&y);
    dbg!(y.next_state());
}

fn get_point<T>(kind: T) -> impl NextState<T>
where
    Point<T>: NextState<T>,
{
    Point {
        x: 0.1, // From network
        y: 0.1, // From network
        kind,
    }
}

fn get_destination<T>(kind: T) -> impl NextState<T>
where
    Destination<T>: NextState<T>,
{
    Destination {
        address: "Rua X, numero Y".into(), // From network
        kind,
    }
}
