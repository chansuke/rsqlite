use derive_new::new;

pub const PAGE_SIZE: u32 = 4096;
pub const ID_SIZE: usize = 4;
pub const USERNAME_SIZE: usize = 32;
pub const EMAIL_SIZE: usize = 255;
pub const ROW_SIZE: u32 = (ID_SIZE + USERNAME_SIZE + EMAIL_SIZE) as u32;
pub const TABLE_MAX_PAGES: u32 = 100;
pub const ROWS_PER_PAGE: u32 = PAGE_SIZE / ROW_SIZE;
pub const TABLE_MAX_ROWS: u32 = ROWS_PER_PAGE * TABLE_MAX_PAGES;

#[derive(new)]
pub struct Pager {
    pub pages: Vec<Vec<u8>>,
}

#[derive(new)]
pub struct Table {
    pub num_rows: u32,
    pub pager: Pager,
}

impl Table {
    pub fn serialize_row(&mut self, row: Row, page_num: u32) {
        let id_bytes = row.id.to_ne_bytes();
        let username_bytes = row.username;
        let email_bytes = row.email;
        self.pager.pages[page_num as usize].extend_from_slice(&id_bytes);
        self.pager.pages[page_num as usize].extend_from_slice(&username_bytes);
        self.pager.pages[page_num as usize].extend_from_slice(&email_bytes);
    }

    pub fn deserialize_row() -> Row {
        todo!();
    }

    pub fn row_slot(&mut self, row_num: u32) -> u32 {
        let page_num = row_num / ROWS_PER_PAGE;
        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;

        page_num + byte_offset
    }
}

#[derive(new)]
pub struct Row {
    pub id: u32,
    pub username: [u8; USERNAME_SIZE],
    pub email: [u8; EMAIL_SIZE],
}

impl Row {
    pub fn print_row(&self) {
        println!(
            "({}, {}, {})",
            self.id,
            std::str::from_utf8(&self.username)
                .unwrap()
                .trim_end_matches(char::from(0)),
            std::str::from_utf8(&self.email)
                .unwrap()
                .trim_end_matches(char::from(0))
        );
    }
}
