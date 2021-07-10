use std::{borrow::Borrow, cell::Cell, rc::Rc};

use super::{KaramelCompilerContext, VmOpCode};

trait OpcodeGeneratorTrait {
    fn generate(&self, context: &mut KaramelCompilerContext);
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
struct OpcodeItem(VmOpCode);
impl OpcodeGeneratorTrait for OpcodeItem {
    fn generate(&self, context: &mut KaramelCompilerContext) {
        context.opcodes.push(self.0 as u8);
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
struct OpcodeLocation(Cell<usize>);
impl OpcodeLocation {
    pub fn get(&self) -> usize {
        self.0.get()
    }

    pub fn set(&self, location: usize) {
        self.0.set(location);
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
struct OpcodeLocationManager { locations: Vec<Rc<OpcodeLocation>> }

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
struct JumpGenerator(Rc<OpcodeLocation>);
impl OpcodeGeneratorTrait for JumpGenerator {
    fn generate(&self, context: &mut KaramelCompilerContext) {
        context.opcodes.push(VmOpCode::Jump as u8);
        context.opcodes.push(self.get() as u8);
        context.opcodes.push((self.get() >> 8) as u8);
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
struct CompareGenerator(Rc<OpcodeLocation>);
impl OpcodeGeneratorTrait for CompareGenerator {
    fn generate(&self, context: &mut KaramelCompilerContext) {
        context.opcodes.push(VmOpCode::Compare as u8);
        context.opcodes.push(self.get() as u8);
        context.opcodes.push((self.get() >> 8) as u8);
    }
}

impl CompareGenerator {
    pub fn get(&self) -> usize {
        self.0.get()
    }

    pub fn set(&self, location: usize) {
        self.0.set(location);
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
struct LoadGenerator(usize);
impl OpcodeGeneratorTrait for LoadGenerator {
    fn generate(&self, context: &mut KaramelCompilerContext) {
        context.opcodes.push(VmOpCode::Load as u8);
        context.opcodes.push(self.0 as u8);
        context.opcodes.push((self.0 >> 8) as u8);
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
struct StoreGenerator(u8, Option<u8>);
impl OpcodeGeneratorTrait for StoreGenerator {
    fn generate(&self, context: &mut KaramelCompilerContext) {
        match self.1 {
            Some(primative_location) => {
                context.opcodes.push(VmOpCode::FastStore as u8);
                context.opcodes.push(self.0);
                context.opcodes.push(primative_location);
            },
            None => {
                context.opcodes.push(VmOpCode::Store as u8);
                context.opcodes.push(self.0 as u8);
            }
        }
    }
}

impl JumpGenerator {
    pub fn get(&self) -> usize {
        self.0.get()
    }

    pub fn set(&self, location: usize) {
        self.0.set(location);
    }
}

struct Generator {
    generators: Vec<Rc<dyn OpcodeGeneratorTrait>>
}

impl Generator {
    pub fn new() -> Self {
        Generator {
            generators: Vec::new()
        }
    }

    pub fn add_opcode<T: Borrow<VmOpCode>>(&mut self, opcode: T) {
        self.generators.push(Rc::new(OpcodeItem(opcode.borrow().clone())));
    }

    pub fn create_location(&mut self) -> Rc<OpcodeLocation> {
        Rc::new(OpcodeLocation(Cell::new(0)))
    }

    pub fn create_jump(&mut self, location: Rc<OpcodeLocation>) -> Rc<JumpGenerator> {
        let generator = Rc::new(JumpGenerator(location.clone()));
        self.generators.push(generator.clone());
        generator
    }

    pub fn create_compare(&mut self, location: Rc<OpcodeLocation>) -> Rc<CompareGenerator> {
        let generator = Rc::new(CompareGenerator(location.clone()));
        self.generators.push(generator.clone());
        generator
    }
}

impl OpcodeGeneratorTrait for Generator {
    fn generate(&self, context: &mut KaramelCompilerContext) {
        for generator in self.generators.iter() {
            generator.generate(context);
        }
    }
}



#[cfg(test)]
mod tests {
    use super::Generator;


    #[test]
    fn test_1() {
        let mut generator = Generator::new();
        let location = generator.create_location();
        assert_eq!(location.get(), 0);

        let jump = generator.create_jump(location.clone());
        assert_eq!(jump.get(), 0);

        location.set(100);
        assert_eq!(jump.get(), 100);
        assert_eq!(location.get(), 100);
    }

    #[test]
    fn test_2() {
        let mut generator = Generator::new();
        let location = generator.create_location();

        let jump_1 = generator.create_jump(location.clone());
        let jump_2 = generator.create_jump(location.clone());
        location.set(100);
        assert_eq!(location.get(), 100);
        assert_eq!(jump_1.get(), 100);
        assert_eq!(jump_2.get(), 100);
    }
}