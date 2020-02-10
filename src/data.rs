use std::collections::HashMap;
use std::vec::Vec;
/** 
 * Basic Data Structure Needed by Lifetime Visualization
 */

// state specific for svg rendering
pub enum OwnershipState{
          //                                   SVG LINE SPEC (plz change
    Full, // full ownership, r/w.              BOLD_SOLID_LINE
    Borrowed, // immutable borrowed, can read. SOLID_LINE
    MutablyBorrowed, // can not r/w.           DASH_LINE
    NoOwnership{ // resource transfered
        can_realloc : bool //         true: SOLID_DOTS  
                              //        false: HOLLOW_DOTS
    }
}

pub struct SvgLineInfo{
    pub start_line_number : u32,
    pub end_line_number : u32,
    pub ownership_state : OwnershipState,
}

// Top level Api that the Timeline object supports
pub trait Visualizable {
    // returns None if the hash does not exist
    fn get_name_from_hash(&self, hash: &u64) -> Option<String>;
    // returns Noneappend_event if the hash does not exist
    fn get_state(&self, hash: &u64, line_number: &usize) -> Option<State>;
    // add an event to the Visualizable data structure
    fn append_event(&mut self, ro : &ResourceOwner, event: Event, line_number: &usize);
    // add an ExternalEvent to the Visualizable data structure
    fn append_external_event(&mut self, line_number : usize, external_event: ExternalEvent); 

    // SVG left panel generation facilities
    // each line represents a state; return all states for a ResourceOwner
    fn svg_line_info(&self, hash : &u64) -> Vec<SvgLineInfo>;
    // return all states for all Resource Owner
    fn svg_line_info_all(&self) -> HashMap<u64, Vec<SvgLineInfo>>;
    // return a timeline for a single resource owner 
    fn svg_dot_info(&self, hash : &u64) -> Timeline;
    // return all timelines
    fn svg_dot_info_map(&self) -> HashMap<u64, Vec<SvgLineInfo>>;
    // return svg_arrows := {Move, Borrow, Return}
    fn svg_arrows_info(&self) -> &Vec<(usize, ExternalEvent)>;
}

// A ResourceOwner is either a Variable or a Function that 
// have ownership to a memory object, during some stage of
// a the program execution.
#[derive(Clone)]
pub struct ResourceOwner {
    pub hash: u64,
    pub name: String,
    // pub is_fun: bool,
    // pub lifetime_trait: LifetimeTrait,
}


pub enum ExternalEvent{
    // let binding
    Acquire{
        from: Option<ResourceOwner>,
        to: Option<ResourceOwner>,
    },
    // copy / clone
    Duplicate{
        from: Option<ResourceOwner>,
        to: Option<ResourceOwner>,
    },
    Move{
        from: Option<ResourceOwner>,
        to: Option<ResourceOwner>,
    },
    Borrow{
        is_mut: bool,
        from: Option<ResourceOwner>,
        to: Option<ResourceOwner>,
    },
    Return{
        // return the resource to "to"
        is_mut: bool,
        from: Option<ResourceOwner>, 
        to: Option<ResourceOwner>,
    },
    GoOutOfScope,
}

// An Event describes the acquisition or release of a 
// resource ownership by a variable on any given line. 
// There are six types of them. 
pub enum Event {
    // this happens when a variable is initiated, it should obtain
    // its resource from either another variable or from a 
    // contructor. 
    // 
    // E.g. in the case of
    //      let x = Vec::new();
    // x obtained the resource from global resource allocator,
    // the Acquire Event's "from" variable is None. 
    // in the case of 
    //      let y = x;
    // y obtained its value from x, which means that the Acquire
    // Event's "from" variable is x. 
    Acquire {
        from: Option<ResourceOwner>
    },
    // this happens when a ResourceOwner implements copy trait or
    // explicitly calls .clone() function
    // to another ResourceOwner, or a function.
    //
    // e.g.
    // 1. x: i32 = y + 15;              here y duplicate to + op, and x acquire from + 
    //                                  at this point, we treat it as y duplicates to None
    // 2. x: MyStruct = y.clone();      here y duplicates to x.
    Duplicate {
        to: Option<ResourceOwner>

    },
    // this happens when a ResourceOwner transfer the ownership of its resource
    // to another ResourceOwner, or if it is no longer used after this line.
    // Typically, this happens at one of the following two cases:
    // 
    // 1. variable is not used after this line. 
    // 2. variable's resource has the move trait, and it transfered
    //    its ownership on this line. This includes tranfering its
    //    ownership to a function as well. 
    Transfer {
        to: Option<ResourceOwner>
    },
    MutableLend {
        to: Option<ResourceOwner>
    },
    MutableBorrow{
        from: ResourceOwner
    },
    MutableReturn{
        from: Option<ResourceOwner>
    },
    MutableReacquire {
        from: Option<ResourceOwner>
    },
    StaticLend {
        to: Option<ResourceOwner>
    },
    StaticBorrow{
        from: ResourceOwner
    },
    StaticReturn{
        to: Option<ResourceOwner>
    },
    StaticReacquire {
        from: Option<ResourceOwner>
    },
    // this happens when a variable is returned this line,
    // or if this variable's scope ends at this line.
    GoOutOfScope,
}

// Trait of a resource owner that might impact the way lifetime visualization
// behaves

#[derive(Clone)]
pub enum LifetimeTrait {
    Copy,
    Move,
    None,
}

// A State is a description of a ResourceOwner IMMEDIATELY AFTER a specific line.
// We think of this as what read/write access we have to its resource.
pub enum State {
    // The viable is no longer in the scope after this line.
    OutOfScope {
        scope_termination_line: usize
    },
    // The resource is transferred on this lResourceOwnerine or before this line,
    // thus it is impossible to access this variable anymore.
    Transfered {
        to: ResourceOwner,
        transfer_line: usize
    },
    // The resource can be fully accessed right after this line. 
    ReadableAndWritable,
    // The resource can be read, but cannot be written to right after this line.
    // This also means that it is not possible to create a mutable reference
    // on the next line.
    ReadableOnly,
}


// a vector of ownership transfer history of a specific variable, 
// in a sorted order by line number.
pub struct Timeline {
    pub resource_owner: ResourceOwner,  
    // line number in usize
    pub history: Vec<(usize, Event)>,
}

// VisualizationData supplies all the information we need in the frontend, 
// from rendering a PNG to producing an interactive HTML guide. 
// The internal data is simple: a map from variable hash to its Timeline.
pub struct VisualizationData {
    pub timelines: HashMap<u64, Timeline>,
    // line_number, event
    // for svg arrows
    pub external_events: Vec<(usize, ExternalEvent)>,
}

// fulfills the promise that we can support all the methods that a 
// frontend would need. 
impl Visualizable for VisualizationData {
    fn get_name_from_hash(&self, hash: &u64) -> Option<String> {
        match self.timelines.get(hash) {
            Some(timeline) => Some(timeline.resource_owner.name.to_owned()),
            _ => None
        }
    }

    // TODO: state solving not complete
    fn get_state(&self, hash: &u64, line_number: &usize) -> Option<State> {
        match self.timelines.get(hash) {
            Some(timeline) => {
                // example return value
                Some(State::OutOfScope {scope_termination_line: 0})
            },
            _ => None
        }
    }


    // add event using (internal) events
    fn append_event(&mut self, ro : &ResourceOwner, event: Event, line_number: &usize) {
        let hash = &ro.hash;
        let var_name = &ro.name;
        // if this event belongs to a new ResourceOwner hash,
        // create a new Timeline first, thenResourceOwner bind it to the corresponding hash.
        match self.timelines.get(hash) {
            None => {
                let timeline = Timeline {
                    resource_owner: ro.clone(),
                    history: Vec::new(),

                };
                self.timelines.insert(*hash, timeline);
            },
            _ => {}
        }

        // append the event to the end of the timeline of the corresponding hash
        match self.timelines.get_mut(hash) {
            Some(timeline) => {
                timeline.history.push((*line_number, event));
            },
            _ => {
                panic!("Timeline disappeared right after creation or when we could index it. This is impossible.");
            }
        }
    }

    // TODO IMPLEMENT
    fn append_external_event(&mut self, line_number : usize, external_event: ExternalEvent){
        self.external_events.push((line_number, external_event));
    } 
    fn svg_line_info(&self, hash : &u64) -> Vec<SvgLineInfo>{
        Vec::<SvgLineInfo>::new()
    }
    // return all states for all Resource Owner
    fn svg_line_info_all(&self) -> HashMap<u64, Vec<SvgLineInfo>>{
        HashMap::new()
    }
    // return a timeline for a single resource owner 
    fn svg_dot_info(&self, hash : &u64) -> Timeline{
        let tl = Timeline{
            resource_owner : ResourceOwner {
                hash : 0,
                name : String::from("x"),
                // is_fun : false,
            },
            history : Vec::<(usize, Event)>::new(),

        };
        tl
    }

    // return all timelines
    fn svg_dot_info_map(&self) -> HashMap<u64, Vec<SvgLineInfo>>{
        HashMap::new()
    }

    // return svg_arrows := {Move, Borrow, Return}
    fn svg_arrows_info(&self) -> &Vec<(usize, ExternalEvent)> {
        &self.external_events
    }
}