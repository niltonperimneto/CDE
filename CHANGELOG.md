# Changelog

## C23 Upgrade and dtudcexch Porting

### Build Configuration
*   **cde/lib/DtSvc/meson.build**: Exposed `DtSvc` as a global dependency via `meson.override_dependency`.
*   **programs/dtudcexch/meson.build**: Added `DtSvc` dependency to resolve linking errors (`_DtGetMessage`, `_DtEnvControl`) and removed `xlfdutil.c` source to prevent duplicate symbols.

### Code Modernization (C Standard Upgrade)
*   **programs/dtudcfonted/libfal/falfont.c**: 
    *   Converted K&R function definitions to ANSI C prototypes.
    *   Fixed conflicting type declarations for `falReadFontInfoLists`, `falReadGpfProp` and others.
    *   Updated `falReadFontInfoLists` calls to pass `FalFontData` struct by value (using dummy struct) instead of `NULL` pointer.
    *   Added missing `codeset_list_num` static variable.
    *   Fixed pointer signedness mismatch in `fal_conv_glyph_to_code`.
*   **programs/dtudcfonted/libfal/_fallcPublic.c**: Fixed function prototype for `_fallcMapOSLocaleName`.
