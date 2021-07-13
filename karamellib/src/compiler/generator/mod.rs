use std::{borrow::Borrow, cell::RefCell, collections::VecDeque, rc::Rc};

use crate::compiler::generator::location::DynamicLocationUpdateGenerator;

use self::{call::{CallGenerator, CallType}, compare::CompareGenerator, function::FunctionGenerator, init_dict::InitDictGenerator, init_list::InitListGenerator, jump::JumpGenerator, load::LoadGenerator, location::{CurrentLocationUpdateGenerator, OpcodeLocation, SubtractionGenerator}, location_group::OpcodeLocationGroup, opcode_item::OpcodeItem, store::{StoreGenerator, StoreType}};

use super::{VmOpCode, function::FunctionReference};

pub mod opcode_item;
pub mod location;
pub mod function;
pub mod jump;
pub mod store;
pub mod compare;
pub mod load;
pub mod call;
pub mod location_group;
pub mod init_list;
pub mod init_dict;

pub trait OpcodeGeneratorTrait {
    fn generate(&self, opcodes: &mut Vec<u8>);
}

pub struct LoopItem {
    pub loop_breaks: OpcodeLocationGroup,
    pub loop_continues:  OpcodeLocationGroup,
}

impl LoopItem {
    pub fn new() -> Self {
        LoopItem {
            loop_breaks: OpcodeLocationGroup::new(),
            loop_continues: OpcodeLocationGroup::new()
        }
    }
}

pub struct OpcodeGenerator {
    generators: RefCell<Vec<Rc<dyn OpcodeGeneratorTrait>>>,
    loop_groups: RefCell<VecDeque<LoopItem>>
}

impl OpcodeGenerator {
    pub fn new() -> Self {
        OpcodeGenerator {
            generators: RefCell::new(Vec::new()),
            loop_groups: RefCell::new(VecDeque::new())
        }
    }

    pub fn add_opcode<T: Borrow<VmOpCode>>(&self, opcode: T) {
        self.generators.borrow_mut().push(Rc::new(OpcodeItem { opcode: opcode.borrow().clone() }));
    }

    pub fn create_load(&self, location: u8) -> Rc<LoadGenerator> {
        let generator = Rc::new(LoadGenerator { location: location });
        self.generators.borrow_mut().push(generator.clone());
        generator
    }

    pub fn add_break_location(&self, location: Rc<OpcodeLocation>) {
        match self.loop_groups.borrow().back() {
            Some(group) => group.loop_breaks.add(location.clone()),
            None => assert_eq!(false, false, "Döngü grubu bulunamadı")
        };
    }

    pub fn add_continue_location(&self, location: Rc<OpcodeLocation>) {
        match self.loop_groups.borrow().back() {
            Some(group) => group.loop_continues.add(location.clone()),
            None => assert_eq!(false, false, "Döngü grubu bulunamadı")
        };
    }

    pub fn set_continues_locations(&self, location: Rc<OpcodeLocation>) {
        match self.loop_groups.borrow().back() {
            Some(group) => {
                for target_location in group.loop_continues.locations.borrow().iter() {
                    self.generators.borrow_mut().push(Rc::new(DynamicLocationUpdateGenerator {
                        target: target_location.clone(),
                        source: location.clone()
                    }));
                }
            },
            None => assert_eq!(false, false, "Döngü grubu bulunamadı")
        };
    }

    pub fn set_breaks_locations(&self, location: Rc<OpcodeLocation>) {
        match self.loop_groups.borrow().back() {
            Some(group) => {
                for target_location in group.loop_breaks.locations.borrow().iter() {
                    self.generators.borrow_mut().push(Rc::new(DynamicLocationUpdateGenerator {
                        target: target_location.clone(),
                        source: location.clone()
                    }));
                }
            },
            None => assert_eq!(false, false, "Döngü grubu bulunamadı")
        };
    }

    pub fn loop_started(&self) {
        self.loop_groups.borrow_mut().push_back(LoopItem::new());
    }

    pub fn loop_finished(&self) {
        self.loop_groups.borrow_mut().pop_back();
    }

    /// Create empty location point. It is used for jump and compare location positions.
    pub fn create_location(&self) -> Rc<OpcodeLocation> {
        Rc::new(OpcodeLocation::empty())
    }

    /// Set location information with opcode length
    /// # Arguments
    /// * `location` - OpcodeLocation reference to be updated dynamically at generation time
    pub fn set_current_location(&self, location: Rc<OpcodeLocation>) {
        let generator = Rc::new(CurrentLocationUpdateGenerator { location: location.clone() });
        self.generators.borrow_mut().push(generator.clone());
    }

    pub fn subtract_location(&self, target: Rc<OpcodeLocation>, left_hand: Rc<OpcodeLocation>, right_hand: Rc<OpcodeLocation>) {
        let generator = Rc::new(SubtractionGenerator { target, left_hand, right_hand });
        self.generators.borrow_mut().push(generator.clone());
    }

    pub fn build_current_location(&self) -> Rc<OpcodeLocation> {
        let location = self.create_location();
        let generator = Rc::new(CurrentLocationUpdateGenerator { location: location.clone() });
        self.generators.borrow_mut().push(generator.clone());
        location
    }

    pub fn create_location_with_data(&self, location: usize) -> Rc<OpcodeLocation> {
        Rc::new(OpcodeLocation::new(location))
    }

    /// Create a new location information and that location information should be populated at generation time with current location
    pub fn current_location(&self) -> Rc<OpcodeLocation> {
        let location = Rc::new(OpcodeLocation::empty());
        self.set_current_location(location.clone());
        location
    }

    pub fn create_jump(&self, location: Rc<OpcodeLocation>) -> Rc<JumpGenerator> {
        let generator = Rc::new(JumpGenerator { location: location.clone() });
        self.generators.borrow_mut().push(generator.clone());
        generator
    }

    pub fn create_compare(&self, location: Rc<OpcodeLocation>) -> Rc<CompareGenerator> {
        let generator = Rc::new(CompareGenerator { location: location.clone() });
        self.generators.borrow_mut().push(generator.clone());
        generator
    }

    pub fn create_store(&self, destination: u8) -> Rc<StoreGenerator> {
        let generator = Rc::new(StoreGenerator { 
            store_type: StoreType::Store(destination)
         });
        self.generators.borrow_mut().push(generator.clone());
        generator
    }

    pub fn create_copy_to_store(&self, destination: u8) -> Rc<StoreGenerator> {
        let generator = Rc::new(StoreGenerator { 
            store_type: StoreType::CopyToStore(destination)
         });
        self.generators.borrow_mut().push(generator.clone());
        generator
    }

    pub fn create_fast_store(&self, source: u8, destination: u8) -> Rc<StoreGenerator> {
        let generator = Rc::new(StoreGenerator { 
            store_type: StoreType::FastStore {
                destination: destination,
                source: source
            }
         });
        self.generators.borrow_mut().push(generator.clone());
        generator
    }

    pub fn create_call(&self, function_location: u8, argument_size: u8, assign_to_temp: bool) -> Rc<CallGenerator> {
        let generator = Rc::new(CallGenerator { 
                call_type: CallType::Call { location: function_location },
                argument_size,
                assign_to_temp
             });
        self.generators.borrow_mut().push(generator.clone());
        generator
    }

    pub fn create_call_stack(&self, argument_size: u8, assign_to_temp: bool) -> Rc<CallGenerator> {
        let generator = Rc::new(CallGenerator { 
                call_type: CallType::CallStack,
                argument_size,
                assign_to_temp
             });
        self.generators.borrow_mut().push(generator.clone());
        generator
    }

    pub fn create_function_definition(&self, function: Rc<FunctionReference>) -> Rc<FunctionGenerator> {
        let generator = Rc::new(FunctionGenerator { function: function.clone() });
        self.generators.borrow_mut().push(generator.clone());
        generator
    }

    pub fn create_init_list(&self, argument_size: usize) -> Rc<InitListGenerator> {
        let generator = Rc::new(InitListGenerator { argument_size });
        self.generators.borrow_mut().push(generator.clone());
        generator
    }

    pub fn create_init_dict(&self, argument_size: usize) -> Rc<InitDictGenerator> {
        let generator = Rc::new(InitDictGenerator { argument_size });
        self.generators.borrow_mut().push(generator.clone());
        generator
    }
}

impl OpcodeGeneratorTrait for OpcodeGenerator {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        for generator in self.generators.borrow().iter() {
            generator.generate(opcodes);
        }
    }
}



#[cfg(test)]
mod tests {
    use super::OpcodeGenerator;


    #[test]
    fn test_1() {
        let mut opcodes = Vec::new();
        let generator = OpcodeGenerator::new();
        let location = generator.create_location();
        assert_eq!(location.get(), 0);

        let jump = generator.create_jump(location.clone());
        assert_eq!(jump.location.get(), 0);

        location.set(100, &mut opcodes);
        assert_eq!(jump.location.get(), 100);
        assert_eq!(location.get(), 100);
    }

    #[test]
    fn test_2() {
        let mut opcodes = Vec::new();
        let generator = OpcodeGenerator::new();
        let location = generator.create_location();

        let jump_1 = generator.create_jump(location.clone());
        let jump_2 = generator.create_jump(location.clone());
        location.set(100, &mut opcodes);
        assert_eq!(location.get(), 100);
        assert_eq!(jump_1.location.get(), 100);
        assert_eq!(jump_2.location.get(), 100);
    }
}