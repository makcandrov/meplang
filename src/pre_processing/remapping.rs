use std::collections::HashMap;

use super::pre_processing::{Block, BlockItemInner, BlockPosition, Contract, Push, PushInner};

pub fn remap_contracts(
    mut contracts: HashMap<usize, Contract>,
    remapping: &[usize],
) -> Vec<Contract> {
    let remapping_map = vec_to_remapping_map(remapping);

    for contract in contracts.values_mut() {
        for block in &mut contract.blocks {
            for item in &mut block.items {
                if let BlockItemInner::Contract(contract_index) = &mut item.inner {
                    *contract_index = *remapping_map.get(contract_index).unwrap();
                }
            }
        }
    }

    remap(contracts, remapping)
}

pub fn remap_blocks(
    mut blocks: HashMap<usize, Block>,
    remapping: &[usize],
    new_positions: &HashMap<usize, BlockPosition>,
) -> Vec<Block> {
    let remapping_map = vec_to_remapping_map(remapping);

    for block in blocks.values_mut() {
        for item in &mut block.items {
            match &mut item.inner {
                BlockItemInner::Push(Push {
                    attributes: _,
                    inner: PushInner::BlockPc { index, line },
                }) => {
                    let position = new_positions.get(index).unwrap();
                    *line = position.start;
                    *index = *remapping_map.get(&position.root_index).unwrap();
                }
                BlockItemInner::Push(Push {
                    attributes: _,
                    inner: PushInner::BlockSize { index, start, end },
                }) => {
                    let position = new_positions.get(index).unwrap();
                    *start = position.start;
                    *end = position.end;
                    *index = *remapping_map.get(&position.root_index).unwrap();
                }
                _ => (),
            }
        }
    }
    remap(blocks, remapping)
}

fn remap<T: std::fmt::Debug>(mut map: HashMap<usize, T>, remapping: &[usize]) -> Vec<T> {
    let res = remapping.iter().map(|x| map.remove(x).unwrap()).collect();
    assert!(map.is_empty());
    res
}

fn vec_to_remapping_map(remapping: &[usize]) -> HashMap<usize, usize> {
    remapping.iter().enumerate().map(|(x, y)| (*y, x)).collect()
}
