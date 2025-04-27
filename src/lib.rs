use std::io::Write;

#[derive(Debug, Clone, Copy)]
enum BufKind {
    Original,
    Add,
}

#[derive(Debug, Clone, Copy)]
struct PieceTableRecord {
    buf_kind: BufKind,
    start: u32,
    len: u32,
}

pub struct PieceTable {
    original: String,
    add_buf: String,
    records: Vec<PieceTableRecord>,
}

impl PieceTable {
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

    pub fn insert_chunk(&mut self, at: u32, content: &str) {
        if content.len() > u32::MAX as usize {
            return;
        }
        eprintln!(
            "content to be inserted: \nlen = {} \ncontents: `{}`",
            content.len(),
            content
        );
        let (record_index, char_index) = self.get_indexes(at);
        eprintln!(
            "record_index: {} \nchar_index: {}",
            record_index, char_index
        );
        let record = PieceTableRecord {
            buf_kind: BufKind::Add,
            start: self.add_buf.len() as u32,
            len: content.len() as u32,
        };
        eprintln!("record to be inserted: {:#?}", record);
        self.add_buf.push_str(content);
        self.split_and_add_record(record, record_index as usize, char_index);
    }

    fn get_indexes(&self, at: u32) -> (u32, u32) {
        // return record and char index
        // iter through the records, sum up record.len
        // we have "at" which points to the index in the original text.
        let mut cumulative_sum = 0;
        for (i, record) in self.records.iter().enumerate() {
            cumulative_sum += record.len;
            if cumulative_sum > at {
                let rest = cumulative_sum - at;
                let char_index = record.len - rest;

                return (i as u32, char_index);
            }
        }
        return ((self.records.len() + 1) as u32, 0);
    }
    fn split_and_add_record(
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

    #[test]
    fn read_file_and_display() {
        let table = PieceTable::from_path("samples/daffodils.txt");
        // table.show();
    }

    #[test]
    fn insert_chunk_fn() {
        let mut table = PieceTable::from_path("samples/daffodils.txt");
        table.insert_chunk(4, "huhuhu HAHAHAHAHAHAHAHAH");
        table.insert_chunk(4, " Next Chunk ");
        table.insert_chunk(15, " Third Chunk ");
        table.insert_chunk(10, " Fourth Chunk");
        table.show();
    }
}
