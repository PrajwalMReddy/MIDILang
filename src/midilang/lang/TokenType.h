#ifndef MIDILANG_TOKENTYPE_H
#define MIDILANG_TOKENTYPE_H


namespace MIDILang {
    enum TokenType {
        // Keywords
        TK_IMPORT, TK_TUNE, TK_VAR,
        TK_LOOP, TK_WHILE, TK_IF, TK_ELSE,
        TK_PLAY, TK_NOTE,

        TK_IDENTIFIER, TK_NUMBER, // Data Tokens

        // Punctuation
        TK_EQUAL, TK_COLON, TK_SEMICOLON,
        TK_LEFT_PAR, TK_RIGHT_PAR, TK_LEFT_BRACE, TK_RIGHT_BRACE,

        // Operators
        TK_COMPARE_EQUAL, TK_COMPARE_NOT_EQUAL, TK_NOT,
        TK_GREATER, TK_LESSER, TK_GREATER_EQUAL, TK_LESSER_EQUAL,
        TK_PLUS, TK_MINUS, TK_MULTIPLY, TK_DIVIDE, TK_MODULUS,

        TK_EOF, TK_ERROR, // Special Tokens
    };
}


#endif
