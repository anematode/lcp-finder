use iced_x86::{Code, Decoder, DecoderOptions, OpKind};
use object::{Architecture, File, Object, ObjectSection, SectionKind};

/// Represents a particular instruction that will incur an LCP stall.
pub struct LcpEntry {
    pub code_addr: u64,
    pub decoded: String,
    pub instruction_bytes: Vec<u8>,
}

pub fn find_lcps(elf_bytes: &[u8]) -> anyhow::Result<Vec<LcpEntry>> {
    let file = File::parse(elf_bytes)?;
    let File::Elf64(file) = file else {
        return Err(anyhow::Error::msg("Not an elf64 file"));
    };
    if file.architecture() != Architecture::X86_64 {
        return Err(anyhow::Error::msg("Only x86 binaries supported"));
    }

    let mut lcps: Vec<LcpEntry> = Vec::new();
    for section in file.sections() {
        if section.kind() != SectionKind::Text {
            continue; // not code
        }
        let bytes = section.uncompressed_data()?;
        let decoder = Decoder::new(64, &*bytes, DecoderOptions::NONE);
        let mut offset = 0usize;
        for inst in decoder {
            let len = inst.len();
            if matches!(inst.op1_kind(), OpKind::Immediate16)
                && !matches!(inst.code(), Code::Mov_r16_imm16 | Code::Mov_rm16_imm16)
            {
                lcps.push(LcpEntry {
                    code_addr: offset as u64 + section.address(),
                    instruction_bytes: Vec::from(&bytes[offset..offset + len]),
                    decoded: inst.to_string(),
                });
            }

            offset += len;
        }
    }

    Ok(lcps)
}
