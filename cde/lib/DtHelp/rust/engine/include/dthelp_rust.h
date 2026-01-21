#ifndef DTHELP_RUST_H
#define DTHELP_RUST_H

#ifdef __cplusplus
extern "C" {
#endif

/* Parsing Events */
#define EV_TEXT 0
#define EV_HEADER_START 1
#define EV_HEADER_END 2
#define EV_PARA_START 3
#define EV_PARA_END 4
#define EV_BOLD_START 5
#define EV_BOLD_END 6
#define EV_ITALIC_START 7
#define EV_ITALIC_END 8
#define EV_CODE_START 9
#define EV_CODE_END 10
#define EV_LINK_START 11
#define EV_LINK_END 12

typedef void (*ParseCallback)(void *ctx, int event_type, const char *text);

void dthelp_parse_markdown(const char *buffer, void *ctx,
                           ParseCallback callback);

#ifdef __cplusplus
}
#endif

#endif /* DTHELP_RUST_H */
