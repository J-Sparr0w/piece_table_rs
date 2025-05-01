use std::io::Write;

#[derive(Debug, Clone, Copy)]
enum BufKind {
    Original,
    Add,
}

#[derive(Debug, Clone, Copy)]
pub struct PieceTableRecord {
    buf_kind: BufKind,
    start: u32,
    len: u32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct TableMetadata {
    text_len: usize,
}

impl TableMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct PieceTable {
    original: String,
    add_buf: String,
    records: Vec<PieceTableRecord>,
    metadata: TableMetadata,
}

impl Into<String> for PieceTable {
    fn into(self) -> String {
        self.to_string()
    }
}

impl ToString for PieceTable {
    fn to_string(&self) -> String {
        let mut out = String::new();
        // eprintln!("records: {:#?}", self.records);
        for record in self.records.iter() {
            let slice = match record.buf_kind {
                BufKind::Original => {
                    &self.original[(record.start as usize)..((record.start + record.len) as usize)]
                }
                BufKind::Add => {
                    &self.add_buf[(record.start as usize)..((record.start + record.len) as usize)]
                }
            };
            out.push_str(slice);
        }
        out
    }
}

impl PieceTable {
    pub fn new() -> Self {
        Self {
            original: String::new(),
            add_buf: String::new(),
            records: Vec::new(),
            metadata: TableMetadata::new(),
        }
    }
    pub fn from_path(_: &str) -> Self {
        let text = String::from(include_str!("../samples/daffodils.txt"));
        let text_len = text.len();
        Self {
            original: text,
            add_buf: String::new(),
            records: vec![PieceTableRecord {
                buf_kind: BufKind::Original,
                start: 0,
                len: text_len as u32,
            }],
            metadata: TableMetadata::new(),
        }
    }
    pub fn show(&self) {
        let mut stdout = std::io::stdout();
        for record in self.records.iter() {
            let slice = match record.buf_kind {
                BufKind::Original => {
                    &self.original[(record.start as usize)..((record.start + record.len) as usize)]
                }
                BufKind::Add => {
                    &self.add_buf[(record.start as usize)..((record.start + record.len) as usize)]
                }
            };
            _ = write!(stdout, "{}", slice);
        }
    }

    pub fn insert_at_end(&mut self, ch: char) {
        // insert at the end implementation
        match self.records.last() {
            Some(PieceTableRecord {
                buf_kind: BufKind::Add,
                start,
                len,
            }) if (start + len) == self.add_buf.len() as u32 => {
                // the last record might point to a different part of addBuf to reduce memory usage
                // while copy pasting chunks of text.

                let last_record = self.records.last_mut().unwrap();
                self.add_buf.push(ch);
                last_record.len += 1;
                self.metadata.text_len += 1;
            }
            Some(_) | None => {
                //add char to addBuf, create new record and add it to records
                self.add_buf.push(ch);
                let new_record = PieceTableRecord {
                    buf_kind: BufKind::Add,
                    start: (self.add_buf.len() - 1) as u32,
                    len: 1,
                };
                eprintln!("I am going to add: {:#?}", new_record);
                self.push_record(new_record);
            }
        }
    }

    pub fn insert_record(&mut self, record: PieceTableRecord, record_index: usize) {
        self.records.insert(record_index, record);
        self.metadata.text_len += 1;
    }
    pub fn push_record(&mut self, record: PieceTableRecord) {
        self.records.push(record);
        self.metadata.text_len += 1;
    }

    fn get_indexes(&self, at: usize) -> (u32, u32) {
        // return record and char index
        // iter through the records, sum up record.len
        // we have "at" which points to the index in the original text.
        let mut cumulative_sum = 0;
        for (i, record) in self.records.iter().enumerate() {
            cumulative_sum += record.len as usize;
            if cumulative_sum > at {
                let rest = (cumulative_sum as usize) - at;
                let char_index = (record.len as usize) - rest;

                return (i as u32, char_index as u32);
            }
        }
        return (self.records.len() as u32, 0);
    }
    fn split_and_insert_record(
        &mut self,
        record: PieceTableRecord,
        record_index: usize,
        char_index: u32,
    ) {
        let first_record = self.records.get_mut(record_index).unwrap();
        let third_record = PieceTableRecord {
            buf_kind: first_record.buf_kind,
            start: char_index,
            len: first_record.len - char_index,
        };

        first_record.len = char_index;

        self.records.reserve(2);
        unsafe {
            // infallible
            // The spot to put the new value
            {
                let p = self.records.as_mut_ptr().add(record_index);
                if record_index < self.records.len() {
                    // Shift everything over to make space. (Duplicating the
                    // `index`th element into two consecutive places.)
                    std::ptr::copy(p.add(1), p.add(3), self.records.len() - record_index - 1);
                }
                // Write it in, overwriting the first copy of the `index`th
                // element.
                std::ptr::write(p.add(1), record);
                std::ptr::write(p.add(2), third_record);
            }
            self.records.set_len(self.records.len() + 2);
        }

        _ = write!(std::io::stdout(), "records: {:#?}\n", self.records);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn read_file_and_display() {
    //     let table = PieceTable::from_path("samples/daffodils.txt");
    //     // table.show();
    // }

    #[test]
    fn insert_at_end() {
        let sample = include_str!("../samples/daffodils.txt");
        let mut table = PieceTable::new();

        for ch in sample.chars() {
            table.insert_at_end(ch);
        }

        let text: String = table.into();
        eprintln!(
            "+++++++PieceTable+++++++++++\n{}\nlen:{}\n++++++++++++++++",
            text,
            text.len()
        );
        eprintln!(
            "+++++++Sample+++++++++++\n{}\nlen:{}\n++++++++++++++++",
            sample,
            sample.len()
        );
        assert_eq!(sample, &text);
    }

    // #[test]
    //     fn insert_chunk_fn() {
    //         let mut table = PieceTable::from_path("samples/daffodils.txt");
    //         let at = "I wandered lonely as a cloud
    // That floats on high o'er vales and hills,
    // When all at once I saw a crowd,
    // "
    //         .len();
    //         let content = "\nA host, of golden daffodils; \nBeside the lake, beneath the trees, \nFluttering and dancing in the breeze.";
    //
    //         table.insert_chunk(at, content);
    //
    //         let at = "I wandered lonely as a cloud
    // That floats on high o'er vales and hills,
    // When all at once I saw a crowd,
    // A host, of golden daffodils;
    // Beside the lake, beneath the trees,
    // Fluttering and dancing in the breeze.
    //
    // Continuous as the stars that shine
    // And twinkle on the milky way,"
    //             .len();
    //         let content = "They stretched in never-ending line
    // Along the margin of a bay:
    // Ten thousand saw I at a glance,
    // Tossing their heads in sprightly dance.";
    //         table.insert_chunk(at, content);
    //
    //         // Inserting at the end
    //         let at = table.metadata.text_len;
    //         let content = "
    // The waves beside them danced; but they
    // Out-did the sparkling waves in glee:
    // A poet could not but be gay,
    // In such a jocund company:
    // I gazed—and gazed—but little thought
    // What wealth the show to me had brought:";
    //         table.insert_chunk(at, content);
    //
    //         table.show();
    //     }
}
