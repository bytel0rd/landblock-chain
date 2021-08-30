// Oyegoke Abiodun

mod blockchain;

fn main() {

    let mut chain = blockchain::block_chain::BlockChain::new();

    let mut block = blockchain::block::Block::new();

    block.set_data(10);

    chain.append_block(block);

    let mut block2 = blockchain::block::Block::new();
    block2.set_data(20);
    chain.append_block(block2);

    let mut block3 = blockchain::block::Block::new();
    block3.set_data(50);
    
    chain.append_block(block3);

    println!("{:#?}", chain);
    println!("{:#?}", chain.validate_chain());
}
