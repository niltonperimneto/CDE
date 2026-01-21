use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let xdr_files = vec![
        "../agent.x",
        "../rtable4.x",
        "../rtable3.x",
        "../rtable2.x",
        "../cm.x",
        // cmcb.x might be needed too
    ];

    println!("cargo:rerun-if-changed=../agent.x");
    println!("cargo:rerun-if-changed=../rtable4.x");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");

    for xdr_file in xdr_files {
        let path = Path::new(xdr_file);
        let stem = path.file_stem().unwrap().to_str().unwrap();
        let output_file = Path::new(&out_dir).join(format!("{}.rs", stem));

        // Preprocess: Remove lines starting with '%' or '#' and truncate at 'program' block
        let content = fs::read_to_string(path).expect("Failed to read xdr file");
        let mut filtered_lines = Vec::new();
        for line in content.lines() {
            let trim = line.trim_start();
            if trim.starts_with("program") {
                break; // Stop at program block (xdrgen doesn't support it)
            }
            if !trim.starts_with('%') && !trim.starts_with('#') {
                filtered_lines.push(line);
            }
        }
        let mut filtered_content = filtered_lines.join("\n");

        // Patch cm.x array definitions to Use XDR array syntax (<>) instead of pointer + count.
        // This ensures xdrgen generates Vec<T> instead of Option<Box<T>>.
        // Patterns are sensitive to whitespace (tabs).

        // Pattern 1: cms_attribute arrays
        filtered_content = filtered_content.replace(
            "CSA_uint32\tnum_attrs;\n\tcms_attribute\t*attrs;",
            "cms_attribute\tattrs<>;",
        );
        filtered_content = filtered_content.replace(
            "u_int\t\tnum_attrs;\n\tcms_attribute\t*attrs;",
            "cms_attribute\tattrs<>;",
        );

        // Pattern 2: cms_attr_name arrays
        filtered_content = filtered_content.replace(
            "CSA_uint32\tnum_names;\n\tcms_attr_name\t*names;",
            "cms_attr_name\tnames<>;",
        );

        // Pattern 3: cms_name arrays (cms_list_calendars_res)
        filtered_content = filtered_content.replace(
            "CSA_uint32\tnum_names;\n\tcms_name\t*names;",
            "cms_name\tnames<>;",
        );

        // Pattern 4: cms_key arrays
        filtered_content = filtered_content.replace(
            "u_int\t\tnum_keys;\n\tcms_key\t\t*keys;",
            "cms_key\t\tkeys<>;",
        );

        // Inject missing definitions
        filtered_content.push_str("\n%// Injected definitions\n");
        filtered_content.push_str("typedef unsigned int CSA_uint32;\n");
        filtered_content.push_str("typedef int CSA_sint32;\n");
        filtered_content.push_str("typedef CSA_uint32 CSA_boolean;\n");
        filtered_content.push_str("typedef CSA_sint32 CSA_enum;\n");
        filtered_content.push_str("typedef CSA_uint32 CSA_flags;\n");

        if stem == "cm" {
            // CSA_opaque_data (xdr_bytes)
            filtered_content.push_str("struct CSA_opaque_data { opaque data<>; };\n");
            // CSA_date_time_entry
            filtered_content.push_str(
                "struct CSA_date_time_entry { string date_time<>; CSA_date_time_entry *next; };\n",
            );
            // CSA_reminder
            filtered_content.push_str("struct CSA_reminder { string lead_time<>; string snooze_time<>; unsigned int repeat_count; CSA_opaque_data reminder_data; };\n");

            // cms_attribute_value union
            filtered_content.push_str("union cms_attribute_value switch (int type) {\n");
            filtered_content.push_str("case 0: CSA_boolean boolean_value;\n");
            filtered_content.push_str("case 1: CSA_enum enumerated_value;\n");
            filtered_content.push_str("case 2: CSA_flags flags_value;\n");
            filtered_content.push_str("case 3: int sint32_value;\n");
            filtered_content.push_str("case 4: unsigned int uint32_value;\n");
            filtered_content.push_str("case 5: string string_value<>;\n");
            filtered_content.push_str("case 6: string calendar_user_value<>;\n");
            filtered_content.push_str("case 7: string date_time_value<>;\n");
            filtered_content.push_str("case 8: string date_time_range_value<>;\n");
            filtered_content.push_str("case 9: string time_duration_value<>;\n");
            filtered_content.push_str("case 10: cms_access_entry *access_list_value;\n"); // Pointer? xdr_pointer in C
                                                                                          // case 10 in C was access_list_value pointer. cms_access_entry is linked list node.
            filtered_content.push_str("case 12: CSA_date_time_entry *date_time_list_value;\n");
            filtered_content.push_str("case 13: CSA_reminder *reminder_value;\n");
            filtered_content.push_str("case 14: CSA_opaque_data *opaque_data_value;\n");
            filtered_content.push_str("default: void;\n");
            filtered_content.push_str("};\n");
        }

        let temp_file_path = Path::new(&out_dir).join(format!("{}_pp.x", stem));
        fs::write(&temp_file_path, filtered_content).expect("Failed to write preprocessed xdr");

        // Invoke xdrgen on temp file
        let xdrgen_path = if let Ok(home) = env::var("HOME") {
            let p = std::path::PathBuf::from(home).join(".cargo/bin/xdrgen");
            if p.exists() {
                p
            } else {
                std::path::PathBuf::from("xdrgen")
            }
        } else {
            std::path::PathBuf::from("xdrgen")
        };

        let status = Command::new(xdrgen_path)
            .arg(&temp_file_path)
            .stdout(fs::File::create(&output_file).expect("Failed to create output file"))
            .status();

        match status {
            Ok(s) if s.success() => {
                println!(
                    "cargo:warning=Successfully generated {}",
                    output_file.display()
                );

                // Patch the generated code to use our shim which discards the argument
                let gen_content =
                    fs::read_to_string(&output_file).expect("Failed to read generated rs");
                let patched_content = gen_content
                    .replace(
                        "xdr_codec :: Error :: invalidenum",
                        "crate :: my_shim :: invalidenum",
                    )
                    .replace(
                        "xdr_codec :: Error :: invalidcase",
                        "crate :: my_shim :: invalidcase",
                    );
                fs::write(&output_file, patched_content).expect("Failed to write patched rs");
            }
            Ok(s) => {
                println!("cargo:warning=xdrgen failed with {}", s);
                // Don't panic yet, maybe some files fail but others work
            }
            Err(e) => {
                println!("cargo:warning=Failed to execute xdrgen: {}", e);
            }
        }
    }

    // Generate C headers using cbindgen
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // We want to generate headers for the C code.
    // The C code expects multiple headers (agent.h, rtable4.h).
    // cbindgen generates ONE header for the crate usually.
    // We might need to configure it to create one monolithic header 'csa_xdr.h' and have the C code include that, OR
    // split it. For now, let's generate one 'libcsa_xdr.h' and see if we can shim the others.

    // Create header directory
    let include_dir = Path::new(&crate_dir).join("include");
    fs::create_dir_all(&include_dir).unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir.clone())
        .with_language(cbindgen::Language::C)
        .with_include_guard("LIBCSA_XDR_H")
        .with_after_include("#include <rpc/clnt.h>\n#include <csa/csa.h>\n#include \"cm.h\"\n#include \"connection.h\"\n#include \"calendar.h\"\n#include \"rtable4.h\"\n#include \"rtable3.h\"\n#include \"rtable2.h\"\n\ntypedef enum clnt_stat clnt_stat;")
        .with_parse_deps(true)
        .with_parse_include(&["libcsa_xdr"]) // Parse specific crates if needed
        .exclude_item("CSA_return_code") // Defined in csa.h
        .exclude_item("Table_Args_4")
        .exclude_item("Table_Res_4")
        .exclude_item("Table_Status_4")
        .exclude_item("Table_Op_Args_4")
        .exclude_item("Access_Args_4")
        .exclude_item("Access_Status_4")
        .exclude_item("Registration_4")
        .exclude_item("Registration_Status_4")
        .exclude_item("Table_Args_3")
        .exclude_item("Table_Res_3")
        .exclude_item("Access_Args_3")
        .exclude_item("Access_Status_3")
        .exclude_item("Table_Status_3")
        .exclude_item("Registration_3")
        .exclude_item("Registration_Status_3")
        .exclude_item("Table_Args_2")
        .exclude_item("Table_Res_2")
        .exclude_item("Access_Args_2")
        .exclude_item("Access_Status_2")
        .exclude_item("Table_Status_2")
        .exclude_item("Registration_2")
        .exclude_item("Registration_Status_2")
        // Added exclusions to prevent conflicts with C headers
        .exclude_item("CSA_attribute")
        .exclude_item("CSA_enum")
        .exclude_item("CSA_uint32")
        .exclude_item("CSA_sint32")
        .exclude_item("CSA_boolean")
        .exclude_item("CSA_flags")
        .exclude_item("_DtCmNameTable")
        .exclude_item("_DtCmCallbackEntry")
        .exclude_item("_DtCm_Target_List")
        .exclude_item("_DtCm_Client_Info")
        .exclude_item("_DtCm_Transport_Type")
        .exclude_item("_DtCm_Connection")
        .exclude_item("Calendar")
        .exclude_item("cms_attribute")
        .exclude_item("CSA_SUCCESS")
        .exclude_item("CSA_E_INVALID_PARAMETER")
        .exclude_item("CSA_E_SERVICE_UNAVAILABLE")
        .exclude_item("TABLEVERS")
        .exclude_item("TABLEVERS_4")
        .exclude_item("_DtCM_LONG_TIMEOUT")
        .exclude_item("B_TRUE")
        .exclude_item("B_FALSE")
        .exclude_item("RPC_SUCCESS")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(include_dir.join("libcsa_xdr.h"));

    let cde_include = Path::new(&crate_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("include");
    let csa_lib = Path::new(&crate_dir).parent().unwrap();

    // Generate C bindings for XDR-generated headers
    // Use ../cm.h to avoid picking up local include/cm.h if it exists
    let xdr_c_bindings = bindgen::Builder::default()
        .header_contents(
            "wrapper_xdr.h",
            "#include \"../cm.h\"\n#include \"../connection.h\"\n#include \"../rtable2.h\"\n#include \"../rtable3.h\"\n#include \"../rtable4.h\"\n#include <csa.h>",
        )
        .clang_arg(format!("-I{}", include_dir.display()))
        .clang_arg("-I/usr/include/tirpc")
        .clang_arg(format!("-I{}", cde_include.display())) // cde/include
        .clang_arg(format!("-I{}", cde_include.join("csa").display())) // cde/include/csa
        .clang_arg(format!("-I{}", csa_lib.display())) // cde/lib/csa (for cm.h)
        .allowlist_type("cms_.*")
        .allowlist_type("CSA_.*")
        .allowlist_type("_DtCm_.*")
        .allowlist_type(".*_4")
        .allowlist_type(".*_3")
        .allowlist_type(".*_2")
        .allowlist_type("CLIENT")
        .allowlist_type("XDR")
        .allowlist_type("bool_t")
        .allowlist_type("enum_t")
        .allowlist_function("xdr_.*")
        .derive_default(true)
        .generate()
        .expect("Unable to generate XDR C bindings");

    xdr_c_bindings
        .write_to_file(Path::new(&out_dir).join("xdr_c_bindings.rs"))
        .expect("Couldn't write XDR C bindings!");

    // Generate libtirpc bindings using bindgen
    let bindings = bindgen::Builder::default()
        .header_contents("wrapper.h", "#include <rpc/rpc.h>")
        .clang_arg("-I/usr/include/tirpc")
        .derive_default(true)
        // Whitelist what we need to minimize content
        .allowlist_type("CLIENT")
        .allowlist_type("XDR")
        .allowlist_type("bool_t")
        .allowlist_type("enum_t")
        .allowlist_function("clnt_call")
        .allowlist_type("CLIENT")
        .allowlist_type("XDR")
        .allowlist_type("bool_t")
        .allowlist_type("enum_t")
        .allowlist_function("clnt_call")
        .allowlist_function("xdr_.*") // Allow all xdr helper functions (xdr_int, xdr_u_int, xdr_pointer, etc)
        .generate()
        .expect("Unable to generate libtirpc bindings");

    let out_path = Path::new(&out_dir).join("tirpc_bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
