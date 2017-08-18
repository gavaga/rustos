mod temporary_page;

pub use self::entry::*;

use mem::FrameAllocator;
use mem::PAGE_SIZE;
use mem::Frame;
use self::temporary_page::TemporaryPage;

const ENTRY_COUNT: usize = 512;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

#[derive(Debug, Clone, Copy)]
pub struct Page {
    number: usize,
}

impl Page {
    fn start_address(&self) -> usize {
        self.number * PAGE_SIZE
    }

    fn p4_index(&self) -> usize {
        (self.number >> 27) & 0o777
    }

    fn p3_index(&self) -> usize {
        (self.number >> 18) & 0o777
    }

    fn p2_index(&self) -> usize {
        (self.number >> 9) & 0o777
    }

    fn p1_index(&self) -> usize {
        (self.number >> 0) & 0o777
    }
}

pub struct InactivePageTable {
    p4_frame: Frame,
}

impl InactivePageTable {
    pub fn new(frame: Frame,
              active_table: &mut ActivePageTable,
              temporary_page: &mut TemporaryPage)
        -> InactivePageTable
    {
        {
            let table = temporary_page.map_table_frame(frame.clone(),
                active_table);

            table.zero();
            table[511].set(frame.clone(), PRESENT | WRITABLE);
        }
        temporary_page.unmap(active_table);

        InactivePageTable { p4_frame: frame }
    }
}

pub fn containing_address(address: VirtualAddress) -> Page {
    assert!(address < 0x0000_8000_0000_0000 || 
            address >= 0xffff_8000_0000_0000,
            "invalid address: 0x{:x}", address);
    Page { number: address / PAGE_SIZE }
}
