use std::collections::HashMap;

use bytes::{BufMut, Bytes, BytesMut};

use super::artifacts::ContractArtifacts;
use super::fillers::{fill_with_pattern, fill_with_random};
use super::settings::{CompilerSettings, FillingPatern};
use crate::compile::artifacts::Artifacts;
use crate::pre_processing::opcode::{PUSH0, PUSH1, PUSH2, PUSH32, push_length};
use crate::pre_processing::pre_processing::{Block, BlockItemInner, Contract, PushInner};
use crate::types::bytes32::Bytes32;

pub fn compile_contracts(contracts: Vec<Contract>, settings: CompilerSettings) -> Artifacts {
    let mut artifacts = Artifacts {
        main: contracts[0].name.clone(),
        ..Default::default()
    };

    let mut bytecodes = HashMap::<usize, Bytes>::new();

    for contract_index in (0..contracts.len()).rev() {
        let contract_artifacts =
            compile_contract(&contracts[contract_index].blocks, &bytecodes, &settings);
        let contract_name = &contracts[contract_index].name;

        artifacts
            .contracts
            .insert(contract_name.clone(), contract_artifacts);

        bytecodes.insert(
            contract_index,
            artifacts
                .contracts
                .get(contract_name)
                .unwrap()
                .bytecode
                .clone(),
        );
    }

    artifacts
}

#[derive(Clone, Debug)]
struct PcHole {
    pub block_index: usize,
    pub line: usize,
    pub hole_pos: usize,
}

#[derive(Clone, Debug)]
struct SizeHole {
    pub block_index: usize,
    pub line_start: usize,
    pub line_end: usize,
    pub hole_pos: usize,
}

#[derive(Clone, Debug)]
enum Hole {
    Pc(PcHole),
    Size(SizeHole),
}

fn compile_contract(
    blocks: &[Block],
    bytecodes: &HashMap<usize, Bytes>,
    settings: &CompilerSettings,
) -> ContractArtifacts {
    let mut contract_artifacts = ContractArtifacts::default();
    let mut res = BytesMut::new();

    let mut block_positions = HashMap::<usize, Vec<usize>>::new();
    let mut holes = Vec::<Hole>::new();
    let blocks_len = blocks.len();
    for (block_index, block) in blocks.iter().enumerate() {
        contract_artifacts.set_pc(&block.name, res.len());

        let mut pcs = Vec::with_capacity(block.items.len());
        for item in &block.items {
            for start_name in &item.start_names {
                contract_artifacts.set_pc(start_name, res.len());
            }

            pcs.push(res.len());
            match &item.inner {
                BlockItemInner::Bytes(bytes) => res.extend_from_slice(bytes),
                BlockItemInner::Contract(contract_index) => {
                    res.extend_from_slice(bytecodes.get(contract_index).unwrap())
                }
                BlockItemInner::Push(push) => {
                    let assumes: HashMap<Bytes32, u8> = if push.attributes.optimization {
                        push.attributes
                            .assumes
                            .iter()
                            .map(|(x, y)| (y.clone(), *x))
                            .collect()
                    } else {
                        HashMap::new()
                    };
                    match &push.inner {
                        PushInner::Constant(cst) => {
                            if settings.push0 && cst.is_zero() {
                                res.put_u8(PUSH0);
                            } else if let Some(op) = assumes.get(cst) {
                                res.put_u8(*op);
                            } else if !settings.push0 && cst.is_zero() {
                                res.put_u8(PUSH1);
                                res.put_u8(0x00);
                            } else {
                                let content = cst.right_content();
                                let push = PUSH0 + (content.len() as u8);
                                assert!(PUSH0 < push && push <= PUSH32);
                                res.put_u8(push);
                                res.extend_from_slice(content);
                            }
                        }
                        PushInner::BlockSize { index, start, end } => {
                            res.put_u8(PUSH2);
                            holes.push(Hole::Size(SizeHole {
                                block_index: *index,
                                line_start: *start,
                                line_end: *end,
                                hole_pos: res.len(),
                            }));
                            res.put_u8(0x00);
                            res.put_u8(0x00);
                        }
                        PushInner::BlockPc { index, line } => {
                            res.put_u8(PUSH2);
                            holes.push(Hole::Pc(PcHole {
                                block_index: *index,
                                line: *line,
                                hole_pos: res.len(),
                            }));
                            res.put_u8(0x00);
                            res.put_u8(0x00);
                        }
                    }
                }
            }

            for start_name in &item.start_names {
                contract_artifacts.set_size(start_name, res.len());
            }
        }
        pcs.push(res.len());

        if block_index != blocks_len - 1 {
            let mut block_bytes_iter = res[pcs[0]..*pcs.last().unwrap()].iter();

            let mut bytes_to_add = 0;
            while let Some(op) = block_bytes_iter.next() {
                if let Some(mut remaining_push) = push_length(*op) {
                    while remaining_push > 0 {
                        if block_bytes_iter.next().is_some() {
                            remaining_push -= 1;
                        } else {
                            bytes_to_add = remaining_push;
                            break;
                        }
                    }
                }
            }

            match &settings.filling_pattern {
                FillingPatern::Random => fill_with_random(&mut res, bytes_to_add),
                FillingPatern::Repeat(pattern) => {
                    fill_with_pattern(&mut res, pattern, bytes_to_add)
                }
            }
        }

        contract_artifacts.set_size(&block.name, res.len());
        block_positions.insert(block_index, pcs);
    }

    for hole in holes {
        match hole {
            Hole::Pc(pc_hole) => {
                let positions = block_positions.get(&pc_hole.block_index).unwrap();
                let pc = positions[pc_hole.line];
                if pc > 0xffff {
                    panic!("file too long");
                }
                res[pc_hole.hole_pos + 1] = (pc % 256) as u8;
                res[pc_hole.hole_pos] = (pc / 256) as u8;
            }
            Hole::Size(size_hole) => {
                let positions = block_positions.get(&size_hole.block_index).unwrap();
                let size = positions[size_hole.line_end] - positions[size_hole.line_start];
                if size > 0xffff {
                    panic!("file too long");
                }
                res[size_hole.hole_pos + 1] = (size % 256) as u8;
                res[size_hole.hole_pos] = (size / 256) as u8;
            }
        }
    }

    contract_artifacts.bytecode = res.into();
    contract_artifacts
}
