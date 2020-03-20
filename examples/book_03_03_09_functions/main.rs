use rrt_lib::data::{Event, ExternalEvent, LifetimeTrait, ResourceOwner, Variable, Function, Visualizable, VisualizationData};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;
// visualization of shadowing
// Ch 3.3
fn main() {
    let x = Some(ResourceOwner::Variable(Variable {
        hash: 1,
        name: String::from("x"),
        is_mut: false,
        is_ref: false,
        is_func: false,
        lifetime_trait: LifetimeTrait::None,
    }));
    let plus_one = Some(ResourceOwner::Function(Function {
        hash: 2,
        name: String::from("plus_one()"),
    }));
    let plus_one_x = Some(ResourceOwner::Variable(Variable {
        hash: 3,
        name: String::from("x"),
        is_mut: false,
        is_ref: false,
        is_func: false,
        lifetime_trait: LifetimeTrait::Copy,
    }));
    let println = Some(ResourceOwner::Function(Function {
        hash: 4,
        name: String::from("println!"),
    }));
    // treat operator as function
    let plus = Some(ResourceOwner::Function(Function {
        hash: 5,
        name: String::from("+"),
    }));

    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };
    

    vd.append_external_event(ExternalEvent::Duplicate{ from: None, to: plus_one.to_owned() }, &(2 as usize));
    vd.append_external_event(ExternalEvent::Duplicate{ from: plus_one.to_owned(), to: x.to_owned() }, &(2 as usize));
    vd.append_external_event(ExternalEvent::StaticBorrow{ from: x.to_owned(), to: println.to_owned() },  &(4 as usize));
    vd.append_external_event(ExternalEvent::StaticReturn{ from: println.to_owned(),  to: x.to_owned() },  &(4 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : x.unwrap().to_owned() },  &(5 as usize));
    
    // in fn plus_one:
    vd.append_external_event(ExternalEvent::Duplicate{from: None, to: plus_one_x.to_owned()}, &(7 as usize));
    vd.append_external_event(ExternalEvent::StaticBorrow{ from: plus_one_x.to_owned(), to: plus.to_owned() },  &(8 as usize));
    vd.append_external_event(ExternalEvent::StaticReturn{ from: plus.to_owned(),  to: plus_one_x.to_owned() },  &(8 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : plus_one_x.unwrap().to_owned() },  &(9 as usize));
    /* old version
    //
    // hash x : 1
    //      plus_one : 2
    // functions: 1
    vd.append_event(&plus_one, Event::Acquire { from: None }, &(2 as usize));
    // Mark event: "x" acquire value x+1
    vd.append_event(&x, Event::Acquire { from: Some(plus_one.clone()) }, &(2 as usize));
    vd.append_event(&plus_one, Event::Move { to: Some(x.clone()) }, &(2 as usize));
    vd.append_event(&plus_one, Event::GoOutOfScope, &(2 as usize));
    //x goes out of scope
    vd.append_event(&x, Event::GoOutOfScope, &(5 as usize));
    //plus_one function call
    vd.append_event(&plus_one_x, Event::Acquire { from: None }, &(7 as usize));
    vd.append_event(&plus_one_x, Event::GoOutOfScope, &(9 as usize));
    */
    //rendering image
    svg_generation::render_svg(&"book_03_03_09_functions".to_owned(), &vd);
}
