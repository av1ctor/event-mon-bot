use crate::memory::get_upgrades_memory;
use crate::states;
use crate::lifecycle::READER_WRITER_BUFFER_SIZE;
use ic_cdk::pre_upgrade;
use ic_stable_structures::writer::{BufferedWriter, Writer};
use serde::Serialize;

#[pre_upgrade]
fn pre_upgrade(
) {
    let mut memory = get_upgrades_memory();
    let writer = BufferedWriter::new(READER_WRITER_BUFFER_SIZE, Writer::new(&mut memory, 0));
    let mut serializer = rmp_serde::Serializer::new(writer).with_struct_map();

    let main_state = states::main::take();
    main_state.serialize(&mut serializer).unwrap();

    let mon_state = states::mon::take();
    mon_state.serialize(&mut serializer).unwrap();
}