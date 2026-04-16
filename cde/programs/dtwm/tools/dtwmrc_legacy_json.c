#include <locale.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "WmParse.h"

static void json_escape(FILE *out, const char *s) {
    const unsigned char *p = (const unsigned char *)s;
    for (; *p; ++p) {
        switch (*p) {
        case '\\': fputs("\\\\", out); break;
        case '"': fputs("\\\"", out); break;
        case '\b': fputs("\\b", out); break;
        case '\f': fputs("\\f", out); break;
        case '\n': fputs("\\n", out); break;
        case '\r': fputs("\\r", out); break;
        case '\t': fputs("\\t", out); break;
        default:
            if (*p < 0x20) {
                fprintf(out, "\\u%04x", (unsigned int)*p);
            } else {
                fputc(*p, out);
            }
            break;
        }
    }
}

static void json_string(FILE *out, const char *s) {
    fputc('"', out);
    json_escape(out, s ? s : "");
    fputc('"', out);
}

static int brace_delta(const char *line, bool *has_open) {
    bool in_quote = false;
    bool escape = false;
    int delta = 0;
    *has_open = false;

    for (const char *p = line; *p; ++p) {
        char c = *p;
        if (escape) {
            escape = false;
            continue;
        }
        if (c == '\\') {
            escape = true;
            continue;
        }
        if (c == '"') {
            in_quote = !in_quote;
            continue;
        }
        if (!in_quote) {
            if (c == '#') {
                break;
            }
            if (c == '{') {
                delta += 1;
                *has_open = true;
            } else if (c == '}') {
                delta -= 1;
            }
        }
    }

    return delta;
}

static const char *normalize_kind(const char *tok) {
    if (!tok) {
        return NULL;
    }
    if (strcmp(tok, "Menu") == 0) {
        return "Menu";
    }
    if (strcmp(tok, "Keys") == 0) {
        return "Keys";
    }
    if (strcmp(tok, "Buttons") == 0) {
        return "Buttons";
    }
    if (strcmp(tok, "PANEL") == 0) {
        return "PANEL";
    }
    if (strcmp(tok, "BOX") == 0) {
        return "BOX";
    }
    if (strcmp(tok, "CONTROL") == 0) {
        return "CONTROL";
    }
    if (strcmp(tok, "SWITCH") == 0) {
        return "SWITCH";
    }
    return NULL;
}

static char **collect_tokens(unsigned char *line, size_t *count) {
    unsigned char *cursor = line;
    unsigned char *tok = NULL;
    char **tokens = NULL;
    size_t used = 0;

    while ((tok = _DtWmParseNextTokenC(&cursor, False)) != NULL) {
        char **next = realloc(tokens, (used + 1) * sizeof(*tokens));
        if (!next) {
            break;
        }
        tokens = next;
        tokens[used] = strdup((char *)tok);
        if (!tokens[used]) {
            break;
        }
        used++;
    }

    *count = used;
    return tokens;
}

static void free_tokens(char **tokens, size_t count) {
    for (size_t i = 0; i < count; ++i) {
        free(tokens[i]);
    }
    free(tokens);
}

static void emit_tokens(FILE *out, char **tokens, size_t count) {
    fputc('[', out);
    for (size_t i = 0; i < count; ++i) {
        if (i > 0) {
            fputc(',', out);
        }
        json_string(out, tokens[i]);
    }
    fputc(']', out);
}

int main(int argc, char **argv) {
    if (argc != 2) {
        fprintf(stderr, "usage: %s <dtwmrc>\n", argv[0]);
        return 2;
    }

    setlocale(LC_ALL, "");

    FILE *fp = fopen(argv[1], "r");
    if (!fp) {
        perror("fopen");
        return 1;
    }

    DtWmpParseBuf *pb = _DtWmParseNewBuf();
    if (!pb) {
        fprintf(stderr, "failed to allocate parse buffer\n");
        fclose(fp);
        return 1;
    }
    _DtWmParseSetFile(pb, fp);

    bool in_decl = false;
    bool body_started = false;
    int brace_depth = 0;
    bool first_decl = true;
    bool first_item = true;
    const char *decl_kind = NULL;

    fputs("{\"declarations\":[", stdout);

    unsigned char *line;
    while ((line = _DtWmParseNextLine(pb)) != NULL) {
        char *line_copy = strdup((char *)line);
        if (!line_copy) {
            continue;
        }
        bool has_open = false;
        int delta = brace_delta(line_copy, &has_open);

        if (!in_decl) {
            unsigned char *cursor = line;
            unsigned char *tok = _DtWmParseNextTokenC(&cursor, False);
            if (tok) {
                unsigned char *name = _DtWmParseNextTokenC(&cursor, False);
                const char *kind = normalize_kind((char *)tok);
                if (kind && name) {
                    in_decl = true;
                    body_started = false;
                    brace_depth = 0;
                    decl_kind = kind;

                    if (!first_decl) {
                        fputc(',', stdout);
                    }
                    first_decl = false;

                    fputs("{\"kind\":", stdout);
                    json_string(stdout, decl_kind);
                    fputs(",\"name\":", stdout);
                    json_string(stdout, (char *)name);
                    fputs(",\"items\":[", stdout);
                    first_item = true;

                    if (has_open) {
                        body_started = true;
                    }
                    brace_depth += delta;
                }
            }
            free(line_copy);
            continue;
        }

        if (!body_started && has_open) {
            body_started = true;
        }

        if (body_started) {
            int before = brace_depth;
            int after = brace_depth + delta;

            size_t token_count = 0;
            char **tokens = collect_tokens(line, &token_count);
            bool is_outer_open = (!body_started ? false : (before == 0 && after >= 1)) &&
                                 (token_count == 1) && (strcmp(tokens[0], "{") == 0);
            bool is_outer_close = (before == 1 && after == 0) && (token_count == 1) &&
                                  (strcmp(tokens[0], "}") == 0);

            if (token_count > 0 && !is_outer_open && !is_outer_close) {
                if (!first_item) {
                    fputc(',', stdout);
                }
                first_item = false;
                emit_tokens(stdout, tokens, token_count);
            }
            free_tokens(tokens, token_count);

            brace_depth = after;
            if (body_started && brace_depth == 0) {
                fputs("]}", stdout);
                in_decl = false;
                body_started = false;
            }
        }

        free(line_copy);
    }

    if (in_decl) {
        fputs("]}", stdout);
    }

    fputs("]}\n", stdout);

    _DtWmParseDestroyBuf(pb);
    fclose(fp);

    return 0;
}
