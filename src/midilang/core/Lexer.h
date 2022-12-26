#ifndef MIDILANG_LEXER_H
#define MIDILANG_LEXER_H

#include "../lang/TokenType.h"
#include "../lang/Token.h"
#include "../lang/ErrorHandler.h"


namespace MIDILang {
    class Lexer {
        public:
            Lexer(std::string file, ErrorHandler* errors);
            std::vector<Token> lex();

        private:
            Token scanToken();
            Token keyword();
            Token number();

            std::string currentLiteral();
            void skipWhiteSpace();
            char advance();
            char peek();
            char peekNext();

            bool isAlpha(char c);
            bool isDigit(char c);
            bool isAtEnd();

            Token makeToken(TokenType ttype);
            Token errorToken(std::string message);

            std::string file;
            std::vector<Token> tokens;
            ErrorHandler* errors;

            int start;
            int current;
            int line;
    };
}


#endif
