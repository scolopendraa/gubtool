use {
    serde::{Deserialize, Serialize},
    std::collections::{HashMap, VecDeque},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct AsmFolder {
    pub functions: HashMap<String, AsmFunction>,
}

impl AsmFolder {
    pub fn new(functions: Vec<AsmFunction>) -> Self {
        let map: HashMap<String, AsmFunction> = functions
            .into_iter()
            .map(|fun| (fun.name.clone(), fun))
            .collect();
        Self {
            functions: map,
        }
    }

    pub fn get_function(&self, name: &'static str) -> AsmFunction {
        self.functions.get(name).unwrap().clone()
    }

    pub fn print_function_sizes(&self) {
        self.functions
            .iter()
            .for_each(|(key, fun)| println!("{}, {:#X}", key, fun.bytes.len()));
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AsmFunction {
    name:        String,
    bytes:       Vec<u8>,
    relocations: VecDeque<Relocation>,
}

impl AsmFunction {
    pub fn new(name: String, bytes: Vec<u8>, relocations: VecDeque<Relocation>) -> Self {
        Self {
            name,
            bytes,
            relocations,
        }
    }

    pub fn take_bytes(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.bytes)
    }

    pub fn print_relocs(&self) {
        self.relocations
            .iter()
            .for_each(|r| println!("{:#X}, {}", r.offset, r.symbol));
    }

    #[track_caller]
    pub fn reloc(&mut self, name: &'static str) -> u64 {
        let reloc = self.relocations.pop_front().unwrap();

        if reloc.symbol == name {
            reloc.offset
        } else {
            panic!("symbol mismatch")
        }
    }

    #[track_caller]
    pub fn reloc_find(&mut self, name: &'static str) -> u64 {
        let pos = self
            .relocations
            .iter()
            .position(|s| s.symbol == name)
            .unwrap();
        let popped = self.relocations.remove(pos).unwrap();
        popped.offset
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Relocation {
    symbol: String,
    offset: u64,
}

impl Relocation {
    pub fn new(symbol: String, offset: u64) -> Self {
        Self {
            symbol,
            offset,
        }
    }
}
