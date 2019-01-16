type Instructions = Vec<u8>;

#[derive(Clone)]
pub enum Opcode {
    OpConstant,
}

#[derive(Clone)]
pub struct Definition {
    opcode: Opcode,
    name: String,
    operand_widths: Vec<u8>,
}

pub fn lookup(op: Opcode) -> Option<Definition> {
    match op {
        Opcode::OpConstant => Some(Definition {
            opcode: Opcode::OpConstant,
            name: "OpConstant".to_string(),
            operand_widths: vec![2],
        }),
        _ => {
            eprintln!("opcode {} undifined", op as u8);
            return None
        },
    }
}

pub fn make(op: Opcode, operands: Vec<u16>) -> Vec<u8> {

    let def = match lookup(op.clone()) {
        Some(value) => value,
        None        => return Vec::new(),
    };
    
    let mut instruction = Vec::new();

    instruction.push(op.clone() as u8);

    for (i, o) in operands.iter().enumerate() {
        let width = def.operand_widths[i];

        match width {
            2 => instruction.append(&mut o.to_be_bytes().to_vec()),
            _ => return Vec::new(),
        }
    }
    
    instruction
}
