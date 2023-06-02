#ifndef MIDILANG_TOKENTYPE_H
#define MIDILANG_TOKENTYPE_H


namespace MIDILang {
    enum TokenType {
        // Keywords
        TK_IMPORT, TK_TUNE, TK_VAR,
        TK_LOOP, TK_IF, TK_ELSE,
        TK_PLAY, TK_NOTE,

        TK_IDENTIFIER, TK_NUMBER, // Data Tokens
        TK_EQUAL, TK_COLON, TK_LEFT_BRACE, TK_RIGHT_BRACE, TK_SEMICOLON, // Punctuation
        TK_PLUS, TK_MINUS, TK_MULTIPLY, TK_DIVIDE, // Operators
        TK_EOF, TK_ERROR, // Special Tokens
    };
}


#endif
