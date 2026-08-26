use std::fmt::Write;

/// Generate LLVM IR for a SpeckyLang source file.
///
/// SpeckyLang values are dynamic and can include arbitrary-precision numbers, so
/// the generated entry point delegates execution to the same runtime used by
/// the interpreter. The source is embedded in the native module, which keeps
/// compiled programs self-contained and behaviorally identical.
pub fn emit_llvm(source: &str) -> String {
    let source_bytes = source.as_bytes();
    let length = source_bytes.len() + 1;
    let mut literal = String::with_capacity(length * 4);

    for byte in source_bytes.iter().copied().chain(std::iter::once(0)) {
        write!(&mut literal, "\\{:02X}", byte).unwrap();
    }

    format!(
        "; SpeckyLang LLVM module\n\
         target triple = \"{target_triple}\"\n\
         @specky_source = private unnamed_addr constant [{length} x i8] c\"{literal}\"\n\
         declare i32 @specky_run(ptr, i64)\n\
         define i32 @main() {{\n\
         entry:\n\
           %source = getelementptr inbounds [{length} x i8], ptr @specky_source, i64 0, i64 0\n\
           %status = call i32 @specky_run(ptr %source, i64 {source_length})\n\
           ret i32 %status\n\
         }}\n",
        target_triple = if cfg!(target_os = "windows") {
            "x86_64-w64-windows-gnu"
        } else {
            ""
        },
        source_length = source_bytes.len(),
    )
}

#[cfg(test)]
mod tests {
    use super::emit_llvm;

    #[test]
    fn embeds_source_and_exposes_main() {
        let ir = emit_llvm("|< a {%}");

        assert!(ir.contains("define i32 @main()"));
        assert!(ir.contains("declare i32 @specky_run(ptr, i64)"));
        assert!(ir.contains("\\7C\\3C\\20\\61\\20\\7B\\25\\7D\\00"));
    }

    #[test]
    fn escapes_non_ascii_source_bytes() {
        let ir = emit_llvm("µ");

        assert!(ir.contains("\\C2\\B5\\00"));
    }
}
