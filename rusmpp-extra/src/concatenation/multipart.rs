#[derive(Debug)]
pub enum MultipartType {
    Udh { size: usize },
    Sar,
}

#[derive(Debug)]
pub struct MultipartSegment {
    pub r#type: MultipartType,
    pub reference: u16,
    pub part_number: u8,
    pub total_parts: u8,
}

impl MultipartSegment {
    pub const fn udh_size(&self) -> Option<usize> {
        match self.r#type {
            MultipartType::Udh { size } => Some(size),
            MultipartType::Sar => None,
        }
    }

    pub const fn header_size(&self) -> usize {
        match self.udh_size() {
            Some(size) => size,
            None => 0,
        }
    }
}

#[derive(Debug)]
pub enum MultipartSegmentError {}
