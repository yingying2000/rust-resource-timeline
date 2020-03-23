use rrt_lib::data::{ExternalEvent, LifetimeTrait, ResourceOwner, Variable, Function, Visualizable, VisualizationData};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;

fn main() {
    // Variables
    let s1 = Some(ResourceOwner::Variable(Variable {
        hash: 1,
        name: String::from("s1"),
        is_mut: false,
        is_ref: false,
        lifetime_trait: LifetimeTrait::Move,
    }));
    let len = Some(ResourceOwner::Variable(Variable {
        hash: 2,
        name: String::from("len"),
        is_mut: false,
        is_ref: false,
        lifetime_trait: LifetimeTrait::Copy,
    }));
    let s = Some(ResourceOwner::Variable(Variable {
        hash: 4,
        name: String::from("s"),
        is_mut: false,
        is_ref: true,
        lifetime_trait: LifetimeTrait::Move,
    }));
    // Functions
    let calculate_length = Some(ResourceOwner::Function(Function {
        hash: 3,
        name: String::from("calculate_length"),
    }));
    let string_ctor = Some(ResourceOwner::Function(Function {
        hash: 5,
        name: String::from("String::from"),
    }));
    let len_func = Some(ResourceOwner::Function(Function {
        hash: 6,
        name: String::from("len"),
    }));
    // Data
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };
    
    // s1 gets resource from String::from
    vd.append_external_event(ExternalEvent::Move{from: string_ctor.clone(), to: s1.clone()}, &(2 as usize));
    vd.append_external_event(ExternalEvent::PassByStaticReference{from: s1.clone(), to: calculate_length.clone()}, &(4 as usize));

    vd.append_external_event(ExternalEvent::Duplicate{from: calculate_length.clone(), to: len.clone()}, &(4 as usize));

    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : s1.unwrap().clone() },  &(7 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : len.unwrap().clone() },  &(7 as usize));

    // vd.append_external_event(ExternalEvent::StaticBorrow{from: Some(calculate_length.clone()), to: Some(s.clone())}, &(9 as usize));
    
    // len(&self) -> usize
    // s is a reference copied from god knows where
    vd.append_external_event(ExternalEvent::Duplicate{ from: None, to: s.clone() }, &(9 as usize));
    vd.append_external_event(ExternalEvent::StaticBorrow{from: s.clone(), to: len_func.clone()}, &(10 as usize));
    vd.append_external_event(ExternalEvent::StaticReturn{from: len_func.clone(), to: s.clone()}, &(10 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : s.unwrap().clone() },  &(11 as usize));

    
    
    //rendering image
    svg_generation::render_svg(&"04_02_01".to_owned(), &"pass_reference".to_owned(), &vd);
}