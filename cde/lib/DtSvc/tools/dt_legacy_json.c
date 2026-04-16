#include <locale.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <X11/Xresource.h>

#include <Dt/DbReader.h>
#include <Dt/DbUtil.h>

typedef struct {
    char *kind;
    char *name;
    char **keys;
    char **values;
    size_t field_count;
} LegacyRecord;

static LegacyRecord *records = NULL;
static size_t record_count = 0;
static size_t record_cap = 0;

static char *target_path = NULL;
static char *target_base = NULL;

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

static char *trim_copy(const char *s) {
    if (!s) {
        return strdup("");
    }
    const char *start = s;
    while (*start && (*start == ' ' || *start == '\t' || *start == '\r' || *start == '\n')) {
        start++;
    }
    const char *end = start + strlen(start);
    while (end > start) {
        char c = end[-1];
        if (c == ' ' || c == '\t' || c == '\r' || c == '\n') {
            end--;
        } else {
            break;
        }
    }
    size_t len = (size_t)(end - start);
    char *out = (char *)malloc(len + 1);
    if (!out) {
        return NULL;
    }
    memcpy(out, start, len);
    out[len] = '\0';
    return out;
}

static char *unquote_value(const char *s) {
    char *trimmed = trim_copy(s);
    if (!trimmed) {
        return NULL;
    }
    size_t len = strlen(trimmed);
    if (len >= 2 && trimmed[0] == '"' && trimmed[len - 1] == '"') {
        char *out = (char *)malloc(len - 1);
        if (!out) {
            free(trimmed);
            return NULL;
        }
        size_t j = 0;
        for (size_t i = 1; i + 1 < len; ++i) {
            if (trimmed[i] == '\\' && i + 1 < len - 1) {
                char next = trimmed[i + 1];
                if (next == '\\' || next == '"') {
                    out[j++] = next;
                    i++;
                    continue;
                }
            }
            out[j++] = trimmed[i];
        }
        out[j] = '\0';
        free(trimmed);
        return out;
    }
    return trimmed;
}

static const char *strip_host_prefix(const char *path) {
    const char *colon = strchr(path, ':');
    if (colon && colon[1] == '/') {
        return colon + 1;
    }
    return path;
}

static bool path_matches(const char *path) {
    if (!path) {
        return false;
    }
    const char *norm = strip_host_prefix(path);
    if (target_path && strcmp(norm, target_path) == 0) {
        return true;
    }
    if (target_base) {
        size_t nlen = strlen(norm);
        size_t blen = strlen(target_base);
        if (nlen >= blen && strcmp(norm + (nlen - blen), target_base) == 0) {
            if (nlen == blen || norm[nlen - blen - 1] == '/') {
                return true;
            }
        }
    }
    return false;
}

static void add_record(LegacyRecord rec) {
    if (record_count == record_cap) {
        size_t next = record_cap == 0 ? 8 : record_cap * 2;
        LegacyRecord *new_records = realloc(records, next * sizeof(*records));
        if (!new_records) {
            return;
        }
        records = new_records;
        record_cap = next;
    }
    records[record_count++] = rec;
}

static void add_field(LegacyRecord *rec, const char *key, const char *value) {
    char **new_keys = realloc(rec->keys, (rec->field_count + 1) * sizeof(*rec->keys));
    char **new_values = realloc(rec->values, (rec->field_count + 1) * sizeof(*rec->values));
    if (!new_keys || !new_values) {
        free(new_keys);
        free(new_values);
        return;
    }
    rec->keys = new_keys;
    rec->values = new_values;
    rec->keys[rec->field_count] = strdup(key ? key : "");
    rec->values[rec->field_count] = unquote_value(value ? value : "");
    if (!rec->values[rec->field_count]) {
        rec->values[rec->field_count] = strdup("");
    }
    rec->field_count++;
}

static void sort_field_order(const LegacyRecord *rec, size_t *order, size_t count) {
    for (size_t i = 1; i < count; ++i) {
        size_t key = order[i];
        size_t j = i;
        while (j > 0 && strcmp(rec->keys[order[j - 1]], rec->keys[key]) > 0) {
            order[j] = order[j - 1];
            j--;
        }
        order[j] = key;
    }
}

static Boolean capture_converter(
    DtDtsDbField *fields,
    DtDbPathId pathId,
    char *hostPrefix,
    Boolean rejectionStatus
) {
    (void)hostPrefix;
    (void)rejectionStatus;

    char *path = _DtDbPathIdToString(pathId);
    bool match = path_matches(path);
    free(path);

    if (!match) {
        return False;
    }

    const char *kind = XrmQuarkToString(fields[0].fieldName);
    char *name = trim_copy(fields[0].fieldValue);
    if (!kind || !name) {
        free(name);
        return False;
    }

    LegacyRecord rec = {0};
    rec.kind = strdup(kind);
    rec.name = name;
    rec.keys = NULL;
    rec.values = NULL;
    rec.field_count = 0;

    for (int i = 1; fields[i].fieldName != (XrmQuark)0; i++) {
        const char *key = XrmQuarkToString(fields[i].fieldName);
        if (!key) {
            continue;
        }
        add_field(&rec, key, fields[i].fieldValue);
    }

    add_record(rec);
    return False;
}

static void emit_records(void) {
    fputs("{\"records\":[", stdout);
    for (size_t i = 0; i < record_count; ++i) {
        LegacyRecord *rec = &records[i];
        if (i > 0) {
            fputc(',', stdout);
        }
        fputs("{\"kind\":", stdout);
        json_string(stdout, rec->kind);
        fputs(",\"name\":", stdout);
        json_string(stdout, rec->name);
        fputs(",\"fields\":{", stdout);

        if (rec->field_count > 0) {
            size_t *order = malloc(rec->field_count * sizeof(*order));
            if (order) {
                for (size_t j = 0; j < rec->field_count; ++j) {
                    order[j] = j;
                }
                sort_field_order(rec, order, rec->field_count);
                for (size_t j = 0; j < rec->field_count; ++j) {
                    size_t idx = order[j];
                    if (j > 0) {
                        fputc(',', stdout);
                    }
                    json_string(stdout, rec->keys[idx]);
                    fputc(':', stdout);
                    json_string(stdout, rec->values[idx]);
                }
                free(order);
            }
        }

        fputs("}}", stdout);
    }
    fputs("]}\n", stdout);
}

static void free_records(void) {
    for (size_t i = 0; i < record_count; ++i) {
        LegacyRecord *rec = &records[i];
        free(rec->kind);
        free(rec->name);
        for (size_t j = 0; j < rec->field_count; ++j) {
            free(rec->keys[j]);
            free(rec->values[j]);
        }
        free(rec->keys);
        free(rec->values);
    }
    free(records);
}

int main(int argc, char **argv) {
    if (argc != 2) {
        fprintf(stderr, "usage: %s <file.dt>\n", argv[0]);
        return 2;
    }

    setlocale(LC_ALL, "");

    target_path = strdup(argv[1]);
    if (!target_path) {
        return 1;
    }
    const char *base = strrchr(argv[1], '/');
    target_base = strdup(base ? base + 1 : argv[1]);
    if (!target_base) {
        free(target_path);
        return 1;
    }

    char *dir = strdup(argv[1]);
    if (!dir) {
        free(target_path);
        free(target_base);
        return 1;
    }
    char *slash = strrchr(dir, '/');
    if (slash) {
        if (slash == dir) {
            slash[1] = '\0';
        } else {
            *slash = '\0';
        }
    } else {
        free(dir);
        dir = strdup(".");
    }

    char **dirs = calloc(2, sizeof(*dirs));
    char **paths = calloc(2, sizeof(*paths));
    if (!dirs || !paths) {
        free(dir);
        free(dirs);
        free(paths);
        free(target_path);
        free(target_base);
        return 1;
    }
    dirs[0] = strdup(dir);
    paths[0] = strdup(dir);

    DtDirPaths dir_paths = {0};
    dir_paths.dirs = dirs;
    dir_paths.paths = paths;

    static DtDbConverter converters[] = { capture_converter, NULL };
    DtDbRecordDesc descs[] = {
        { "ACTION", DTUNLIMITEDFIELDS, converters },
        { "DATA_ATTRIBUTES", DTUNLIMITEDFIELDS, converters },
        { "DATA_CRITERIA", DTUNLIMITEDFIELDS, converters },
        { "SET", DTUNLIMITEDFIELDS, converters },
    };

    _DtDbRead(&dir_paths, ".dt", descs, (int)(sizeof(descs) / sizeof(descs[0])));

    emit_records();

    free_records();
    free(dir);
    free(dirs[0]);
    free(paths[0]);
    free(dirs);
    free(paths);
    free(target_path);
    free(target_base);

    return 0;
}
