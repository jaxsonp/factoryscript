pub static mut COLOR_OUTPUT: bool = false;
pub static mut DEBUG_LEVEL: u8 = 0;

use std::cmp::min;

pub mod builtins;
pub mod error;
pub mod macros;
pub mod preprocessor;
pub mod runtime;

use core::*;
use error::{Error, ErrorType::*};

pub type Namespace = Vec<&'static StationType<'static>>;

pub fn run<'a>(src: &String) -> Result<(), Error> {
    debug!(2, "Initializing namespace...");
    let mut namespace: Namespace = Vec::new();
    for name in (*builtins::MANIFEST).iter() {
        namespace.push(name);
    }

    debug!(2, "Preprocessing...");
    let lines: Vec<&str> = src.split('\n').collect();
    let (mut stations, start_i, assign_table) = preprocessor::process(&lines, &namespace)?;

    debug!(2, "Starting");
    runtime::execute(&mut stations, start_i, &assign_table)?;
    Ok(())
}

/// Instance of a station
#[derive(Debug)]
pub struct Station {
    /// Location of the station in source code
    pub loc: SourceLocation,
    /// Station functionality and type information
    pub logic: &'static StationType<'static>,
    /// Modifiers duh
    pub modifiers: StationModifiers,
    /// Queues for each input bay
    pub in_bays: Vec<Option<Pallet>>,
    /// Map of each output bay connection in the form (station_index, in_bay_index)
    pub out_bays: Vec<(usize, usize)>,
}
impl Station {
    pub fn new(
        identifier: &str,
        loc: SourceLocation,
        modifiers: StationModifiers,
        ns: &Namespace,
    ) -> Result<Self, Error> {
        for station_type in ns {
            if station_type.has_id(identifier) {
                return Ok(Self {
                    loc,
                    logic: station_type,
                    modifiers,
                    in_bays: Vec::new(),
                    out_bays: Vec::new(),
                });
            }
        }
        return Err(Error::new(
            IdentifierError,
            loc,
            format!("Failed to find station type with identifier \"{identifier}\"").as_str(),
        ));
    }

    pub fn clear_in_bays(&mut self) {
        for bay in self.in_bays.iter_mut() {
            if bay.is_some() {
                *bay = None;
            }
        }
    }
}

/// Struct for holding the modifiers of an instance of a station
#[derive(Debug)]
pub struct StationModifiers {
    /// Reverse input precedence (false=cw, true=ccw)
    pub reverse: bool,
    /// Which direction the precedence starts with
    pub priority: Direction,
}
impl StationModifiers {
    /// Default modifiers for a station
    pub fn default() -> Self {
        Self {
            reverse: false,
            priority: Direction::NORTH,
        }
    }
    /// toggles the reverse direction modifier
    pub fn reverse(self) -> Self {
        Self {
            reverse: !self.reverse,
            ..self
        }
    }
    /// sets the direction with priority to a new value
    pub fn with_priority(self, new_priority: Direction) -> Self {
        Self {
            priority: new_priority,
            ..self
        }
    }
}

/// Defines the position of a span of characters in the source code, used for
/// syntax parsing and error reporting
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SourceLocation {
    /// line number
    pub line: usize,
    /// column number
    pub col: usize,
    /// length of span
    pub len: usize,
}
impl SourceLocation {
    /// Value to represent if the source location is not applicable
    pub fn none() -> Self {
        Self {
            line: 0,
            col: 0,
            len: 0,
        }
    }
}
impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}-{}", self.line + 1, self.col, self.col + self.len)
    }
}

/// Helper for the cardinal directions
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}
impl std::ops::Not for Direction {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Direction::NORTH => Direction::SOUTH,
            Direction::EAST => Direction::WEST,
            Direction::SOUTH => Direction::NORTH,
            Direction::WEST => Direction::EAST,
        }
    }
}
impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::NORTH => "north",
                Direction::EAST => "east",
                Direction::SOUTH => "south",
                Direction::WEST => "west",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_negation() {
        assert_eq!(!Direction::NORTH, Direction::SOUTH);
        assert_eq!(!Direction::EAST, Direction::WEST);
        assert_eq!(!Direction::SOUTH, Direction::NORTH);
        assert_eq!(!Direction::WEST, Direction::EAST);
    }

    #[test]
    fn test_station_clear_in_bays() {
        let mut station = Station::new(
            "joint",
            SourceLocation::none(),
            StationModifiers::default(),
            &builtins::MANIFEST,
        )
        .unwrap();
        station.in_bays.push(Some(Pallet::Empty));
        station.in_bays.push(Some(Pallet::Int(3)));
        station.in_bays.push(Some(Pallet::Char('a')));
        station.clear_in_bays();
        assert!(station.in_bays[0].is_none());
        assert!(station.in_bays[1].is_none());
        assert!(station.in_bays[2].is_none());
    }

    #[test]
    fn test_station_modifiers() {
        assert!(matches!(
            StationModifiers::default(),
            StationModifiers {
                reverse: false,
                priority: Direction::NORTH
            }
        ));
        assert!(matches!(
            StationModifiers::default().reverse(),
            StationModifiers {
                reverse: true,
                priority: Direction::NORTH
            }
        ));
        assert!(matches!(
            StationModifiers::default().with_priority(Direction::SOUTH),
            StationModifiers {
                reverse: false,
                priority: Direction::SOUTH
            }
        ));
        assert!(matches!(
            StationModifiers::default()
                .reverse()
                .with_priority(Direction::EAST),
            StationModifiers {
                reverse: true,
                priority: Direction::EAST
            }
        ));
    }
}
