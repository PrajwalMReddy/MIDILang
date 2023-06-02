#include "Lexer.h"


MIDILang::Lexer::Lexer(std::string file, MIDILang::ErrorHandler *errors) {
    this->file = file;
    this->tokens = std::vector<Token>();
    this->errors = errors;

    this->start = 0;
    this->current = 0;
    this->line = 1;
}

std::vector<MIDILang::Token> MIDILang::Lexer::lex() {
    while (true) {
        Token toAdd = scanToken();
        this->tokens.push_back(toAdd);

        if (toAdd.ttype == TK_EOF) break;
    }

    return this->tokens;
}

MIDILang::Token MIDILang::Lexer::scanToken() {
    skipWhiteSpace();
    this->start = this->current;

    if (isAtEnd()) return makeToken(TK_EOF);

    char c = advance();

    if (isAlpha(c)) return keyword();
    if (isDigit(c)) return number();

    switch (c) {
        // Punctuation
        case '=': return makeToken(TK_EQUAL);
        case ':': return makeToken(TK_COLON);
        case '{': return makeToken(TK_LEFT_BRACE);
        case '}': return makeToken(TK_RIGHT_BRACE);
        case ';': return makeToken(TK_SEMICOLON);

        // Operators
        case '+': return makeToken(TK_PLUS);
        case '-': return makeToken(TK_MINUS);
        case '*': return makeToken(TK_MULTIPLY);
        case '/': return makeToken(TK_DIVIDE);

        default: return errorToken("Unexpected Character");
    }
}

MIDILang::Token MIDILang::Lexer::keyword() {
    while (isAlpha(peek()) || isDigit(peek())) {
        advance();
    }

    if (currentLiteral() == "else") return makeToken(TK_ELSE);
    else if (currentLiteral() == "if") return makeToken(TK_IF);
    else if (currentLiteral() == "import") return makeToken(TK_IMPORT);
    else if (currentLiteral() == "loop") return makeToken(TK_LOOP);
    else if (currentLiteral() == "note") return makeToken(TK_NOTE);
    else if (currentLiteral() == "play") return makeToken(TK_PLAY);
    else if (currentLiteral() == "tune") return makeToken(TK_TUNE);
    else if (currentLiteral() == "var") return makeToken(TK_VAR);
    else return makeToken(TK_IDENTIFIER);
}

MIDILang::Token MIDILang::Lexer::number() {
    while (isDigit(peek())) {
        advance();
    }

    return makeToken(TK_NUMBER);
}

std::string MIDILang::Lexer::currentLiteral() {
    return this->file.substr(this->start, this->current - this->start);
}

void MIDILang::Lexer::skipWhiteSpace() {
    while (true) {
        char next = peek();

        switch (next) {
            case ' ': case '\r': case '\t':
                advance();
                break;

            case '\n':
                advance();
                this->line++;
                break;

            case '/':
                if (peekNext() == '/') { while (peek() != '\n' && !isAtEnd()) { advance(); } }
                else return;
                break;

            default:
                return;
        }
    }
}

char MIDILang::Lexer::advance() {
    this->current++;
    return this->file[this->current - 1];
}

char MIDILang::Lexer::peek() {
    if (isAtEnd()) return '\0';
    return this->file[this->current];
}

char MIDILang::Lexer::peekNext() {
    if (isAtEnd()) return '\0';
    return this->file[this->current + 1];
}

bool MIDILang::Lexer::isAlpha(char c) {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
}

bool MIDILang::Lexer::isDigit(char c) {
    return c >= '0' && c <= '9';
}

bool MIDILang::Lexer::isAtEnd() {
    return this->current >= this->file.length();
}

MIDILang::Token MIDILang::Lexer::makeToken(MIDILang::TokenType ttype) {
    return Token {
        ttype,
        currentLiteral(),
        this->line,
    };
}

MIDILang::Token MIDILang::Lexer::errorToken(std::string message) {
    this->errors->addError("Lexer Error", message, this->line);

    return Token {
        TokenType::TK_ERROR,
        message,
        this->line,
    };
}
