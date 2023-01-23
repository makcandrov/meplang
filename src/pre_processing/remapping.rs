use std::collections::HashMap;

use super::pre_processing::{Block, BlockItem, BlockPosition, Contract, Push, PushInner};

pub fn remap_contracts(
    mut contracts: HashMap<usize, Contract>,
    remapping: &Vec<usize>,
) -> Vec<Contract> {
    let remapping_map = vec_to_remapping_map(remapping);

    for (_, contract) in &mut contracts {
        for block in &mut contract.blocks {
            for item in &mut block.items {
                match item {
                    BlockItem::Contract(ref mut contract_index) => {
                        *contract_index = *remapping_map.get(contract_index).unwrap();
                    },
                    _ => (),
                }
            }
        }
    }

    remap(contracts, remapping)
}

pub fn remap_blocks(
    mut blocks: HashMap<usize, Block>,
    remapping: &Vec<usize>,
    new_positions: &HashMap<usize, BlockPosition>,
) -> Vec<Block> {
    let remapping_map = vec_to_remapping_map(remapping);

    for (_, block) in &mut blocks {
        for item in &mut block.items {
            match item {
                BlockItem::Push(Push {
                    attributes: _,
                    inner: PushInner::BlockPc { index, line },
                }) => {
                    let position = new_positions.get(index).unwrap();
                    *line = position.start;
                    *index = *remapping_map.get(&position.root_index).unwrap();
                },
                BlockItem::Push(Push {
                    attributes: _,
                    inner: PushInner::BlockSize { index, start, end },
                }) => {
                    let position = new_positions.get(index).unwrap();
                    *start = position.start;
                    *end = position.end;
                    *index = *remapping_map.get(&position.root_index).unwrap();
                },
                _ => (),
            }
        }
    }

    remap(blocks, remapping)
}

fn remap<T>(mut map: HashMap<usize, T>, remapping: &Vec<usize>) -> Vec<T> {
    let res = remapping.iter().map(|x| map.remove(x).unwrap()).collect();
    assert!(map.len() == 0);
    res
}

fn vec_to_remapping_map(remapping: &Vec<usize>) -> HashMap<usize, usize> {
    remapping.iter().enumerate().map(|(x, y)| (*y, x)).collect()
}
