use crate::bytecode::Opcode;

#[derive(Clone, Debug)]
pub struct CodePtr {
    chunk_id: usize,
    code_ptr: usize,
}

impl CodePtr {
    pub fn new(chunk_id: usize, code_ptr: usize) -> Self {
        Self { chunk_id, code_ptr }
    }
    pub fn get_chunk_id(&self) -> usize {
        self.chunk_id
    }

    pub fn get_code_ptr(&self) -> usize {
        self.code_ptr
    }

    pub fn set_code_ptr(&mut self, ptr: usize) {
        self.code_ptr = ptr;
    }
}

#[derive(Debug)]
pub struct CodeVector {
    chunks: Vec<Vec<Opcode>>,
}

impl CodeVector {
    pub fn new(chunks: Vec<Vec<Opcode>>) -> Self {
        Self { chunks }
    }

    pub fn get_chunk(&self, chunk_id: usize) -> Option<&Vec<Opcode>> {
        self.chunks.get(chunk_id)
    }

    pub fn has_chunk(&self, chunk_id: usize) -> bool {
        self.chunks.len() > chunk_id
    }

    pub fn is_empty(&self) -> bool {
        self.chunks.len() == 0
    }

    pub fn get_current_opcode(&self, chunk_id: usize, code_ptr: usize) -> Option<&Opcode> {
        self.get_chunk(chunk_id)
            .and_then(|chunk| chunk.get(code_ptr))
    }
}
