use std::{borrow::Borrow, cell::{Cell, RefCell}, cmp, collections::VecDeque, rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use crate::{compiler::generator::location::DynamicLocationUpdateGenerator, constants::{DUMP_INDEX_WIDTH, DUMP_OPCODE_COLUMN_1, DUMP_OPCODE_COLUMN_2, DUMP_OPCODE_COLUMN_3, DUMP_OPCODE_TITLE, DUMP_OPCODE_WIDTH}};

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
    fn dump<'a>(&self, builder: &'a DumpBuilder, index: Rc<AtomicUsize>, opcodes: &Vec<u8>);
}

pub fn dump_single_opcode<'a, T: Borrow<String>>(builder: &'a DumpBuilder, index: usize, opcode: T, buffer: &mut String) {
    dump_default(builder, index, opcode, buffer, "".to_string(), "".to_string(), "".to_string());
}

pub fn dump_default<'a, T: Borrow<String>, S1: Borrow<String>, S2: Borrow<String>, S3: Borrow<String>>(builder: &'a DumpBuilder, index: usize, opcode: T, buffer: &mut String, c1: S1, c2: S2, c3: S3) {
    buffer.push_str(&format!("║ {:DUMP_INDEX_WIDTH$} ║ {:DUMP_OPCODE_WIDTH$} ║ {:^DUMP_OPCODE_COLUMN_1$} ║ {:^DUMP_OPCODE_COLUMN_2$} ║ {:^DUMP_OPCODE_COLUMN_3$} ║\n", index, opcode.borrow(), c1.borrow(), c2.borrow(), c3.borrow(), DUMP_INDEX_WIDTH=builder.max_index_width.get(), DUMP_OPCODE_WIDTH=builder.max_opcode_width.get(), DUMP_OPCODE_COLUMN_1=builder.max_column1_width.get(), DUMP_OPCODE_COLUMN_2=builder.max_column2_width.get(), DUMP_OPCODE_COLUMN_3=builder.max_column3_width.get())[..]);
}

pub fn opcode_to_location(index: Rc<AtomicUsize>, opcodes: &Vec<u8>) -> usize {
    let location_2 = opcodes[index.fetch_add(1, Ordering::SeqCst)];
    let location_1 = opcodes[index.fetch_add(1, Ordering::SeqCst)];

    ((location_1 as u16 * 256) + location_2 as u16) as usize
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

pub enum DumpItemType {
    Opcode(VmOpCode),
    Text(String)
}

pub struct DumpItem {
    pub index: usize,
    pub opcode: DumpItemType,
    pub column1: String,
    pub column2: String,
    pub column3: String
}

pub struct DumpBuilder {
    pub dumps: RefCell<Vec<DumpItem>>,
    pub max_index_width: Cell<usize>,
    pub max_opcode_width: Cell<usize>,
    pub max_column1_width: Cell<usize>,
    pub max_column2_width: Cell<usize>,
    pub max_column3_width: Cell<usize>
}

impl DumpBuilder {
    pub fn new() -> Self {
        DumpBuilder {
            dumps: RefCell::new(Vec::new()),
            max_index_width: Cell::new(DUMP_INDEX_WIDTH),
            max_opcode_width: Cell::new(DUMP_OPCODE_WIDTH),
            max_column1_width: Cell::new(DUMP_OPCODE_COLUMN_1),
            max_column2_width: Cell::new(DUMP_OPCODE_COLUMN_2),
            max_column3_width: Cell::new(DUMP_OPCODE_COLUMN_3)
        }
    }

    pub fn add(&self, index: usize, opcode: VmOpCode, column1: String, column2: String, column3: String) {
        self.max_column1_width.set(cmp::max(self.max_column1_width.get(), column1.len()));
        self.max_column2_width.set(cmp::max(self.max_column2_width.get(), column2.len()));
        self.max_column3_width.set(cmp::max(self.max_column3_width.get(), column3.len()));

        let item = DumpItem {
            index: index,
            opcode: DumpItemType::Opcode(opcode),
            column1: column1,
            column2: column2,
            column3: column3
        };
        self.dumps.borrow_mut().push(item);
    }

    pub fn add_with_text(&self, index: usize, opcode: String, column1: String, column2: String, column3: String) {
        self.max_column1_width.set(cmp::max(self.max_column1_width.get(), column1.len()));
        self.max_column2_width.set(cmp::max(self.max_column2_width.get(), column2.len()));
        self.max_column3_width.set(cmp::max(self.max_column3_width.get(), column3.len()));
        self.max_opcode_width.set(cmp::max(self.max_opcode_width.get(), opcode.len()));

        let item = DumpItem {
            index: index,
            opcode: DumpItemType::Text(opcode),
            column1: column1,
            column2: column2,
            column3: column3
        };
        self.dumps.borrow_mut().push(item);
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

impl OpcodeGenerator {
    pub fn generate(&self, opcodes: &mut Vec<u8>) {
        for generator in self.generators.borrow().iter() {
            generator.generate(opcodes);
        }
    }

    pub fn dump(&self, opcodes: &Vec<u8>) -> String {
        let builder = DumpBuilder::new();
        let indexer = Rc::new(AtomicUsize::new(0));

        let mut buffer = String::with_capacity(1024);

        for generator in self.generators.borrow().iter() {
            generator.dump(&builder, indexer.clone(), opcodes);
        }

        match &self.generators.borrow().len() {
            0 => (),
            _ => builder.max_index_width.set(format!("{}", builder.dumps.borrow().iter().last().unwrap().index).len())
        };
        

        let total_width = builder.max_index_width.get()+builder.max_opcode_width.get()+builder.max_column1_width.get()+builder.max_column2_width.get()+builder.max_column3_width.get()+12;
        let left_width = (total_width - DUMP_OPCODE_TITLE.len()) / 2;
        let right_width = (total_width - DUMP_OPCODE_TITLE.len()) - left_width;

        buffer.push_str(&format!("╔═{:═<WIDTH$}═╗\n", "═", WIDTH=total_width)[..]);
        buffer.push_str(&format!("║ {:<LEFT_WIDTH$}{}{:<RIGHT_WIDTH$} ║\n", " ", DUMP_OPCODE_TITLE, " ", LEFT_WIDTH=left_width, RIGHT_WIDTH=right_width)[..]);
        buffer.push_str(&format!("╠═{:═<DUMP_INDEX_WIDTH$}═╦═{:═<DUMP_OPCODE_WIDTH$}═╦═{:═<DUMP_OPCODE_COLUMN_1$}═╦═{:═<DUMP_OPCODE_COLUMN_2$}═╦═{:═<DUMP_OPCODE_COLUMN_3$}═╣\n", "═", "═", "═", "═", "═", DUMP_INDEX_WIDTH=builder.max_index_width.get(), DUMP_OPCODE_WIDTH=builder.max_opcode_width.get(), DUMP_OPCODE_COLUMN_1=builder.max_column1_width.get(), DUMP_OPCODE_COLUMN_2=builder.max_column2_width.get(), DUMP_OPCODE_COLUMN_3=builder.max_column3_width.get())[..]);

        for item in builder.dumps.borrow().iter() {
            let item_type = match &item.opcode {
                DumpItemType::Opcode(opcode) => opcode.to_string(),
                DumpItemType::Text(text) => text.to_string()
            };

            dump_default(&builder, item.index, item_type, &mut buffer, &item.column1, &item.column2, &item.column3);
        }

        buffer.push_str(&format!("╚═{:═<DUMP_INDEX_WIDTH$}═╩═{:═<DUMP_OPCODE_WIDTH$}═╩═{:═<DUMP_OPCODE_COLUMN_1$}═╩═{:═<DUMP_OPCODE_COLUMN_2$}═╩═{:═<DUMP_OPCODE_COLUMN_3$}═╝", "═", "═", "═", "═", "═", DUMP_INDEX_WIDTH=builder.max_index_width.get(), DUMP_OPCODE_WIDTH=builder.max_opcode_width.get(), DUMP_OPCODE_COLUMN_1=builder.max_column1_width.get(), DUMP_OPCODE_COLUMN_2=builder.max_column2_width.get(), DUMP_OPCODE_COLUMN_3=builder.max_column3_width.get())[..]);
        buffer
    }
}



#[cfg(test)]
mod tests {
    use crate::{buildin::DummyModule, compiler::ast::KaramelAstType};

    use super::*;


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

    #[test]
    fn test_dump_1() {
        let mut expected = String::with_capacity(1024);
        let mut opcodes = Vec::new();
        let generator = OpcodeGenerator::new();

        generator.generate(&mut opcodes);
        let generated = generator.dump(&opcodes);

        expected.push_str(r#"╔═════════════════════════════════════════════════╗
║                   OPCODE DUMP                   ║
╠═══════╦═════════════════╦═══════╦═══════╦═══════╣
╚═══════╩═════════════════╩═══════╩═══════╩═══════╝"#);

        assert_eq!(expected, generated);
    }

    #[test]
    fn test_dump_2() {
        let mut expected = String::with_capacity(1024);
        let mut opcodes = Vec::new();
        let generator = OpcodeGenerator::new();
        generator.add_opcode(VmOpCode::Equal);
        generator.add_opcode(VmOpCode::Compare);
        generator.add_opcode(VmOpCode::And);

        generator.generate(&mut opcodes);
        let generated = generator.dump(&opcodes);
        println!("{}", generated);

        expected.push_str(r#"╔═════════════════════════════════════════════╗
║                 OPCODE DUMP                 ║
╠═══╦═════════════════╦═══════╦═══════╦═══════╣
║ 0 ║ Equal           ║       ║       ║       ║
║ 1 ║ Compare         ║       ║       ║       ║
║ 2 ║ And             ║       ║       ║       ║
╚═══╩═════════════════╩═══════╩═══════╩═══════╝"#);

        assert_eq!(expected, generated);
    }

    #[test]
    fn test_dump_3() {
        let mut expected = String::with_capacity(1024);
        let mut opcodes = Vec::new();
        let generator = OpcodeGenerator::new();

        generator.add_opcode(VmOpCode::Halt);
        let location = generator.current_location();
        generator.create_compare(location);

        generator.generate(&mut opcodes);
        let generated = generator.dump(&opcodes);
        println!("{}", generated);

        expected.push_str(r#"╔═════════════════════════════════════════════╗
║                 OPCODE DUMP                 ║
╠═══╦═════════════════╦═══════╦═══════╦═══════╣
║ 0 ║ Halt            ║       ║       ║       ║
║ 1 ║ Compare         ║   1   ║       ║       ║
╚═══╩═════════════════╩═══════╩═══════╩═══════╝"#);

        assert_eq!(expected, generated);
    }

    #[test]
    fn test_dump_4() {
        let mut expected = String::with_capacity(1024);
        let mut opcodes = Vec::new();
        let generator = OpcodeGenerator::new();

        let function = FunctionReference::opcode_function("TEST FUNCTION".to_string(), Vec::new(), Rc::new(KaramelAstType::None), Rc::new(DummyModule::new()), 0, 0, true);

        generator.add_opcode(VmOpCode::Halt);
        generator.create_function_definition(function);

        generator.generate(&mut opcodes);
        let generated = generator.dump(&opcodes);
        println!("{}", generated);

        expected.push_str(r#"╔═══════════════════════════════════════════════════════╗
║                      OPCODE DUMP                      ║
╠═══╦═══════════════════════════╦═══════╦═══════╦═══════╣
║ 0 ║ Halt                      ║       ║       ║       ║
║ 1 ║ [FUNCTION: TEST FUNCTION] ║   0   ║       ║       ║
╚═══╩═══════════════════════════╩═══════╩═══════╩═══════╝"#);

        assert_eq!(expected, generated);
    }

    #[test]
    fn test_dump_5() {
        let mut expected = String::with_capacity(1024);
        let mut opcodes = Vec::new();
        let generator = OpcodeGenerator::new();

        generator.add_opcode(VmOpCode::Halt);
        let location = generator.current_location();
        generator.create_jump(location);

        generator.generate(&mut opcodes);
        let generated = generator.dump(&opcodes);
        println!("{}", generated);

        expected.push_str(r#"╔═════════════════════════════════════════════╗
║                 OPCODE DUMP                 ║
╠═══╦═════════════════╦═══════╦═══════╦═══════╣
║ 0 ║ Halt            ║       ║       ║       ║
║ 1 ║ Jump            ║   1   ║       ║       ║
╚═══╩═════════════════╩═══════╩═══════╩═══════╝"#);

        assert_eq!(expected, generated);
    }

    #[test]
    fn test_dump_6() {
        let mut expected = String::with_capacity(1024);
        let mut opcodes = Vec::new();
        let generator = OpcodeGenerator::new();

        generator.add_opcode(VmOpCode::Halt);
        generator.create_call(20, 1, true);

        generator.generate(&mut opcodes);
        let generated = generator.dump(&opcodes);
        println!("{}", generated);

        expected.push_str(r#"╔═════════════════════════════════════════════╗
║                 OPCODE DUMP                 ║
╠═══╦═════════════════╦═══════╦═══════╦═══════╣
║ 0 ║ Halt            ║       ║       ║       ║
║ 1 ║ Call            ║  20   ║   1   ║   1   ║
╚═══╩═════════════════╩═══════╩═══════╩═══════╝"#);

        assert_eq!(expected, generated);
    }

    #[test]
    fn test_dump_7() {
        let mut expected = String::with_capacity(1024);
        let mut opcodes = Vec::new();
        let generator = OpcodeGenerator::new();

        generator.add_opcode(VmOpCode::Halt);
        generator.create_call(10, 0, false);

        generator.generate(&mut opcodes);
        let generated = generator.dump(&opcodes);
        println!("{}", generated);

        expected.push_str(r#"╔═════════════════════════════════════════════╗
║                 OPCODE DUMP                 ║
╠═══╦═════════════════╦═══════╦═══════╦═══════╣
║ 0 ║ Halt            ║       ║       ║       ║
║ 1 ║ Call            ║  10   ║   0   ║   0   ║
╚═══╩═════════════════╩═══════╩═══════╩═══════╝"#);

        assert_eq!(expected, generated);
    }

    #[test]
    fn test_dump_8() {
        let mut expected = String::with_capacity(1024);
        let mut opcodes = Vec::new();
        let generator = OpcodeGenerator::new();

        generator.add_opcode(VmOpCode::Halt);
        generator.create_call_stack(0, false);

        generator.generate(&mut opcodes);
        let generated = generator.dump(&opcodes);
        println!("{}", generated);

        expected.push_str(r#"╔═════════════════════════════════════════════╗
║                 OPCODE DUMP                 ║
╠═══╦═════════════════╦═══════╦═══════╦═══════╣
║ 0 ║ Halt            ║       ║       ║       ║
║ 1 ║ CallStack       ║   0   ║   0   ║       ║
╚═══╩═════════════════╩═══════╩═══════╩═══════╝"#);

        assert_eq!(expected, generated);
    }

    #[test]
    fn test_dump_9() {
        let mut expected = String::with_capacity(1024);
        let mut opcodes = Vec::new();
        let generator = OpcodeGenerator::new();

        generator.add_opcode(VmOpCode::Halt);
        generator.create_call_stack(5, true);

        generator.generate(&mut opcodes);
        let generated = generator.dump(&opcodes);
        println!("{}", generated);

        expected.push_str(r#"╔═════════════════════════════════════════════╗
║                 OPCODE DUMP                 ║
╠═══╦═════════════════╦═══════╦═══════╦═══════╣
║ 0 ║ Halt            ║       ║       ║       ║
║ 1 ║ CallStack       ║   5   ║   1   ║       ║
╚═══╩═════════════════╩═══════╩═══════╩═══════╝"#);

        assert_eq!(expected, generated);
    }

    #[test]
    fn test_dump_10() {
        let mut expected = String::with_capacity(1024);
        let mut opcodes = Vec::new();
        let generator = OpcodeGenerator::new();

        generator.add_opcode(VmOpCode::Halt);
        generator.create_init_dict(22);

        generator.generate(&mut opcodes);
        let generated = generator.dump(&opcodes);
        println!("{}", generated);

        expected.push_str(r#"╔═════════════════════════════════════════════╗
║                 OPCODE DUMP                 ║
╠═══╦═════════════════╦═══════╦═══════╦═══════╣
║ 0 ║ Halt            ║       ║       ║       ║
║ 1 ║ Init            ║   0   ║  22   ║       ║
╚═══╩═════════════════╩═══════╩═══════╩═══════╝"#);

        assert_eq!(expected, generated);
    }

    #[test]
    fn test_dump_11() {
        let mut expected = String::with_capacity(1024);
        let mut opcodes = Vec::new();
        let generator = OpcodeGenerator::new();

        generator.add_opcode(VmOpCode::Halt);
        generator.create_init_dict(22);

        generator.generate(&mut opcodes);
        let generated = generator.dump(&opcodes);
        println!("{}", generated);

        expected.push_str(r#"╔═════════════════════════════════════════════╗
║                 OPCODE DUMP                 ║
╠═══╦═════════════════╦═══════╦═══════╦═══════╣
║ 0 ║ Halt            ║       ║       ║       ║
║ 1 ║ Init            ║   0   ║  22   ║       ║
╚═══╩═════════════════╩═══════╩═══════╩═══════╝"#);

        assert_eq!(expected, generated);
    }

    #[test]
    fn test_dump_12() {
        let mut expected = String::with_capacity(1024);
        let mut opcodes = Vec::new();
        let generator = OpcodeGenerator::new();

        generator.add_opcode(VmOpCode::Halt);
        generator.create_load(11);

        generator.generate(&mut opcodes);
        let generated = generator.dump(&opcodes);
        println!("{}", generated);

        expected.push_str(r#"╔═════════════════════════════════════════════╗
║                 OPCODE DUMP                 ║
╠═══╦═════════════════╦═══════╦═══════╦═══════╣
║ 0 ║ Halt            ║       ║       ║       ║
║ 1 ║ Load            ║  11   ║       ║       ║
╚═══╩═════════════════╩═══════╩═══════╩═══════╝"#);

        assert_eq!(expected, generated);
    }

    #[test]
    fn test_dump_13() {
        let mut expected = String::with_capacity(1024);
        let mut opcodes = Vec::new();
        let generator = OpcodeGenerator::new();

        generator.add_opcode(VmOpCode::Halt);
        generator.create_store(11);

        generator.generate(&mut opcodes);
        let generated = generator.dump(&opcodes);
        println!("{}", generated);

        expected.push_str(r#"╔═════════════════════════════════════════════╗
║                 OPCODE DUMP                 ║
╠═══╦═════════════════╦═══════╦═══════╦═══════╣
║ 0 ║ Halt            ║       ║       ║       ║
║ 1 ║ Store           ║  11   ║       ║       ║
╚═══╩═════════════════╩═══════╩═══════╩═══════╝"#);

        assert_eq!(expected, generated);
    }

    #[test]
    fn test_dump_14() {
        let mut expected = String::with_capacity(1024);
        let mut opcodes = Vec::new();
        let generator = OpcodeGenerator::new();

        generator.add_opcode(VmOpCode::Halt);
        generator.create_fast_store(11, 22);

        generator.generate(&mut opcodes);
        let generated = generator.dump(&opcodes);
        println!("{}", generated);

        expected.push_str(r#"╔═════════════════════════════════════════════╗
║                 OPCODE DUMP                 ║
╠═══╦═════════════════╦═══════╦═══════╦═══════╣
║ 0 ║ Halt            ║       ║       ║       ║
║ 1 ║ FastStore       ║  22   ║  11   ║       ║
╚═══╩═════════════════╩═══════╩═══════╩═══════╝"#);

        assert_eq!(expected, generated);
    }

    #[test]
    fn test_dump_15() {
        let mut expected = String::with_capacity(1024);
        let mut opcodes = Vec::new();
        let generator = OpcodeGenerator::new();

        generator.add_opcode(VmOpCode::Halt);
        generator.create_copy_to_store(33);

        generator.generate(&mut opcodes);
        let generated = generator.dump(&opcodes);
        println!("{}", generated);

        expected.push_str(r#"╔═════════════════════════════════════════════╗
║                 OPCODE DUMP                 ║
╠═══╦═════════════════╦═══════╦═══════╦═══════╣
║ 0 ║ Halt            ║       ║       ║       ║
║ 1 ║ CopyToStore     ║  33   ║       ║       ║
╚═══╩═════════════════╩═══════╩═══════╩═══════╝"#);

        assert_eq!(expected, generated);
    }

    #[test]
    fn test_dump_16() {
        let mut expected = String::with_capacity(1024);
        let mut opcodes = Vec::new();
        let generator = OpcodeGenerator::new();

        generator.add_opcode(VmOpCode::Halt);
        generator.create_init_list(44);

        generator.generate(&mut opcodes);
        let generated = generator.dump(&opcodes);
        println!("{}", generated);

        expected.push_str(r#"╔═════════════════════════════════════════════╗
║                 OPCODE DUMP                 ║
╠═══╦═════════════════╦═══════╦═══════╦═══════╣
║ 0 ║ Halt            ║       ║       ║       ║
║ 1 ║ Init            ║   1   ║  44   ║       ║
╚═══╩═════════════════╩═══════╩═══════╩═══════╝"#);

        assert_eq!(expected, generated);
    }
}