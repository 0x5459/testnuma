use anyhow::Result;
use mapr::{MmapMut, MmapOptions};

pub fn allocate_layer(sector_size: usize) -> Result<MmapMut> {
//    if {
//        unimplemented!();
//    }
    match MmapOptions::new()
        .len(sector_size)
        .private()
        .clone()
        .lock()
        .map_anon()
        .and_then(|mut layer| {
            layer.mlock()?;
            Ok(layer)
        }) {
        Ok(layer) => Ok(layer),
        Err(err) => {
            println!("failed to lock map {:?}, falling back", err);
            // fallback to not locked if permissions are not available
            let layer = MmapOptions::new().len(sector_size).private().map_anon()?;
            Ok(layer)
        }
    }
}
