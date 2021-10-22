use crate::font::FontError;

pub struct TableMaxp {
    pub version_major: u16,
    pub version_minor: u16,
    pub num_glyphs: u16,
    pub max_points: u16,
    pub max_contours: u16,
    pub max_component_points: u16,
    pub max_component_contours: u16,
    pub max_zones: u16,
    pub max_twilight_points: u16,
    pub max_storage: u16,
    pub max_function_defs: u16,
    pub max_instruction_defs: u16,
    pub max_stack_elements: u16,
    pub max_size_of_instructions: u16,
    pub max_component_elements: u16,
    pub max_component_depth: u16,
}

impl TableMaxp {
    pub fn new(bytes: &[u8]) -> Result<Self, FontError> {
        let mut offset = 0;

        let version_major = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let version_minor = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        if version_major != 1 || version_minor != 0 {
            return Err(format!(
                "Unexpected maxp version {}.{}",
                version_major, version_minor
            )
            .into());
        }

        let num_glyphs = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let max_points = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let max_contours = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let max_component_points = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let max_component_contours = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let max_zones = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        if max_zones != 2 {
            return Err(format!("Invalid number of max zones in maxp table: {}", max_zones).into());
        }

        let max_twilight_points = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let max_storage = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let max_function_defs = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let max_instruction_defs = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let max_stack_elements = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let max_size_of_instructions = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let max_component_elements = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let max_component_depth = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        if offset != bytes.len() {
            return Err("Failed parsing the entirety of the head table".into());
        }

        Ok(TableMaxp {
            version_major,
            version_minor,
            num_glyphs,
            max_points,
            max_contours,
            max_component_points,
            max_component_contours,
            max_zones,
            max_twilight_points,
            max_storage,
            max_function_defs,
            max_instruction_defs,
            max_stack_elements,
            max_size_of_instructions,
            max_component_elements,
            max_component_depth,
        })
    }
}
