use std::collections::HashMap;

use bytes::{Bytes, BytesMut, BufMut};

use crate::pre_processing::pre_processing::{Block, Contract, BlockItem, PushInner};

/// dumb compiler, will be improved later ;)
pub fn compile_contracts(
    contracts: Vec<Contract>,
) -> Bytes {
    let mut bytecodes = HashMap::<usize, Bytes>::new();

    for contract_index in (0..contracts.len()).rev() {
        bytecodes.insert(
            contract_index,
            compile_contract(&contracts[contract_index].blocks, &bytecodes),
        );
    }

    bytecodes.remove(&0).unwrap()
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
    blocks: &Vec<Block>,
    bytecodes: &HashMap<usize, Bytes>,
) -> Bytes {
    let mut res = BytesMut::new();

    let mut block_positions = HashMap::<usize, Vec<usize>>::new();
    let mut holes = Vec::<Hole>::new();

    for block_index in 0..blocks.len() {
        let block = &blocks[block_index];
        let mut pcs = Vec::with_capacity(block.items.len());
        for item in &block.items {
            pcs.push(res.len());
            match item {
                BlockItem::Bytes(bytes) => res.extend_from_slice(bytes),
                BlockItem::Contract(contract_index) => res.extend_from_slice(bytecodes.get(contract_index).unwrap()),
                BlockItem::Push(push) => {
                    let assumes: HashMap<Bytes, u8> = if push.attributes.optimization {
                        push.attributes.assumes.iter().map(|(x, y)| (y.clone(), *x)).collect()
                    } else {
                        HashMap::new()
                    };
                    match &push.inner {
                        PushInner::Constant(cst) => {
                            if let Some(op) = assumes.get(cst) {
                                res.put_u8(*op);
                            } else {
                                res.put_u8(0x5f + (cst.len() as u8));
                                res.extend_from_slice(cst);
                            }
                        },
                        PushInner::BlockSize { index, start, end } => {
                            res.put_u8(0x61);
                            holes.push(Hole::Size(SizeHole {
                                block_index: *index,
                                line_start: *start,
                                line_end: *end,
                                hole_pos: res.len(),
                            }));
                            res.put_u8(0x00);
                            res.put_u8(0x00);

                        },
                        PushInner::BlockPc { index, line } => {
                            res.put_u8(0x61);
                            holes.push(Hole::Pc(PcHole {
                                block_index: *index,
                                line: *line,
                                hole_pos: res.len(),
                            }));
                            res.put_u8(0x00);
                            res.put_u8(0x00);

                        },
                    }
                },
            }
        }
        pcs.push(res.len());
        block_positions.insert(block_index, pcs);
    }

    for hole in holes {
        match hole {
            Hole::Pc(pc_hole) => {
                let positions = block_positions.get(&pc_hole.block_index).unwrap();
                let pc = positions[pc_hole.line];
                res[pc_hole.hole_pos + 1] = (pc % 256) as u8;
                res[pc_hole.hole_pos] = (pc / 256) as u8;
            },
            Hole::Size(size_hole) => {
                let positions = block_positions.get(&size_hole.block_index).unwrap();
                let size = positions[size_hole.line_end] - positions[size_hole.line_start];
                res[size_hole.hole_pos + 1] = (size % 256) as u8;
                res[size_hole.hole_pos] = (size / 256) as u8;
            },
        }
    }
    res.into()
}