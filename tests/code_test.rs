extern crate simiaVM;

use simiaVM::code::*;

#[test]
fn test_make() {
    let tests = [(Opcode::OpConstant,
                  vec![65534],
                  vec![Opcode::OpConstant as u8, 255, 254]),
    ];

    for test in tests.iter() {
        let instruction = make(test.clone().0, test.clone().1);
        if instruction.clone().len() != test.2.clone().len() {
            panic!("instruction has wrong length, want={}, got={}",
                   instruction.len(), test.2.len());
        }

        for (i, b) in test.2.iter().enumerate() {
            if instruction[i] != test.2[i] {
                panic!("wrong byte at pos {}. want={} got={}",
                       i, b, instruction[i]);
            }
        }
    }
}
