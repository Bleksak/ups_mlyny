#include "error.h"

static const char* err_table[] = {
    "",
};

const char* get_err_message(ErrorCode code) {
    return err_table[code];
}