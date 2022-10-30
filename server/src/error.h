#pragma once

typedef enum ErrorCode {
    OK,
    
} ErrorCode;

const char* get_err_message(ErrorCode code);
