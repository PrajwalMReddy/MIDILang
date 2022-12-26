#ifndef MIDILANG_TOKEN_H
#define MIDILANG_TOKEN_H

#include <string>

#include "TokenType.h"


namespace MIDILang {
    struct Token {
        TokenType ttype;
        std::string literal;
        int line;
    };
}


#endif
