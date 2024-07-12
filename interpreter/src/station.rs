use crate::*;
use fs_core::*;

/// Instance of a station
pub struct Station {
    /// Location of the station in source code
    pub loc: SourceLocation,
    /// Station functionality and type information
    pub station_type: &'static StationType<'static>,
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
        for name in ns {
            if identifier == name.id {
                return Ok(Self {
                    loc,
                    station_type: name,
                    modifiers,
                    in_bays: Vec::new(),
                    out_bays: Vec::new(),
                });
            }
        }
        return Err(Error {
            t: ErrorType::IdentifierError,
            loc,
            msg: format!("Failed to find station type with identifier \"{identifier}\""),
        });
    }
    /*pub fn new_assign(
        value: &str,
        loc: SourceLocation,
        modifiers: StationModifiers,
    ) -> Result<Self, Error> {
        let station_type = stdlib::get_assign_station(value);
        Ok(Self {
            loc,
            station_type,
            modifiers,
            in_bays: Vec::new(),
            out_bays: Vec::new(),
        })
    }*/
    pub fn new_in_bay(&mut self) {
        self.in_bays.push(None);
    }
}

/// Struct for holding the modifiers of an instance of a station
pub struct StationModifiers {
    /// Reverse input precedence (false=cw, true=ccw)
    pub reverse: bool,
    /// Which direction the precedence starts with
    pub priority: Direction,
}
impl StationModifiers {
    pub fn default() -> Self {
        Self {
            reverse: false,
            priority: Direction::NORTH,
        }
    }
    pub fn reverse(self) -> Self {
        Self {
            reverse: !self.reverse,
            ..self
        }
    }
    pub fn with_priority(self, new_priority: Direction) -> Self {
        Self {
            priority: new_priority,
            ..self
        }
    }
}